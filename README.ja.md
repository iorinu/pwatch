<div align="center">

![logo](assets/logo.png)

ポート使用状況の可視化とプロセスキルを行うCLI/TUIツール。

[English README](README.md)

<!-- demo.gif を assets/ に配置してください -->
![demo](assets/demo.gif)

</div>

「ポートが既に使われている」エラー発生時に、ポート確認 → プロセス特定 → kill を1コマンドで完結させます。

## 特徴

- 指定ポートを使っているプロセスを瞬時に特定
- 1コマンドで複数ポートを一括キル
- ライブ更新する `--watch` モードと auto-refresh 対応の TUI
- 英語 / 日本語 UI を実行時に切替可能
- bash / zsh / fish / powershell / elvish 向けのシェル補完を生成
- ランタイム依存ゼロの単一バイナリ

## インストール

### シェルインストーラ (macOS / Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/iorinu/pwatch/releases/latest/download/pwatch-installer.sh | sh
```

### Homebrew (macOS / Linux)

```bash
brew install iorinu/tap/pwatch
```

### Cargo (crates.io から)

```bash
cargo install pwatch
```

### ソースから

```bash
git clone https://github.com/iorinu/pwatch
cd pwatch
cargo install --path .
```

各リリースの事前ビルド済みバイナリは
[Releases ページ](https://github.com/iorinu/pwatch/releases) からもダウンロードできます。

## クイックスタート

```bash
pwatch list           # 何がリッスンしてるか確認
pwatch kill 3000      # 詰まったポートを解放
pwatch kill 3000 5173 8080   # 複数まとめてキル
pwatch ui             # 対話型ビューワ + キラー (TUI)
```

## 使い方

### 全リスニングポートを表示

```bash
pwatch list
```

JSON形式で出力:

```bash
pwatch list --json
```

`top` のように継続表示:

```bash
pwatch list --watch                # 2秒ごとに更新
pwatch list --watch --interval 5   # 間隔を指定
```

### 特定ポートの使用状況を確認

```bash
pwatch check 8080
```

### ポートを使用しているプロセスをキル

```bash
pwatch kill 8080                     # SIGTERM
pwatch kill 8080 --force             # SIGKILL
pwatch kill 8080 3000 5173           # 複数ポート一括キル
```

権限エラーが出る場合:

```bash
sudo pwatch kill 8080
```

### TUIモード

```bash
pwatch ui
```

| キー | 操作 |
|------|------|
| `j` / `↓` | 選択を下に移動 |
| `k` / `↑` | 選択を上に移動 |
| `d` | SIGTERM でキル (確認あり) |
| `D` | SIGKILL でキル (確認あり) |
| `/` | 検索モード |
| `r` | 手動リフレッシュ |
| `a` | 自動更新のトグル |
| `+` / `-` | 自動更新間隔を調整 (±0.5秒) |
| `q` / `Esc` | 終了 |

### 設定

起動時のバナーを非表示にする:

```bash
pwatch config banner off
```

再表示する:

```bash
pwatch config banner on
```

表示言語を切り替える (runtime メッセージと TUI が対象)。デフォルトは英語:

```bash
pwatch config lang ja   # 日本語
pwatch config lang en   # 英語 (デフォルト)
```

> 注意: `--help` の出力は常に英語です。CLI 出力や TUI ラベルなど runtime メッセージのみ切り替わります。

設定は `~/.config/pwatch/config.toml` に保存されます。

### シェル補完

利用中のシェル用の補完スクリプトを生成できます:

```bash
pwatch completion bash > /usr/local/etc/bash_completion.d/pwatch
pwatch completion zsh  > ~/.zsh/completion/_pwatch     # $fpath に含めること
pwatch completion fish > ~/.config/fish/completions/pwatch.fish
```

対応シェル: `bash`, `zsh`, `fish`, `powershell`, `elvish`。

## 対応プラットフォーム

| OS | スキャン方法 |
|----|-------------|
| Linux | `/proc/net/tcp` 直接パース |
| macOS | `lsof` コマンド経由 |

## ビルド

```bash
cargo build --release
```

## ライセンス

[MIT License](LICENSE) の下で公開しています。

Copyright (c) 2026 iorinu
