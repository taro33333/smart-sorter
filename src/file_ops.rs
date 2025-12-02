//! ファイル操作モジュール
//!
//! ファイルの移動、重複ファイル名の生成、ディレクトリ作成などの
//! 低レベルなファイル操作を担当します。

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// ファイル移動の結果を表す構造体
#[derive(Debug, Clone)]
pub struct MoveResult {
    /// 移動元のパス
    #[allow(dead_code)]
    pub source: PathBuf,
    /// 移動先のパス（重複回避後の実際のパス）
    pub destination: PathBuf,
    /// 重複回避のためにリネームされたかどうか
    pub was_renamed: bool,
}

/// 移動先に同名ファイルが存在する場合、連番付きの新しいファイル名を生成する
///
/// # Arguments
/// * `dest_dir` - 移動先ディレクトリ
/// * `filename` - 元のファイル名
///
/// # Returns
/// 重複しないファイルパス
///
/// # Example
/// `report.pdf` → `report_1.pdf` → `report_2.pdf` ...
pub fn generate_unique_path(dest_dir: &Path, filename: &str) -> PathBuf {
    let base_path = dest_dir.join(filename);

    // ファイルが存在しなければそのまま返す
    if !base_path.exists() {
        return base_path;
    }

    // ファイル名を stem と extension に分割
    let path = Path::new(filename);
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(filename);
    let extension = path.extension().and_then(|s| s.to_str());

    // 連番を付けて重複しない名前を探す
    let mut counter = 1u32;
    loop {
        let new_filename = match extension {
            Some(ext) => format!("{}_{}.{}", stem, counter, ext),
            None => format!("{}_{}", stem, counter),
        };

        let new_path = dest_dir.join(&new_filename);
        if !new_path.exists() {
            debug!(
                "Generated unique filename: {} -> {}",
                filename, new_filename
            );
            return new_path;
        }

        counter += 1;

        // 安全のため、上限を設ける（実用上ありえないが念のため）
        if counter > 10000 {
            warn!(
                "Could not generate unique filename after 10000 attempts for: {}",
                filename
            );
            // タイムスタンプを追加してユニーク性を確保
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis())
                .unwrap_or(0);
            let fallback_filename = match extension {
                Some(ext) => format!("{}_{}_{}.{}", stem, counter, timestamp, ext),
                None => format!("{}_{}_{}", stem, counter, timestamp),
            };
            return dest_dir.join(fallback_filename);
        }
    }
}

/// ディレクトリを作成する（既に存在する場合は何もしない）
///
/// # Arguments
/// * `path` - 作成するディレクトリのパス
///
/// # Returns
/// 成功時は `Ok(())`、失敗時はエラー
pub fn ensure_directory(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {}", path.display()))?;
        info!("Created directory: {}", path.display());
    }
    Ok(())
}

/// ファイルを移動する
///
/// `std::fs::rename` を使用してファイルを移動します。
/// 異なるファイルシステム間の移動の場合は、コピー＆削除にフォールバックします。
///
/// # Arguments
/// * `source` - 移動元のファイルパス
/// * `destination` - 移動先のファイルパス
///
/// # Returns
/// 成功時は `Ok(())`、失敗時はエラー
pub fn move_file(source: &Path, destination: &Path) -> Result<()> {
    // まず rename を試行（同一ファイルシステム内なら高速）
    match fs::rename(source, destination) {
        Ok(()) => {
            debug!(
                "Moved file (rename): {} -> {}",
                source.display(),
                destination.display()
            );
            Ok(())
        }
        Err(e) => {
            // rename が失敗した場合（異なるファイルシステム間など）
            // コピー＆削除にフォールバック
            debug!("rename failed ({}), falling back to copy+delete", e);

            fs::copy(source, destination).with_context(|| {
                format!(
                    "Failed to copy file from {} to {}",
                    source.display(),
                    destination.display()
                )
            })?;

            fs::remove_file(source).with_context(|| {
                format!(
                    "Failed to remove original file after copy: {}",
                    source.display()
                )
            })?;

            debug!(
                "Moved file (copy+delete): {} -> {}",
                source.display(),
                destination.display()
            );
            Ok(())
        }
    }
}

/// ファイルを移動する（重複回避付き）
///
/// 移動先に同名ファイルが存在する場合、連番を付けてリネームします。
///
/// # Arguments
/// * `source` - 移動元のファイルパス
/// * `dest_dir` - 移動先ディレクトリ
///
/// # Returns
/// 成功時は `MoveResult`、失敗時はエラー
pub fn move_file_with_dedup(source: &Path, dest_dir: &Path) -> Result<MoveResult> {
    let filename = source
        .file_name()
        .and_then(|n| n.to_str())
        .with_context(|| format!("Invalid filename: {}", source.display()))?;

    // 移動先ディレクトリを作成
    ensure_directory(dest_dir)?;

    // 重複回避した移動先パスを生成
    let original_dest = dest_dir.join(filename);
    let final_dest = generate_unique_path(dest_dir, filename);
    let was_renamed = final_dest != original_dest;

    if was_renamed {
        info!(
            "File renamed to avoid duplicate: {} -> {}",
            filename,
            final_dest.file_name().unwrap_or_default().to_string_lossy()
        );
    }

    // 実際に移動
    move_file(source, &final_dest)?;

    Ok(MoveResult {
        source: source.to_path_buf(),
        destination: final_dest,
        was_renamed,
    })
}

/// パスからファイルの拡張子を取得する（小文字で返す）
///
/// # Arguments
/// * `path` - ファイルパス
///
/// # Returns
/// 拡張子がある場合は `Some(extension)`、ない場合は `None`
pub fn get_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
}

/// ディレクトリかどうかを判定
pub fn is_directory(path: &Path) -> bool {
    path.is_dir()
}

/// ファイルかどうかを判定
pub fn is_file(path: &Path) -> bool {
    path.is_file()
}

/// シンボリックリンクかどうかを判定
pub fn is_symlink(path: &Path) -> bool {
    path.symlink_metadata()
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_generate_unique_path_no_conflict() {
        let dir = tempdir().unwrap();
        let result = generate_unique_path(dir.path(), "test.txt");
        assert_eq!(result, dir.path().join("test.txt"));
    }

    #[test]
    fn test_generate_unique_path_with_conflict() {
        let dir = tempdir().unwrap();

        // 既存ファイルを作成
        File::create(dir.path().join("test.txt")).unwrap();

        let result = generate_unique_path(dir.path(), "test.txt");
        assert_eq!(result, dir.path().join("test_1.txt"));
    }

    #[test]
    fn test_generate_unique_path_multiple_conflicts() {
        let dir = tempdir().unwrap();

        // 複数の既存ファイルを作成
        File::create(dir.path().join("test.txt")).unwrap();
        File::create(dir.path().join("test_1.txt")).unwrap();
        File::create(dir.path().join("test_2.txt")).unwrap();

        let result = generate_unique_path(dir.path(), "test.txt");
        assert_eq!(result, dir.path().join("test_3.txt"));
    }

    #[test]
    fn test_generate_unique_path_no_extension() {
        let dir = tempdir().unwrap();

        // 拡張子なしファイルを作成
        File::create(dir.path().join("README")).unwrap();

        let result = generate_unique_path(dir.path(), "README");
        assert_eq!(result, dir.path().join("README_1"));
    }

    #[test]
    fn test_get_extension() {
        assert_eq!(
            get_extension(Path::new("test.txt")),
            Some("txt".to_string())
        );
        assert_eq!(
            get_extension(Path::new("test.PDF")),
            Some("pdf".to_string())
        );
        assert_eq!(
            get_extension(Path::new("test.tar.gz")),
            Some("gz".to_string())
        );
        assert_eq!(get_extension(Path::new("README")), None);
    }

    #[test]
    fn test_ensure_directory() {
        let dir = tempdir().unwrap();
        let new_dir = dir.path().join("subdir").join("nested");

        assert!(!new_dir.exists());
        ensure_directory(&new_dir).unwrap();
        assert!(new_dir.exists());

        // 既存ディレクトリでも成功すること
        ensure_directory(&new_dir).unwrap();
    }

    #[test]
    fn test_move_file_basic() {
        let dir = tempdir().unwrap();
        let source = dir.path().join("source.txt");
        let dest = dir.path().join("dest.txt");

        // ソースファイルを作成
        fs::write(&source, "test content").unwrap();

        move_file(&source, &dest).unwrap();

        assert!(!source.exists());
        assert!(dest.exists());
        assert_eq!(fs::read_to_string(&dest).unwrap(), "test content");
    }
}
