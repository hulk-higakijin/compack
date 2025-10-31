# compack

汎用コマンド補完システム - zshでコマンドを入力すると、自動的にサブコマンド候補をインタラクティブに表示・選択できるツール

## 概要

compackは、コマンドライン上で各種CLIツールのサブコマンドを素早く選択できるようにする補完システムです。

### 主な特徴

- **自動候補表示**: コマンド入力後、自動的にサブコマンドリストを表示（Tabキー不要）
- **インタラクティブ選択**: 上下キーで候補を選択、Enterで確定
- **設定ファイルベース**: 各コマンドの候補を構造化された設定ファイルで管理
- **拡張性**: 複数のコマンド（rails、cargo、opencode等）を一元管理
- **クロスシェル対応**: 将来的にbash、fishなど他のシェルにも対応可能

## 使用イメージ

```bash
$ opencode [スペース入力]
> acp
  attach
  run
  auth
  agent
  upgrade
  serve
  models
  export
  github
```

上下キーで選択 → Enterで確定 → コマンドラインに自動挿入

## アーキテクチャ

### コンポーネント構成

1. **compack CLI（Rust製）**
   - 設定ファイルの読み込み
   - コマンド候補の取得・提供
   - クエリインターフェース

2. **設定ファイル（TOML）**
   - 各コマンドのサブコマンド候補を手動定義
   - プロジェクト内の`commands/`ディレクトリに配置
   - 各コマンドごとに独立したファイル（例: `opencode.toml`, `cargo.toml`）

3. **zsh統合スクリプト**
   - コマンド入力の検知
   - fzfによるリスト表示・選択
   - 選択結果のコマンドライン挿入

### データフロー

```
ユーザー入力 → zsh検知 → compack query → 候補取得 → fzf表示 → 選択 → コマンドライン挿入
```

## 設定ファイル形式

各コマンドは`commands/`ディレクトリ内に独立したTOMLファイルとして配置されます。

### commands/opencode.toml
```toml
subcommands = [
  "acp",
  "attach",
  "run",
  "auth",
  "agent",
  "upgrade",
  "serve",
  "models",
  "export",
  "github"
]
```

### commands/cargo.toml
```toml
subcommands = [
  "build",
  "run",
  "test",
  "check",
  "clean",
  "doc"
]
```

### commands/rails.toml
```toml
subcommands = [
  "new",
  "server",
  "console",
  "generate",
  "db:migrate",
  "routes"
]
```

**新しいコマンドの追加方法**: `commands/`ディレクトリに新しいTOMLファイルを作成するだけで自動的に認識されます。

## CLI インターフェース

### サブコマンド候補の取得

```bash
$ compack query opencode
acp
attach
run
auth
agent
upgrade
serve
models
export
github
```

### zsh統合スクリプトの生成

```bash
$ compack init zsh
# compack zsh integration
_compack_completion() {
    ...
}
...
```

このコマンドで生成されたスクリプトを`.zshrc`に追加します：

```bash
eval "$(compack init zsh)"
```

## インストール

```bash
# リポジトリのクローン
git clone <repository-url>
cd compack

# ビルド
cargo build --release

# インストール
cargo install --path .

# zsh統合設定（.zshrcに追加）
echo 'eval "$(compack init zsh)"' >> ~/.zshrc

# シェルを再起動または設定を再読み込み
source ~/.zshrc
```

**注意**: 設定ファイルはプロジェクト内の`commands/`ディレクトリに配置されています。
新しいコマンドを追加する場合は、`commands/`ディレクトリに新しいTOMLファイルを作成してください。

## 依存関係

- **Rust**: 1.70+
- **fzf**: インタラクティブ選択に使用
- **zsh**: シェル統合（将来的に他シェル対応予定）

## 将来の拡張性

- [ ] 動的サブコマンド取得（`--help`パース）
- [ ] サブコマンドの説明文表示
- [ ] bash、fish対応
- [ ] ネストされたサブコマンド対応（例: `git branch --list`）
- [ ] 補完候補のキャッシング
- [ ] コミュニティ共有設定リポジトリ

## 技術スタック

- **言語**: Rust
- **CLIフレームワーク**: clap
- **設定ファイル**: TOML（serdeでパース）
- **シェル統合**: zsh widget
- **UI**: fzf（fuzzy finder）

## ライセンス

MIT

## 開発状況

現在、初期実装フェーズ。最初のターゲットは`opencode`コマンドの補完機能実装。
