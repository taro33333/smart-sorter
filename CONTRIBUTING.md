# Contributing to smart-sorter

smart-sorter への貢献をありがとうございます！

## 開発環境のセットアップ

```bash
# リポジトリをクローン
git clone https://github.com/taro33333/smart-sorter.git
cd smart-sorter

# ビルド
cargo build

# テスト
cargo test

# リント
cargo clippy

# フォーマット
cargo fmt
```

## コミットメッセージ規約

このプロジェクトは **Conventional Commits** を採用しています。
リリースノートはコミットメッセージから自動生成されます。

### フォーマット

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### タイプ一覧

| タイプ | 説明 | リリースノートでの表示 |
|-------|------|----------------------|
| `feat` | 新機能 | ✨ Features |
| `fix` | バグ修正 | 🐛 Bug Fixes |
| `docs` | ドキュメントのみの変更 | 📚 Documentation |
| `style` | コードの意味に影響しない変更 | 🎨 Styling |
| `refactor` | バグ修正でも新機能でもないコード変更 | ♻️ Refactor |
| `perf` | パフォーマンス改善 | ⚡ Performance |
| `test` | テストの追加・修正 | 🧪 Testing |
| `chore` | ビルドプロセスやツールの変更 | ⚙️ Miscellaneous Tasks |

### スコープ（オプション）

変更の影響範囲を示します：

- `cli` - CLI関連
- `config` - 設定関連
- `sorter` - 分類ロジック
- `file_ops` - ファイル操作
- `ci` - CI/CD関連
- `deps` - 依存関係

### 例

```bash
# 新機能
feat(cli): add --exclude option to skip specific extensions

# バグ修正
fix(sorter): handle files with no extension correctly

# ドキュメント
docs: update installation instructions in README

# パフォーマンス改善
perf(file_ops): optimize duplicate filename generation

# リファクタリング
refactor(config): simplify extension mapping logic

# テスト追加
test(sorter): add tests for recursive file collection

# 雑務
chore(deps): update clap to v4.5
```

### Breaking Changes（破壊的変更）

破壊的変更がある場合は、フッターに `BREAKING CHANGE:` を追加：

```
feat(cli): change default behavior to dry-run mode

BREAKING CHANGE: The default behavior is now dry-run mode.
Use --execute flag to actually move files.
```

## プルリクエスト

1. フォークしてブランチを作成
2. 変更を加える
3. テストを通す (`cargo test`)
4. リントを通す (`cargo clippy`)
5. フォーマットする (`cargo fmt`)
6. プルリクエストを作成

## 質問・問題報告

- バグ報告: [Issues](https://github.com/taro33333/smart-sorter/issues)
- 質問: [Discussions](https://github.com/taro33333/smart-sorter/discussions)

