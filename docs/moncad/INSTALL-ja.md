# MonCad 2026 — インストール ガイド

MonCad は、AutoCAD に似たコマンドで操作できる無料・オープンソースの 2D CAD です。
日本語の画面で使えます。手で作図することも、Claude や ChatGPT に作図を任せることもできます。
すべての機能が無料で、有料版や機能制限はありません。

ダウンロード: <https://jp.monhydro.com>
(または <https://github.com/Mandakha1/moncad/releases/latest> の `MonCad-2026-Setup.exe`)

## Windows (10 / 11、64 ビット)

1. `MonCad-2026-Setup.exe` をダウンロードします。
2. ダブルクリックしてインストールします。管理者権限は不要です
   (インストール先: `%LOCALAPPDATA%\Programs\MonCad`)。
   - 「Windows によって PC が保護されました」(SmartScreen) と表示されたら、
     **詳細情報** → **実行** をクリックします。MonCad にはまだコード署名がないため表示されます。
3. スタート メニュー → **MonCad 2026** を開きます。
4. 初回起動時に **Gmail でサインイン** の画面が出ます。ボタンを押すとブラウザが開くので、
   Google アカウントで一度だけサインインします。その後はオフラインでも使えます。

Windows の表示言語が日本語なら、MonCad は自動で日本語の画面になります。
手動で切り替えるには **編集 → 表示言語 → 日本語** (英語の画面では Edit → Interface Language)。
再起動は不要です。

サイレント インストール: `MonCad-2026-Setup.exe /S` (デスクトップのショートカットも作る場合は `/DESKTOP=1`)。
アンインストール: **設定 → アプリ → MonCad 2026 → アンインストール**。図面と設定
(`%APPDATA%\MonCad`) は削除されません。

### 動作環境

- Windows 10 / 11 (64 ビット)
- DirectX 12 または Vulkan に対応したグラフィックス (ここ 8〜10 年のほぼすべての PC)
- 日本語フォント: Windows 標準の游ゴシック (Yu Gothic) を使います。追加のインストールは不要です。

## ファイル形式

- **DXF** (R12〜2018): 読み込み・書き出しとも安定しています。他のソフトとのやり取りには DXF をおすすめします。
- **DWG**: 試験的な対応です。開くことはできますが、一部のオブジェクトが失われることがあります。
  DWG で保存して失われるものがある場合は、保存前に警告が出ます。大切な図面は DXF でも保存してください。
- **PDF、SVG、PNG**: 印刷・書き出し。

## AI で作図する (MCP)

Claude Code の場合 (インストール先のパスはインストール フォルダの `AI-connect.txt` にあります):

```
claude mcp add moncad -- "%LOCALAPPDATA%\Programs\MonCad\moncad-cli.exe" mcp --connect 127.0.0.1:7979 --launch
```

Claude Desktop、ChatGPT デスクトップ アプリ、Codex の設定も `AI-connect.txt` に書かれています。

## 記録される情報

初回起動時に Google (Gmail) で一度サインインします。起動のたびに、インストール コード、
プログラムのバージョン、Windows のバージョン、コンピュータのメーカーと機種、プロセッサ、
画面サイズ、表示言語、Windows のマシン ID のハッシュを monhydro.com に送ります
(サーバ側では IP アドレスと国も記録します)。図面、ファイル、入力した内容は送信されません。
詳細: <https://monhydro.com/jp/privacy.html>

## お問い合わせ

tsedeehvac@gmail.com
