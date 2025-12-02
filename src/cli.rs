//! CLI引数の定義モジュール
//!
//! clapのderiveパターンを使用して、型安全なCLIインターフェースを定義します。

use clap::Parser;
use std::path::PathBuf;

/// smart-sorter: ファイルを拡張子に基づいて自動整理するCLIツール
///
/// 指定されたディレクトリ内のファイルを、拡張子に基づいて
/// Images, Videos, Documents, Music, Archives, Code, Others などの
/// カテゴリフォルダに自動的に振り分けます。
#[derive(Parser, Debug)]
#[command(
    name = "smart-sorter",
    version,
    author,
    about = "ファイルを拡張子に基づいて自動整理するCLIツール",
    long_about = "指定されたディレクトリ内のファイルを、拡張子に基づいて\n\
                  Images, Videos, Documents, Music, Archives, Code, Others などの\n\
                  カテゴリフォルダに自動的に振り分けます。\n\n\
                  安全のため、--dry-run オプションで事前確認することを推奨します。"
)]
pub struct Args {
    /// 整理対象のディレクトリパス
    #[arg(value_name = "TARGET_DIR", help = "整理対象のディレクトリパス")]
    pub target_dir: PathBuf,

    /// Dry Runモード（実際には移動せず、プレビューのみ表示）
    #[arg(
        short = 'd',
        long = "dry-run",
        help = "Dry Runモード（実際には移動せず、プレビューのみ表示）"
    )]
    pub dry_run: bool,

    /// サブディレクトリも再帰的に処理する
    #[arg(
        short = 'r',
        long = "recursive",
        help = "サブディレクトリも再帰的に処理する"
    )]
    pub recursive: bool,

    /// 詳細なログを出力する
    #[arg(short = 'v', long = "verbose", help = "詳細なログを出力する")]
    pub verbose: bool,
}

impl Args {
    /// コマンドライン引数をパースしてArgs構造体を返す
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_default_values() {
        // デフォルト値のテスト
        let args = Args::try_parse_from(["smart-sorter", "/tmp/test"]).unwrap();
        assert_eq!(args.target_dir, PathBuf::from("/tmp/test"));
        assert!(!args.dry_run);
        assert!(!args.recursive);
        assert!(!args.verbose);
    }

    #[test]
    fn test_args_with_flags() {
        let args = Args::try_parse_from([
            "smart-sorter",
            "-d",
            "-r",
            "-v",
            "/home/user/Downloads",
        ])
        .unwrap();
        assert!(args.dry_run);
        assert!(args.recursive);
        assert!(args.verbose);
    }
}

