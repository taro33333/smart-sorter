# smart-sorter

📁 ファイルを拡張子に基づいて自動整理するCLIツール

## 概要

`smart-sorter` は、指定されたディレクトリ（例: ダウンロードフォルダ）にある散乱したファイルを、拡張子に基づいて自動的に適切なカテゴリフォルダに移動・整理するツールです。

## 特徴

- **🗂 自動分類**: ファイルの拡張子を判別し、適切なカテゴリフォルダに振り分け
- **👀 Dry Run モード**: 実際に移動せず、プレビューのみ表示（安全確認用）
- **📝 重複回避**: 同名ファイルが存在する場合、`filename_1.ext` のように連番付きで保存
- **📂 再帰処理**: オプションでサブディレクトリ内も探索可能

## カテゴリ分類

| カテゴリ | 拡張子例 |
|----------|----------|
| **Images** | jpg, jpeg, png, gif, bmp, svg, webp, heic, raw, psd |
| **Videos** | mp4, avi, mkv, mov, wmv, flv, webm, m4v, mpeg |
| **Documents** | pdf, doc, docx, xls, xlsx, ppt, pptx, txt, rtf, csv |
| **Music** | mp3, wav, flac, aac, ogg, wma, m4a, aiff |
| **Archives** | zip, rar, 7z, tar, gz, bz2, xz, dmg, iso |
| **Code** | rs, py, js, ts, html, css, json, yaml, toml, md, sh |
| **Others** | 上記以外の拡張子、または拡張子なし |

## インストール

### ビルド方法

```bash
# リポジトリをクローン
git clone <repository-url>
cd smart-sorter

# リリースビルド
cargo build --release

# バイナリは target/release/smart-sorter に生成されます
```

### インストール（オプション）

```bash
# cargo install でシステムにインストール
cargo install --path .

# または直接バイナリをPATHの通った場所にコピー
sudo cp target/release/smart-sorter /usr/local/bin/
```

## 使用方法

### ヘルプの表示

```bash
smart-sorter --help
```

出力例:

```
smart-sorter: ファイルを拡張子に基づいて自動整理するCLIツール

Usage: smart-sorter [OPTIONS] <TARGET_DIR>

Arguments:
  <TARGET_DIR>  整理対象のディレクトリパス

Options:
  -d, --dry-run    Dry Runモード（実際には移動せず、プレビューのみ表示）
  -r, --recursive  サブディレクトリも再帰的に処理する
  -v, --verbose    詳細なログを出力する
  -h, --help       Print help
  -V, --version    Print version
```

### 基本的な使い方

#### 1. Dry Run（プレビュー）で確認

**まずはDry Runで実行することを強く推奨します。**

```bash
# ダウンロードフォルダをDry Runで確認
smart-sorter --dry-run ~/Downloads
```

出力例:

```
  ╔═══════════════════════════════════════════╗
  ║                                           ║
  ║   📁 smart-sorter                         ║
  ║   File organizer by extension             ║
  ║                                           ║
  ╚═══════════════════════════════════════════╝

Target directory: /Users/user/Downloads
[DRY RUN MODE] No files will be moved.

  [DRY RUN] photo1.jpg → Images/photo1.jpg [Images]
  [DRY RUN] document.pdf → Documents/document.pdf [Documents]
  [DRY RUN] song.mp3 → Music/song.mp3 [Music]
  [DRY RUN] archive.zip → Archives/archive.zip [Archives]
  [DRY RUN] report.pdf → Documents/report.pdf [Documents]
  [DRY RUN] report.pdf → Documents/report_1.pdf (renamed)

=== Dry Run Summary ===
Total files found: 6
Files to be moved: 6

Category breakdown:
  Images: 1
  Documents: 3
  Music: 1
  Archives: 1
```

#### 2. 実際にファイルを移動

Dry Runで確認後、問題なければ実際に移動を実行:

```bash
# 実際にファイルを移動
smart-sorter ~/Downloads
```

#### 3. サブディレクトリも含めて処理

```bash
# 再帰的に処理（サブディレクトリ内も対象）
smart-sorter --recursive ~/Downloads

# Dry Runと組み合わせ
smart-sorter --dry-run --recursive ~/Downloads
```

#### 4. 詳細ログ出力

```bash
# デバッグ情報を表示
smart-sorter --verbose --dry-run ~/Downloads
```

### オプション一覧

| オプション | 短縮形 | 説明 |
|-----------|--------|------|
| `--dry-run` | `-d` | 実際には移動せず、プレビューのみ表示 |
| `--recursive` | `-r` | サブディレクトリも再帰的に処理 |
| `--verbose` | `-v` | 詳細なログを出力 |
| `--help` | `-h` | ヘルプを表示 |
| `--version` | `-V` | バージョンを表示 |

### 使用例

```bash
# 基本的なDry Run
smart-sorter -d ~/Downloads

# サブディレクトリも含めてDry Run
smart-sorter -d -r ~/Downloads

# 実際に移動（ルートディレクトリのみ）
smart-sorter ~/Downloads

# 実際に移動（サブディレクトリも含む）
smart-sorter -r ~/Downloads

# 詳細ログ付きで実行
smart-sorter -v ~/Downloads
```

## 注意事項

- **必ずDry Runで確認してから実行してください**: ファイル移動は取り消しが困難な場合があります。
- **カテゴリフォルダ内のファイルはスキップされます**: 既に `Images/` などのカテゴリフォルダ内にあるファイルは処理対象外です。
- **シンボリックリンクはスキップされます**: 安全のため、シンボリックリンクは移動対象外です。
- **異なるファイルシステム間の移動もサポート**: `rename` が失敗した場合、自動的にコピー＆削除にフォールバックします。

## 開発

### テストの実行

```bash
# 全テストを実行
cargo test

# 特定のモジュールのテストを実行
cargo test --lib config
cargo test --lib file_ops
cargo test --lib sorter
```

### リントとフォーマット

```bash
# フォーマット
cargo fmt

# Clippy（リント）
cargo clippy
```

## インストール（Homebrew）

macOS / Linux ユーザーは Homebrew でインストールできます：

```bash
# Tap を追加してインストール
brew tap taro33333/tap
brew install smart-sorter

# または直接インストール
brew install taro33333/tap/smart-sorter
```

## ライセンス

MIT License

## 貢献

プルリクエストやイシューの報告を歓迎します。

## ドキュメント

- [リリース手順](docs/RELEASE.md) - 新しいバージョンのリリース方法
