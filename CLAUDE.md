# CLAUDE.md - smart-sorter プロジェクトガイド

このファイルはClaude Code（claude.ai/code）がこのリポジトリを理解するためのガイドです。

## プロジェクト概要

**smart-sorter** は、ファイルを拡張子に基づいて自動整理するRust製CLIツールです。

### 主な機能
- 拡張子によるファイルの自動分類（Images, Videos, Documents, Music, Archives, Code, Others）
- Dry Runモード（安全確認）
- 重複ファイル名の自動リネーム（`filename_1.ext` 形式）
- サブディレクトリの再帰処理

## ビルドとテスト

```bash
# ビルド
cargo build

# リリースビルド
cargo build --release

# テスト実行
cargo test

# 特定モジュールのテスト
cargo test --lib config
cargo test --lib file_ops
cargo test --lib sorter

# フォーマット
cargo fmt

# リント
cargo clippy

# 実行（Dry Run）
cargo run -- --dry-run <TARGET_DIR>

# 実行（実際に移動）
cargo run -- <TARGET_DIR>
```

## アーキテクチャ

```
src/
├── main.rs       # エントリーポイント、ロギング初期化
├── cli.rs        # clap deriveによるCLI引数定義
├── config.rs     # 拡張子→カテゴリのマッピング（HashMap + once_cell::Lazy）
├── file_ops.rs   # ファイル操作（移動、重複回避、パス処理）
└── sorter.rs     # 分類コアロジック（Sorter構造体）
```

### モジュール責務

| モジュール | 責務 |
|-----------|------|
| `cli.rs` | CLI引数のパース（`Args`構造体） |
| `config.rs` | 拡張子とカテゴリのマッピング（`Category` enum, `EXTENSION_MAP`） |
| `file_ops.rs` | 低レベルファイル操作（`move_file`, `generate_unique_path`） |
| `sorter.rs` | 高レベル分類ロジック（`Sorter`, `SorterConfig`, `SortStats`） |

## 主要な型

```rust
// カテゴリ列挙型
pub enum Category {
    Images, Videos, Documents, Music, Archives, Code, Others
}

// ソーター設定
pub struct SorterConfig {
    pub target_dir: PathBuf,
    pub dry_run: bool,
    pub recursive: bool,
}

// CLI引数
pub struct Args {
    pub target_dir: PathBuf,
    pub dry_run: bool,      // -d, --dry-run
    pub recursive: bool,    // -r, --recursive
    pub verbose: bool,      // -v, --verbose
}
```

## 依存クレート

| クレート | 用途 |
|---------|------|
| `clap` | CLI引数解析（deriveパターン） |
| `anyhow` | エラーハンドリング |
| `tracing` + `tracing-subscriber` | 構造化ログ |
| `colored` | ターミナル色付き出力 |
| `once_cell` | 遅延初期化（拡張子マップ） |

## コーディング規約

- **エラー処理**: `anyhow::Result` を使用、`.with_context()` でコンテキスト付与
- **ログ**: `tracing` マクロ（`info!`, `debug!`, `warn!`）を使用
- **パス操作**: `std::path::PathBuf` を使用（クロスプラットフォーム対応）
- **テスト**: 各モジュールに `#[cfg(test)]` でユニットテストを配置

## 拡張ポイント

### 新しいカテゴリを追加する場合
1. `config.rs` の `Category` enum に追加
2. `Category::folder_name()` にマッピング追加
3. `Category::all()` に追加
4. `EXTENSION_MAP` に対応する拡張子を追加

### 新しい拡張子を追加する場合
`config.rs` の `EXTENSION_MAP` 初期化部分に追加:
```rust
let new_extensions = ["ext1", "ext2"];
for ext in new_extensions {
    map.insert(ext, Category::TargetCategory);
}
```

## CI/CD

- **CI**: `.github/workflows/ci.yml` - プッシュ/PR時にfmt, clippy, test, build実行
- **Release**: `.github/workflows/release.yml` - タグプッシュ時にマルチプラットフォームバイナリ生成

## 注意事項

- ファイル移動は不可逆操作のため、`dry_run` オプションを推奨
- シンボリックリンクはスキップされる
- カテゴリフォルダ内のファイルは処理対象外（無限ループ防止）

