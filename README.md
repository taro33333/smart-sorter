# smart-sorter

[![CI](https://github.com/taro33333/smart-sorter/actions/workflows/ci.yml/badge.svg)](https://github.com/taro33333/smart-sorter/actions/workflows/ci.yml)
[![Release](https://github.com/taro33333/smart-sorter/actions/workflows/release.yml/badge.svg)](https://github.com/taro33333/smart-sorter/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

📁 ファイルを拡張子に基づいて自動整理するCLIツール

## 概要

`smart-sorter` は、指定されたディレクトリ（例: ダウンロードフォルダ）にある散乱したファイルを、拡張子に基づいて自動的に適切なカテゴリフォルダに移動・整理するツールです。

## 特徴

- 🗂 **自動分類**: ファイルの拡張子を判別し、適切なカテゴリフォルダに振り分け
- 👀 **Dry Run モード**: 実際に移動せず、プレビューのみ表示（安全確認用）
- 📝 **重複回避**: 同名ファイルが存在する場合、`filename_1.ext` のように連番付きで保存
- 📂 **再帰処理**: オプションでサブディレクトリ内も探索可能
- 🚀 **高速**: Rust製で高速に動作

## インストール

### Homebrew（推奨）

macOS / Linux ユーザーは Homebrew でインストールできます：

```bash
brew tap taro33333/tap
brew install smart-sorter
```

### GitHub Releases

[Releases ページ](https://github.com/taro33333/smart-sorter/releases) からバイナリをダウンロード：

| OS | アーキテクチャ | ファイル名 |
|----|--------------|-----------|
| macOS | Apple Silicon (M1/M2) | `smart-sorter-darwin-arm64` |
| macOS | Intel | `smart-sorter-darwin-amd64` |
| Linux | x86_64 | `smart-sorter-linux-amd64` |
| Windows | x86_64 | `smart-sorter-windows-amd64.exe` |

```bash
# 例: macOS Apple Silicon
curl -LO https://github.com/taro33333/smart-sorter/releases/latest/download/smart-sorter-darwin-arm64
chmod +x smart-sorter-darwin-arm64
sudo mv smart-sorter-darwin-arm64 /usr/local/bin/smart-sorter
```

### ソースからビルド

```bash
git clone https://github.com/taro33333/smart-sorter.git
cd smart-sorter
cargo install --path .
```

## クイックスタート

```bash
# 1. まずDry Runでプレビュー（推奨）
smart-sorter --dry-run ~/Downloads

# 2. 問題なければ実行
smart-sorter ~/Downloads
```

## 使用方法

```
smart-sorter [OPTIONS] <TARGET_DIR>

Arguments:
  <TARGET_DIR>  整理対象のディレクトリパス

Options:
  -d, --dry-run    Dry Runモード（実際には移動せず、プレビューのみ表示）
  -r, --recursive  サブディレクトリも再帰的に処理する
  -v, --verbose    詳細なログを出力する
  -h, --help       ヘルプを表示
  -V, --version    バージョンを表示
```

### 使用例

```bash
# Dry Run（プレビュー）
smart-sorter -d ~/Downloads

# サブディレクトリも含めてDry Run
smart-sorter -d -r ~/Downloads

# 実際に移動
smart-sorter ~/Downloads

# サブディレクトリも含めて移動
smart-sorter -r ~/Downloads

# 詳細ログ付き
smart-sorter -v ~/Downloads
```

### 出力例

```
  ╔═══════════════════════════════════════════╗
  ║                                           ║
  ║   📁 smart-sorter                         ║
  ║   File organizer by extension             ║
  ║                                           ║
  ╚═══════════════════════════════════════════╝

Target directory: /Users/user/Downloads
[DRY RUN MODE] No files will be moved.

  [DRY RUN] photo.jpg → Images/photo.jpg [Images]
  [DRY RUN] document.pdf → Documents/document.pdf [Documents]
  [DRY RUN] song.mp3 → Music/song.mp3 [Music]

=== Dry Run Summary ===
Total files found: 3
Files to be moved: 3

Category breakdown:
  Images: 1
  Documents: 1
  Music: 1

✓ Operation completed successfully.
```

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

## 注意事項

> ⚠️ **必ずDry Runで確認してから実行してください**
>
> ファイル移動は取り消しが困難な場合があります。

- カテゴリフォルダ内のファイルはスキップされます
- シンボリックリンクは安全のためスキップされます
- 異なるファイルシステム間の移動もサポート

## 開発

```bash
# テスト
cargo test

# フォーマット
cargo fmt

# リント
cargo clippy
```

## ライセンス

MIT License

## リンク

- [GitHub Repository](https://github.com/taro33333/smart-sorter)
- [Releases](https://github.com/taro33333/smart-sorter/releases)
- [リリース手順](docs/RELEASE.md)
