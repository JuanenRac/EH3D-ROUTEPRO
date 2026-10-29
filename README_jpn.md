<p align="center">
  <img src="images/EH3D_BANNER.svg" alt="EH3D-ROUTEPRO banner" width="100%">
</p>

# 🪚 EH3D-ROUTEPRO

<p align="center"><a href="README.md">🇺🇸 English</a> | <a href="README_spa.md">🇪🇸 Español</a> | <a href="README_fra.md">🇫🇷 Français</a> | <a href="README_ita.md">🇮🇹 Italiano</a> | <a href="README_deu.md">🇩🇪 Deutsch</a> | <a href="README_zho.md">🇨🇳 中文</a> | 🇯🇵 <b>日本語</b> | <a href="README_por.md">🇵🇹 Português</a></p>

### Bungard CCD マシン向けの従来の DOS ソフトウェアのフライス加工作業を、HP-GL プロットファイルを入力として Rust でクロスプラットフォームに再実装し、元のソフトに似たテキストモードの画面を 8 言語で提供します

<p align="center">
  <img src="https://img.shields.io/badge/License-GPL%203.0--or--later-blue.svg" alt="GPL 3.0 or later">
  <img src="https://img.shields.io/badge/Stack-Rust-DEA584.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Languages-8-00E5FF.svg" alt="Languages">
  <img src="https://img.shields.io/badge/Version-0.0.4-0f6cbd.svg" alt="Version">
  <img src="https://img.shields.io/badge/Maturity-scaffolding-FFB020.svg" alt="Maturity">
</p>

---

**正直さの確認 - 今日実際に動くもの：** 80x25 のテキストモードインターフェースを持つ Rust プログラムです。HP-GL `.PLT` プロットファイル、工具テーブル、機械設定を読み込み、多段パスのフライス経路（早送り、切り込み、切削、引き上げ）を計画し、それを **シミュレーター上**（既定）で、または機械ファイルで指定すればパソコンのシリアルポート、USB シリアルアダプター、EH3D-NODE-BUNGARD 経由で **実機上** で実行します。パソコン上で 49 件のテストが動きます。EH3D-DRILLPRO と共有するシリアルプロトコルは、実機の Bungard CCD MTC で試しました（3 軸の原点出しと各軸の動き）。**フライス加工ジョブ全体はまだ機械で実行していません**。1 ミリあたりのステップ数は、オリジナルの設定と照合しただけです。 既定では MS-DOS 版のクラシック画面（原版とセル単位で照合済み）が表示されます。`--modern` でメニュー式の画面を起動します。 キーボードとマウスで操作でき、ウィンドウは最大化できます（画面は 640x400 から 1080p 以上まで拡大縮小）。設定ファイルと工具表はダイアログで編集・保存でき、「情報」ボタンで Bungard CCD と CCD/2 の機種（MTC 対応、ATC は未対応）を確認できます。 コンソール版（`routepro-console`）は同じ画面をターミナルに表示し、キーボードとマウスで操作でき、ウィンドウシステムは不要です（SSH、テキストコンソール、古い PC）。 各バージョンのパッケージには、1行ずつ解説した基本設定と、四角から10万点までの `examples` フォルダーが含まれます。ATC 機種の工具交換にはまだ対応していません。

---

> **安全性とセキュリティの現状：** このプログラムは安全システムではありません。実機接続（機械ファイルの `connection`）では **開始で機械が動きます**。工具を外し、機械の電源スイッチに手を置き、ジョブを即座に止めてステッピングドライバをオフにする `Esc` をいつでも押せるようにしてください。リミットスイッチは機械自身の制御ユニットが監視します（スイッチで軸を自分で止めます）。このソフトウェアには独自の非常停止がなく、誤ったステップバイトや速度は、工具を加工物やテーブルにぶつけたり、モーターに脱調を起こしたりすることがあります。プロトコル層は、アーム（有効化）されるまで何も動かしません。このプロジェクトは独立しており、Bungard Elektronik が製造・推奨・サポートするものではありません。製品名は、対応するファイル形式と機械を示すためだけに使っています。

## 📋 ひと目で

| | |
|---|---|
| バージョン | `0.0.4` |
| 成熟度 | 足場 |
| テスト | 49 |
| 技術スタック | Rust |
| 実機での動作 | はい、Bungard CCD MTC 1 台で：原点出しと各軸の動き。ジョブ全体はまだ実行していません。機械ファイルがなければシミュレーターで動きます |

**移動先：** [概要](#-概要) · [アーキテクチャ](#-アーキテクチャ) · [クイックスタート](#-クイックスタート) · [テスト](#-テスト) · [構成](#-リポジトリの構成) · [関連](#-関連プロジェクト)

## 🚦 プロジェクトの状況

各行はこのリポジトリの一部分で、信号は今日どこまで進んでいるかを示します。計画ではなく、リポジトリが示せるもの（テスト、ビルド、実行）に基づいて書かれています。

🟢 完了・検証済み · 🟡 動作する（既知の制限あり） · 🟠 一部のみ、または未検証 · 🔴 未実装または動作しない

| 領域 | 状態 | ここでの意味 |
|---|---|---|
| ソフトウェア | 🟡 | 80x25 のテキスト画面を持つ Rust プログラムが、HP-GL プロットファイル、工具テーブル、マシンファイルを読み、複数パスの切削経路を計画して、シミュレータ上で、またはマシンファイルの指定があれば実機で実行します。隣に EH3D-DRILLPRO のフォルダが必要です。 |
| テスト | 🟢 | 49 件のテストが通ります（`cargo test --workspace`）：HP-GL リーダー、経路計画、8 言語すべての画面。プロトコルは EH3D-DRILLPRO の 202 件のテストが対象です。 |
| HP-GL リーダー | 🟡 | 解釈できるのは `IN`、`SP`、`PU`、`PD`、`PA`、`PR` だけで、他のコマンドは数えて無視され、致命的になりません。テスト用に作った小さなサンプルでしか試していません。 |
| 経路計画 | 🟡 | 各経路はパスごとに早送り、切り込み、切削、持ち上げになり、深さは均等に分割され、工具は必ず表面で終わります。確認はテストとシミュレータのみです。 |
| 実機 | 🟠 | EH3D-DRILLPRO と共有するシリアルプロトコルは Bungard CCD MTC 1 台で試しました（基準点復帰と各軸の移動）。ルーティングのジョブを実機で実行したことはなく、スピンドルと集塵機の出力は未試験です。 |
| 運用上の安全 | 🟡 | `Esc` でジョブは一瞬で停止し、ステッピングドライバがオフになります。リミットスイッチはマシン自身の制御ユニットが監視しており、このソフトウェアに独自の非常停止はありません。 |
| サーバー接続 | 🟠 | `connection = server` を指定すると、プログラムはジョブを EH3D-SERVER に送り、その進行を追います。この経路はシミュレータでしか試していません。 |
| 配布パッケージ | 🟡 | 最初のリリースには Windows、Linux、macOS（Apple シリコンと Intel）用のパッケージがあり、それぞれ SHA-256 付きで、リリースワークフローが作成します。 |
| ドキュメント | 🟢 | 8 言語の README と、仮説のまま残っている点を含むプロトコルのメモ。マシンファイルのガイドは EH3D-DRILLPRO にあります。 |

## 🎯 概要

* **ファイルリーダー**：HP-GL の `.PLT` プロットファイル（`IN`、`SP`、`PU`、`PD`、`PA`、`PR`。その他のコマンドは数えて無視し、致命的エラーにはしません）、工具テーブル `ROUTEPRO.RTD`（深さ、パス数、XY と Z の送り速度、主軸回転数）、マシン設定 `ROUTEPRO.RCF`。
* **経路プランナー**：各経路は、始点への早送りのあと、パスごとに切り込み、経路に沿った切削、持ち上げになります。深さはパス間で均等に分けられ、工具は必ず表面に戻って終わります。
* **画面**：CGA の色、メニュー、ファイルダイアログ、穴のベクター表示を備えた 80x25 のテキストモード画面を、ウィンドウに表示します。英語、スペイン語、イタリア語、ドイツ語、フランス語、ポルトガル語、中国語、日本語に対応し、言語はメニューか起動時の `--lang` で選びます。
* **マシンプロトコル**（`protocol/`、クレート `eh3d-routepro-protocol`）：EH3D-DRILLPRO のプロトコルライブラリ（バイト転送、動作計画、安全ゲート）を再エクスポートし、フライス加工計画を追加します。
* **シミュレーター**：ハードウェアなしで作業全体を最後まで実行できます。テストもこれを使います。
* **対象機械**：DOS ソフトウェア 4.6 を使っていた、シリアルポート接続の制御ユニットを持つ機械。Bungard CCD MTC と CCD/2 MTC（手動工具交換。CCD/2 は小型の 325 x 270 mm テーブル）です。テーブルサイズなどの機械定数は機械の設定ファイルから取ります。試したのは CCD MTC のみで、スピンドルと集塵機の起動出力はまだ試していません。ATC の 2 モデル（CCD ATC と CCD/2 ATC、自動工具交換）は、リバースエンジニアリングで可能な範囲で、別のアプリケーションとして作ります。XL モデルとそれ以降の機械は、独自の最新制御ユニットとソフトウェアを備えており、対象外です。
* **機械ファイル**（`machine.cfg`、DRILLPRO と ROUTEPRO で共有）：ジョブが機械に届く方法（`demo` シミュレーター、`rs232` パソコン自身のシリアルポート、`usb` USB シリアルアダプター、`node` ネットワーク経由の EH3D-NODE-BUNGARD）と、1 行 1 値で、実機で調整したすべて（探索速度とランプ、校正ジャンプ、モーターの限界）を記します。
* **原点出し**：Z をリミットスイッチまで上げ、次に X と Y、最後にやわらかい校正ジャンプを行います。制御ユニットは RI ラインで「3 軸すべて原点」を知らせ、プログラムがそれを監視します。機械が原点に停止している場合は探索を省略し、X と Y をスイッチから 10 mm 離してから再度探索して、到着をその場で確認します。
* **安定した動き**：動きは、バイトの位置が時間を表す 1 本の連続ストリームとして送られます（そうしないと USB アダプターはバイトをまとめて届けます）。Z は 1 バイトずつ動かし、`Esc` はジョブを瞬時にキャンセルしてステッピングドライバをオフにします。
* **スピンドル**：既定では制御ユニット自身のもの（ラッチビット、最大 60000 rpm）。機械ファイルで `spindle = node` にすると、作業は代わりに EH3D-NODE-BUNGARD に頼みます（ノードの出力に付けたインバーター、ノードの最大までの任意の速度、停止のたびに停止）。ノードを使うと、プログラムは機械のランプも読むので、各軸が原点に着くのをストリームの途中で原点ラインを読まずにその場で確認できます。
* **サーバー接続**（機械ファイルで `connection = server`、`server_address`、`server_token`、`server_machine` を指定）：プログラムは自分で機械を動かす代わりに、作業を [EH3D-SERVER](https://github.com/JuanenRac/EH3D-SERVER) に送ります。サーバーが機械の定数で計画し、機械のノード経由で実行し、進み具合をプログラムに伝えます。プログラムはこれまでどおり進捗を表示してダイアログに答え、`Esc` でサーバー上の作業を停止します。プログラムを閉じても作業は止まりません。
* **作業アーカイブ、Offset ウィンドウ、シミュレーション**：作業は ZIP アーカイブにまとめられます（Select File ウィンドウがその中に入り、`routepro JOB.ZIP` は最初のルーター加工ファイルを読み込みます）。ファイル名の横の矢印は `*.PLT`、`*.ZIP`、`*.*` のパターンを一覧にします。工具ウィンドウと設定ウィンドウの Load ボタンは読み込むファイルを選びます。実機では Offset ウィンドウが原点復帰を行い、ウィンドウの動きすべてに追従し、再び原点復帰します。シミュレーションは実機と同じ時間をかけて進みます。

## 🧩 EH3D での役割

EH3D-ROUTEPRO は EH3D-DRILLPRO のフライス加工向けの姉妹プロジェクトで、パスで依存しています。フォルダー `EH3D-DRILLPRO` をこのフォルダーの隣に置く必要があります。シリアルプロトコル、画面、言語対応を再利用します。今のところ他の EH3D リポジトリには依存していません。

## 🏛️ アーキテクチャ

<p align="center">
  <img src="images/architecture.svg" alt="EH3D-ROUTEPRO のアーキテクチャ：誰が使い、何を持ち、何に依存するか" width="100%">
</p>

このリポジトリの各部分の関係：誰が使い、何を持ち、何に依存するか。図は `docs/architecture.json` から描かれ、コードと照合されます。

## 🚀 クイックスタート

安定版の Rust ツールチェーンが必要です。

```bash
cargo build --release --features gui
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
python -I tools/ci_validate.py
```

`run.bat`（Windows）または `./run.sh`（Linux と macOS）で実行します。どちらも必要ならプログラムを先にビルドします。どちらも Rust と隣のフォルダー `EH3D-DRILLPRO` が必要です：`run.bat [FILE.PLT] [--lang en|es|it|de|fr|pt|zh|ja] [--data DIR] [--dump]`。`--data` は `ROUTEPRO.RTD` と `ROUTEPRO.RCF` があるフォルダー（既定：`testdata/`）、`--dump` は画面をプレーンテキストで出力します。キー：`Alt` とハイライトされた文字でメニューを開き、矢印で移動、`Enter` で決定、`Esc` で戻ります。メニューバーはクリックもできます。中国語と日本語にはシステムフォントに CJK フォントが必要です。 `--connection FILE` で既定とは別の機械ファイルを指定できます（既定は Windows では `%APPDATA%\EH3D\machine.cfg`、その他では `~/.config/eh3d/machine.cfg`）。機械ファイルがなければプログラムはシミュレーターで動きます。 `--modern` でクラシック画面の代わりにメニュー式の画面を起動します。

## 🧪 テスト

テストスイートは 49 件のテストから成ります（`cargo test --workspace`）。HP-GL リーダー、経路計画（パス、突入、切削、退避）、プロトコルクレートのルーティング計画、従来の画面のセル単位の照合、8 言語すべての画面を検証します。再利用しているプロトコル（バイト、動作、安全ゲート）は EH3D-DRILLPRO でテストされています。CI では `cargo fmt --check` と、警告をエラーとする `cargo clippy` も実行し、`python -I tools/ci_validate.py` がマニフェスト、バージョン、変更履歴、8 つの README を検査します。

## 🏗️ ビルドとリリース

バージョンは `Cargo.toml` と `eh3d.project.json` にあり、各バージョンには [CHANGELOG.md](CHANGELOG.md) に項目があります。リリースは GitHub で公開されます。

**ビルド済みパッケージ:** 公開される各バージョンには、Windows（zip）、Linux（tar.gz）、macOS（Apple シリコンと Intel、tar.gz）用のパッケージが含まれ、プログラム、ライセンス、この README が入っており、それぞれ隣に SHA-256 が付きます。このリポジトリの `Release binaries` ワークフローでビルドされます。Releases ページからダウンロードしてください。Linux ではウィンドウに X11 または Wayland が必要です。

## 📂 リポジトリの構成

<p align="center">
  <img src="images/repository-structure.svg" alt="EH3D-ROUTEPRO のリポジトリ構成" width="100%">
</p>

<details>
<summary>ツリーのテキスト版</summary>

```text
EH3D-ROUTEPRO/
├── src/
│   ├── hpgl.rs                                  HP-GL リーダー（IN, SP, PU, PD, PA, PR）
│   ├── tools.rs                                 ツールテーブル ROUTEPRO.RTD
│   ├── job.rs                                   HP-GL の経路をルーティング作業に変換
│   ├── app.rs texts.rs                          アプリケーションと 8 言語のテキスト
│   └── bin/                                     実行ファイル routepro と routepro-console
├── protocol/src/
│   ├── route.rs                                 各経路のパス、突入、切削、退避
│   └── lib.rs                                   EH3D-DRILLPRO のプロトコルを再エクスポート
├── tests/                                       HP-GL の解析、経路計画、従来の画面、全言語の画面
├── docs/                                        プロトコルノート、従来の画面
├── testdata/                                    テスト用に書いた小さなサンプルファイル
├── config/                                      基本設定（1行ずつ解説）
├── examples/                                    例：四角から10万点
├── tools/                                       CI チェック、README チェック、図の生成ツール
├── images/                                      バナーとアーキテクチャ・リポジトリ構成の図
├── .github/                                     CI とリリースのワークフロー、Issue と PR のテンプレート
├── build.sh build.bat                           ビルドとテストのスクリプト
├── run.sh run.bat                               プログラムを起動
├── eh3d.project.json                            マニフェスト：バージョン、役割、成熟度、ビルドコマンド
├── Cargo.toml Cargo.lock                        Rust パッケージと固定された依存関係
├── CHANGELOG.md                                 各バージョンの変更点
├── SECURITY.md SUPPORT.md                       脆弱性の報告方法と問い合わせ先
└── CONTRIBUTING.md CODE_OF_CONDUCT.md LICENSE   貢献方法、行動規範、ライセンス
```

</details>

## 🗺️ ロードマップと既知の制限

**現時点で分かっている制限**

* 実機でのルーティング作業はまだ実行されていません。EH3D-DRILLPRO と共有するシリアルプロトコルは 1 台の Bungard CCD MTC で試しました（3 軸の原点復帰と各軸の動作）が、ミリあたりのステップ数は元の設定と照合しただけで、スピンドルと集塵機の出力は未試験です。
* 解釈できる HP-GL コマンドは `IN`、`SP`、`PU`、`PD`、`PA`、`PR` だけです。それ以外のコマンドは数えて無視し、致命的エラーにはしません。
* リーダーとプランナーは、テスト用に書いた小さなサンプルファイルでしか試していません。
* パスで EH3D-DRILLPRO に依存しているため、そのフォルダーをこのプログラムの隣に置く必要があります。
* 対応しているのは MTC モデルだけです。ATC モデルは未対応、XL モデルは対象外です。中国語と日本語にはシステムフォントに CJK フォントが必要です。

**次のステップ**

* 機械でルーティング作業を実行し、ミリあたりのステップ数、スピンドル、集塵機の出力を確認する。
* 実際の基板のプロットファイルで HP-GL リーダーを試す。
* HP-GL を、予定されている [EH3D-CORE-LIBS](https://github.com/JuanenRac/EH3D-CORE-LIBS) の共有リーダーで読み込む。

## 🌍 言語

README とテキストは、英語、スペイン語、フランス語、イタリア語、ドイツ語、中国語、日本語、ヨーロッパポルトガル語で提供されています。

## 🔗 関連プロジェクト

* [EH3D-DRILLPRO](https://github.com/JuanenRac/EH3D-DRILLPRO): 穴あけ向けの姉妹プロジェクト。このプロジェクトはそのプロトコル、画面、言語を使います
* [EH3D-CORE-LIBS](https://github.com/JuanenRac/EH3D-CORE-LIBS): スイート共通の G-code / Excellon リーダー。HP-GL もそこに追加する予定です
* [EH3D-COMMON](https://github.com/JuanenRac/EH3D-COMMON): スイートのプロジェクトコンテナとマシンテレメトリの型
* [EH3D-NODE-BUNGARD](https://github.com/JuanenRac/EH3D-NODE-BUNGARD)：機械の筐体内に置く ESP32-S3 ノード（イーサネットまたは Wi-Fi）。USB アダプターでは保てないタイミングで自身のシリアルポートからステップを送り出し、機械のランプを読み、スピンドルのドライブを制御します。独自の Web パネルがあります
* [EH3D-SERVER](https://github.com/JuanenRac/EH3D-SERVER)： EH3D ファミリーの中心。このプログラム（`connection = server`）や他のクライアントの作業を受け取り、ノード経由で機械上で実行します

## 📚 ドキュメント

* [プロトコルのメモ](docs/PROTOCOL.md): 制御ユニットの駆動方法についてわかっていること（EH3D-DRILLPRO と共通）、およびまだ仮説にとどまっていること。
* [接続と調整](https://github.com/JuanenRac/EH3D-DRILLPRO/blob/main/docs/CONNECTION.md)：機械ファイル、3 つの接続方法、各値
* [従来の画面](docs/CLASSIC.md)：MS-DOS プログラムの画面をどのように再現し検証するか
* [変更履歴](CHANGELOG.md)

## 🤝 コントリビュートとコミュニティ

* プルリクエストの前に `cargo fmt --all --check`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo test --workspace`、`python -I tools/ci_validate.py` を実行し、すべて通してください。[CONTRIBUTING.md](CONTRIBUTING.md) と[行動規範](CODE_OF_CONDUCT.md)を参照してください。
* README は 8 言語あります。文章を変更するときは 8 つのファイルすべてを変更し、2 つの図は `docs/architecture.json` と `docs/structure.json` から `tools/make_architecture.py` と `tools/make_structure.py` で描画します。
* シミュレーターでしか試していないことを、機械で動作するものとして書かないでください。
* 脆弱性は非公開で報告してください：[SECURITY.md](SECURITY.md)。質問、アイデア、報告：electrohobby3d@gmail.com（[SUPPORT.md](SUPPORT.md)）。

## 👤 作者

**JuanenRac (Electro Hobby 3D)** · electrohobby3d@gmail.com

## 📜 ライセンス

ソフトウェア：GPL-3.0-or-later。[LICENSE](LICENSE) を参照。スイートのハードウェア設計には CERN OHL-S を使います。
