//! ソーターモジュール
//!
//! ファイル分類のコアロジックを担当します。
//! ディレクトリの走査、ファイルの分類、移動処理を統括します。

use crate::config::{get_category, get_default_category, Category};
use crate::file_ops::{
    ensure_directory, generate_unique_path, get_extension, is_directory, is_file, is_symlink,
    move_file_with_dedup,
};
use anyhow::{Context, Result};
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// ソーターの設定
#[derive(Debug, Clone)]
pub struct SorterConfig {
    /// 対象ディレクトリ
    pub target_dir: PathBuf,
    /// Dry Runモード
    pub dry_run: bool,
    /// 再帰処理
    pub recursive: bool,
}

/// ファイル分類の計画（移動前の状態）
#[derive(Debug, Clone)]
pub struct FilePlan {
    /// 移動元のパス
    pub source: PathBuf,
    /// 移動先のパス（重複回避前の予定パス）
    pub destination: PathBuf,
    /// 分類されるカテゴリ
    pub category: Category,
    /// 移動先に重複ファイルが存在するか
    pub has_conflict: bool,
}

/// 分類処理の統計情報
#[derive(Debug, Default)]
pub struct SortStats {
    /// 処理対象ファイル数
    pub total_files: usize,
    /// 移動成功数
    pub moved_files: usize,
    /// リネームされたファイル数
    pub renamed_files: usize,
    /// スキップされたファイル数
    pub skipped_files: usize,
    /// エラー数
    pub error_count: usize,
    /// カテゴリごとのファイル数
    pub category_counts: HashMap<Category, usize>,
}

impl SortStats {
    /// 統計情報のサマリーを表示
    pub fn print_summary(&self, dry_run: bool) {
        println!();
        if dry_run {
            println!("{}", "=== Dry Run Summary ===".cyan().bold());
        } else {
            println!("{}", "=== Summary ===".green().bold());
        }
        println!(
            "Total files found: {}",
            self.total_files.to_string().yellow()
        );

        if dry_run {
            println!("Files to be moved: {}", self.moved_files.to_string().cyan());
        } else {
            println!("Files moved: {}", self.moved_files.to_string().green());
            if self.renamed_files > 0 {
                println!(
                    "Files renamed (due to conflicts): {}",
                    self.renamed_files.to_string().yellow()
                );
            }
        }

        if self.skipped_files > 0 {
            println!("Files skipped: {}", self.skipped_files.to_string().yellow());
        }

        if self.error_count > 0 {
            println!("Errors: {}", self.error_count.to_string().red());
        }

        println!();
        println!("{}", "Category breakdown:".bold());
        for category in Category::all() {
            if let Some(&count) = self.category_counts.get(category) {
                if count > 0 {
                    println!("  {}: {}", category.folder_name(), count);
                }
            }
        }
    }
}

/// ファイルソーター
pub struct Sorter {
    config: SorterConfig,
}

impl Sorter {
    /// 新しいソーターインスタンスを作成
    pub fn new(config: SorterConfig) -> Self {
        Self { config }
    }

    /// メインの実行関数
    pub fn run(&self) -> Result<SortStats> {
        // 対象ディレクトリの存在確認
        if !self.config.target_dir.exists() {
            anyhow::bail!(
                "Target directory does not exist: {}",
                self.config.target_dir.display()
            );
        }

        if !self.config.target_dir.is_dir() {
            anyhow::bail!(
                "Target path is not a directory: {}",
                self.config.target_dir.display()
            );
        }

        // 読み取り権限の確認
        fs::read_dir(&self.config.target_dir).with_context(|| {
            format!(
                "Cannot read directory: {}",
                self.config.target_dir.display()
            )
        })?;

        println!(
            "{} {}",
            "Target directory:".bold(),
            self.config.target_dir.display()
        );

        if self.config.dry_run {
            println!("{}", "[DRY RUN MODE] No files will be moved.".cyan().bold());
        }

        if self.config.recursive {
            println!("{}", "[RECURSIVE MODE] Processing subdirectories.".yellow());
        }

        println!();

        // ファイルを収集
        let files = self.collect_files(&self.config.target_dir)?;
        info!("Found {} files to process", files.len());

        if files.is_empty() {
            println!("{}", "No files found to sort.".yellow());
            return Ok(SortStats::default());
        }

        // 分類計画を作成
        let plans = self.create_plans(&files)?;

        // 実行（Dry Run または 実際の移動）
        let stats = if self.config.dry_run {
            self.execute_dry_run(&plans)?
        } else {
            self.execute_move(&plans)?
        };

        stats.print_summary(self.config.dry_run);

        Ok(stats)
    }

    /// ファイルを収集
    fn collect_files(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in fs::read_dir(dir)
            .with_context(|| format!("Failed to read directory: {}", dir.display()))?
        {
            let entry = entry.with_context(|| "Failed to read directory entry")?;
            let path = entry.path();

            // シンボリックリンクはスキップ
            if is_symlink(&path) {
                debug!("Skipping symlink: {}", path.display());
                continue;
            }

            if is_file(&path) {
                // カテゴリフォルダ内のファイルはスキップ（無限ループ防止）
                if self.is_category_folder(&path) {
                    debug!("Skipping file in category folder: {}", path.display());
                    continue;
                }
                files.push(path);
            } else if is_directory(&path) && self.config.recursive {
                // カテゴリフォルダは再帰処理しない
                let folder_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if Category::all()
                    .iter()
                    .any(|c| c.folder_name() == folder_name)
                {
                    debug!("Skipping category folder: {}", path.display());
                    continue;
                }

                // 再帰的にファイルを収集
                let sub_files = self.collect_files(&path)?;
                files.extend(sub_files);
            }
        }

        Ok(files)
    }

    /// パスがカテゴリフォルダ内にあるかチェック
    fn is_category_folder(&self, path: &Path) -> bool {
        if let Some(parent) = path.parent() {
            if let Some(folder_name) = parent.file_name().and_then(|n| n.to_str()) {
                if parent.parent() == Some(&self.config.target_dir) {
                    return Category::all()
                        .iter()
                        .any(|c| c.folder_name() == folder_name);
                }
            }
        }
        false
    }

    /// 分類計画を作成
    fn create_plans(&self, files: &[PathBuf]) -> Result<Vec<FilePlan>> {
        let mut plans = Vec::new();

        for file in files {
            let category = self.categorize_file(file);
            let dest_dir = self.config.target_dir.join(category.folder_name());
            let filename = file
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            let destination = dest_dir.join(filename);
            let has_conflict = destination.exists();

            plans.push(FilePlan {
                source: file.clone(),
                destination,
                category,
                has_conflict,
            });
        }

        Ok(plans)
    }

    /// ファイルをカテゴリ分類
    fn categorize_file(&self, path: &Path) -> Category {
        match get_extension(path) {
            Some(ext) => get_category(&ext),
            None => get_default_category(),
        }
    }

    /// Dry Run実行
    fn execute_dry_run(&self, plans: &[FilePlan]) -> Result<SortStats> {
        let mut stats = SortStats::default();
        stats.total_files = plans.len();

        for plan in plans {
            // カテゴリカウントを更新
            *stats.category_counts.entry(plan.category).or_insert(0) += 1;

            // 相対パスを計算（表示用）
            let relative_source = plan
                .source
                .strip_prefix(&self.config.target_dir)
                .unwrap_or(&plan.source);

            let dest_dir = self.config.target_dir.join(plan.category.folder_name());

            // 重複がある場合の移動先ファイル名を計算
            let filename = plan
                .source
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            let final_dest = if plan.has_conflict {
                generate_unique_path(&dest_dir, filename)
            } else {
                dest_dir.join(filename)
            };

            let relative_dest = final_dest
                .strip_prefix(&self.config.target_dir)
                .unwrap_or(&final_dest);

            // 表示
            let arrow = "→".cyan();
            let category_colored = format!("[{}]", plan.category).blue();

            if plan.has_conflict {
                println!(
                    "  {} {} {} {} {}",
                    "[DRY RUN]".cyan(),
                    relative_source.display(),
                    arrow,
                    relative_dest.display(),
                    "(renamed)".yellow()
                );
                stats.renamed_files += 1;
            } else {
                println!(
                    "  {} {} {} {} {}",
                    "[DRY RUN]".cyan(),
                    relative_source.display(),
                    arrow,
                    relative_dest.display(),
                    category_colored
                );
            }

            stats.moved_files += 1;
        }

        Ok(stats)
    }

    /// 実際のファイル移動を実行
    fn execute_move(&self, plans: &[FilePlan]) -> Result<SortStats> {
        let mut stats = SortStats::default();
        stats.total_files = plans.len();

        // カテゴリフォルダを事前に作成
        for category in Category::all() {
            let dir = self.config.target_dir.join(category.folder_name());
            // 必要に応じて作成（ファイルがある場合のみ）
            if plans.iter().any(|p| p.category == *category) {
                ensure_directory(&dir)?;
            }
        }

        for plan in plans {
            let dest_dir = self.config.target_dir.join(plan.category.folder_name());

            match move_file_with_dedup(&plan.source, &dest_dir) {
                Ok(result) => {
                    // カテゴリカウントを更新
                    *stats.category_counts.entry(plan.category).or_insert(0) += 1;

                    // 相対パスを計算（表示用）
                    let relative_source = plan
                        .source
                        .strip_prefix(&self.config.target_dir)
                        .unwrap_or(&plan.source);
                    let relative_dest = result
                        .destination
                        .strip_prefix(&self.config.target_dir)
                        .unwrap_or(&result.destination);

                    let arrow = "→".green();

                    if result.was_renamed {
                        println!(
                            "  {} {} {} {}",
                            "✓".green(),
                            relative_source.display(),
                            arrow,
                            format!("{} (renamed)", relative_dest.display()).yellow()
                        );
                        stats.renamed_files += 1;
                    } else {
                        println!(
                            "  {} {} {} {}",
                            "✓".green(),
                            relative_source.display(),
                            arrow,
                            relative_dest.display()
                        );
                    }

                    stats.moved_files += 1;
                }
                Err(e) => {
                    warn!("Failed to move file: {}", e);
                    println!(
                        "  {} {} - {}",
                        "✗".red(),
                        plan.source.display(),
                        e.to_string().red()
                    );
                    stats.error_count += 1;
                }
            }
        }

        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_categorize_file() {
        let dir = tempdir().unwrap();
        let config = SorterConfig {
            target_dir: dir.path().to_path_buf(),
            dry_run: true,
            recursive: false,
        };
        let sorter = Sorter::new(config);

        assert_eq!(
            sorter.categorize_file(Path::new("test.jpg")),
            Category::Images
        );
        assert_eq!(
            sorter.categorize_file(Path::new("test.mp4")),
            Category::Videos
        );
        assert_eq!(
            sorter.categorize_file(Path::new("test.pdf")),
            Category::Documents
        );
        assert_eq!(
            sorter.categorize_file(Path::new("test.mp3")),
            Category::Music
        );
        assert_eq!(
            sorter.categorize_file(Path::new("test.zip")),
            Category::Archives
        );
        assert_eq!(sorter.categorize_file(Path::new("test.rs")), Category::Code);
        assert_eq!(
            sorter.categorize_file(Path::new("test.xyz")),
            Category::Others
        );
    }

    #[test]
    fn test_collect_files_non_recursive() {
        let dir = tempdir().unwrap();

        // ルートにファイルを作成
        File::create(dir.path().join("file1.txt")).unwrap();
        File::create(dir.path().join("file2.jpg")).unwrap();

        // サブディレクトリを作成
        fs::create_dir(dir.path().join("subdir")).unwrap();
        File::create(dir.path().join("subdir").join("file3.txt")).unwrap();

        let config = SorterConfig {
            target_dir: dir.path().to_path_buf(),
            dry_run: true,
            recursive: false,
        };
        let sorter = Sorter::new(config);

        let files = sorter.collect_files(dir.path()).unwrap();
        assert_eq!(files.len(), 2); // サブディレクトリ内は含まれない
    }

    #[test]
    fn test_collect_files_recursive() {
        let dir = tempdir().unwrap();

        // ルートにファイルを作成
        File::create(dir.path().join("file1.txt")).unwrap();

        // サブディレクトリを作成
        fs::create_dir(dir.path().join("subdir")).unwrap();
        File::create(dir.path().join("subdir").join("file2.txt")).unwrap();

        let config = SorterConfig {
            target_dir: dir.path().to_path_buf(),
            dry_run: true,
            recursive: true,
        };
        let sorter = Sorter::new(config);

        let files = sorter.collect_files(dir.path()).unwrap();
        assert_eq!(files.len(), 2); // サブディレクトリ内も含まれる
    }

    #[test]
    fn test_create_plans() {
        let dir = tempdir().unwrap();

        File::create(dir.path().join("photo.jpg")).unwrap();
        File::create(dir.path().join("document.pdf")).unwrap();

        let config = SorterConfig {
            target_dir: dir.path().to_path_buf(),
            dry_run: true,
            recursive: false,
        };
        let sorter = Sorter::new(config);

        let files = sorter.collect_files(dir.path()).unwrap();
        let plans = sorter.create_plans(&files).unwrap();

        assert_eq!(plans.len(), 2);

        // 各ファイルが正しいカテゴリに分類されているか
        for plan in &plans {
            let filename = plan.source.file_name().unwrap().to_str().unwrap();
            match filename {
                "photo.jpg" => assert_eq!(plan.category, Category::Images),
                "document.pdf" => assert_eq!(plan.category, Category::Documents),
                _ => panic!("Unexpected file: {}", filename),
            }
        }
    }
}
