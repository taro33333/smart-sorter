//! smart-sorter
//!
//! ファイルを拡張子に基づいて自動整理するCLIツール
//!
//! # 機能
//! - 自動分類: ファイルの拡張子を判別し、カテゴリフォルダに振り分け
//! - Dry Run: 実際に移動せずプレビュー表示
//! - 重複回避: 同名ファイルは連番付きでリネーム
//! - 再帰処理: サブディレクトリ内も探索可能

mod cli;
mod config;
mod file_ops;
mod sorter;

use anyhow::Result;
use cli::Args;
use colored::Colorize;
use sorter::{Sorter, SorterConfig};
use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    // CLI引数をパース
    let args = Args::parse_args();

    // ロギングを初期化
    init_logging(args.verbose);

    // バナー表示
    print_banner();

    // ソーター設定を作成
    let config = SorterConfig {
        target_dir: args.target_dir,
        dry_run: args.dry_run,
        recursive: args.recursive,
    };

    // 実行前の確認（実際の移動時のみ）
    if !config.dry_run {
        print_warning();
    }

    // ソーターを実行
    let sorter = Sorter::new(config);
    match sorter.run() {
        Ok(_stats) => {
            println!();
            println!("{}", "✓ Operation completed successfully.".green().bold());
            Ok(())
        }
        Err(e) => {
            eprintln!();
            eprintln!("{} {}", "✗ Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
}

/// ロギングを初期化
fn init_logging(verbose: bool) {
    let level = if verbose { Level::DEBUG } else { Level::INFO };

    let filter = EnvFilter::from_default_env()
        .add_directive(level.into())
        .add_directive("hyper=warn".parse().unwrap())
        .add_directive("reqwest=warn".parse().unwrap());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::NONE)
        .with_target(false)
        .without_time()
        .init();
}

/// バナーを表示
fn print_banner() {
    println!();
    println!(
        "{}",
        r#"
  ╔═══════════════════════════════════════════╗
  ║                                           ║
  ║   📁 smart-sorter                         ║
  ║   File organizer by extension             ║
  ║                                           ║
  ╚═══════════════════════════════════════════╝
"#
        .cyan()
    );
}

/// 警告を表示（実際の移動実行時）
fn print_warning() {
    println!(
        "{}",
        "⚠️  WARNING: This will move files. Use --dry-run first to preview."
            .yellow()
            .bold()
    );
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banner_does_not_panic() {
        // バナー表示がパニックしないことを確認
        print_banner();
    }
}

