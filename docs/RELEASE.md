# リリース手順

このドキュメントでは、smart-sorter の新しいバージョンをリリースする手順を説明します。

## コミットメッセージ規約

リリースノートは **Conventional Commits** から自動生成されます。
詳細は [CONTRIBUTING.md](../CONTRIBUTING.md) を参照してください。

```bash
# 例
feat(cli): add new option
fix(sorter): fix bug in file handling
docs: update README
```

## 前提条件

### 1. GitHub Secret の設定（初回のみ）

Homebrew Tap への自動プッシュには Personal Access Token (PAT) が必要です。

1. **PAT を作成**
   - <https://github.com/settings/tokens?type=beta> へアクセス
   - 「Generate new token (Beta)」をクリック
   - **Token name**: `homebrew-tap-access`
   - **Expiration**: 適切な期限を設定
   - **Repository access**: `Only select repositories` → `taro33333/homebrew-tap`
   - **Permissions**:
     - `Contents`: `Read and write`
   - 「Generate token」をクリックしてトークンをコピー

2. **Secret を追加**
   - <https://github.com/taro33333/smart-sorter/settings/secrets/actions> へアクセス
   - 「New repository secret」をクリック
   - **Name**: `HOMEBREW_TAP_TOKEN`
   - **Value**: コピーしたPATを貼り付け
   - 「Add secret」をクリック

---

## リリースフロー

### 1. バージョンを更新

`Cargo.toml` のバージョンを更新：

```toml
[package]
name = "smart-sorter"
version = "0.2.0"  # ← 新しいバージョン
```

### 2. 変更をコミット

```bash
git add Cargo.toml Cargo.lock
git commit -m "Bump version to 0.2.0"
git push origin main
```

### 3. タグを作成してプッシュ

```bash
git tag v0.2.0
git push origin v0.2.0
```

### 4. 自動実行される処理

タグプッシュ後、GitHub Actions が自動で以下を実行します：

1. **ビルド** (約5-10分)
   - Linux (x86_64)
   - macOS Intel (x86_64)
   - macOS Apple Silicon (aarch64)
   - Windows (x86_64)

2. **GitHub Release 作成**
   - バイナリをアップロード
   - checksums.txt を生成

3. **Homebrew Formula 更新**
   - `taro33333/homebrew-tap` に自動プッシュ

### 5. リリースの確認

- **GitHub Release**: <https://github.com/taro33333/smart-sorter/releases>
- **Homebrew Formula**: <https://github.com/taro33333/homebrew-tap/blob/main/smart-sorter.rb>

---

## バージョニング規則

[Semantic Versioning](https://semver.org/) に従います：

| 変更内容 | バージョン例 |
|---------|-------------|
| 後方互換性のあるバグ修正 | 0.1.0 → 0.1.1 |
| 後方互換性のある機能追加 | 0.1.0 → 0.2.0 |
| 後方互換性のない変更 | 0.1.0 → 1.0.0 |

---

## トラブルシューティング

### リリースが失敗した場合

1. **GitHub Actions のログを確認**
   - <https://github.com/taro33333/smart-sorter/actions>

2. **タグを削除して再作成**

   ```bash
   # ローカルのタグを削除
   git tag -d v0.2.0

   # リモートのタグを削除
   git push origin :refs/tags/v0.2.0

   # 修正後、再度タグを作成
   git tag v0.2.0
   git push origin v0.2.0
   ```

### Homebrew Formula の更新が失敗した場合

- `HOMEBREW_TAP_TOKEN` が有効か確認
- PAT の権限が正しいか確認
- PAT が期限切れでないか確認

---

## ユーザー向けインストール方法

リリース後、ユーザーは以下でインストールできます：

```bash
# Homebrew (macOS/Linux)
brew tap taro33333/tap
brew install smart-sorter

# または直接インストール
brew install taro33333/tap/smart-sorter

# アップグレード
brew upgrade smart-sorter
```

---

## 関連リンク

- [GitHub Releases](https://github.com/taro33333/smart-sorter/releases)
- [GitHub Actions ワークフロー](https://github.com/taro33333/smart-sorter/actions)
