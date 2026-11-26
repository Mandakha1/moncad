# MonCad — Japanese UI glossary and rules (`ja`)

Audience: Japanese architects, civil and mechanical engineers and students who know AutoCAD
(日本語版) or Jw_cad. Use the words the Japanese edition of AutoCAD uses, so the menus feel
familiar. Plain, short, modern software Japanese. Labels are nouns or noun phrases; messages are
polite です/ます sentences; command-line prompts use AutoCAD's terse form (「〜を指定」).

## Hard format rules (a test enforces 1–4)
1. Keep every `{placeholder}` exactly (same names, same count). You may move it.
2. If the English ends with `...` the translation ends with `...` (three ASCII dots); if it ends
   with `…` it ends with `…`. If the English does NOT end with an ellipsis, neither does the
   translation.
3. Mirror leading/trailing spaces of the English exactly (`  Cancel  ` → `  キャンセル  `,
   ` or [` → ` または [`).
4. Never use a TAB or newline. A literal `\n` in the English stays a literal `\n`.
5. Shortcuts and symbols stay: Ctrl+S, F3, ⌘, ⇧, ↵, %, °, Δ, ±, Ø.
6. Product, format and technology names stay in Latin letters: MonCad, Mon3D, MonBIM, CADCraft,
   AutoCAD, DXF, DWG, PDF, SVG, PNG, JPEG, AutoLISP, LSP, MCP, OLE, UCS, WCS, OpenGL, GPU, Git,
   GitHub, URL, ID, CTB, STB, SHX, TrueType, ISO, ANSI, JIS, DIN, Express, ViewCube, Claude, ChatGPT.
7. Command names, system variables and typed keywords stay English: LINE, PLINE, OSMODE,
   DIMSCALE, CONSTRAINTINFER, SELECTSIMILARMODE, ByLayer, ByBlock, Continuous (linetype name),
   SOLID (hatch pattern name), Standard (style name) when it names a style stored in the drawing.
8. Everything inside `[...]` and `<...>` in a prompt stays exactly as in the English
   (`Enter an option [Add/Remove] <Add>` → `オプションを入力 [Add/Remove] <Add>`): the command line
   matches what the user types against those words. Only the words outside the brackets change.
9. `@plural` rows: ONE form (Japanese has no plural): `{n} commit` → `{n} 件のコミット`.
10. `menu` rows are the top menu headings: View → 表示, Draw → 作成, Dimension → 寸法,
    Modify → 修正.

## Typography
- Half-width space between Latin letters, digits or a `{placeholder}` and Japanese text, as AutoCAD
  does: `3D ポリライン`, `DXF ファイル`, `2 点目を指定`, `{n} 個のオブジェクト`.
- Half-width space between separate katakana loanwords in a compound, as AutoCAD does:
  `オブジェクト スナップ`, `コマンド ライン`, `ブロック エディタ`, `ステータス バー`,
  `ツール パレット`, `マルチ テキスト`, `ラスター イメージ`. No space between kanji and katakana
  (`画層プロパティ管理`, `寸法スタイル管理`, `文字スタイル`).
- Drop the final ー on technical loanwords from -er/-or/-ar, as AutoCAD does: ユーザ, パラメータ,
  エディタ, フィルタ, プロッタ, プリンタ, マネージャ, ビューア, コンピュータ, ジオメトリ,
  フォルダ, ヘッダ, スライダ, セパレータ. Keep established ones: カラー, メジャー, カーソル, レイヤー
  (only when quoting; the term is 画層), ドライバー → ドライバ.
- Sentence end: `.` → `。`; `?` → `？`; `!` → `！`; a sentence comma → `、`. A label colon `:`
  stays a half-width `:`. Parentheses, `/`, `+`, `=`, `<`, `>`, `[`, `]`, `-` stay half-width.
- English quotation marks around a name or a word → 「」 (`"Untitled"` → 「無題」).
- No ください in prompts or buttons. Sentences in messages and tooltips use です/ます
  (`Could not open the file.` → `ファイルを開けませんでした。`).

## Prompts (command line)
- `Specify X` → `X を指定` (`Specify center point` → `中心点を指定`,
  `Specify radius of circle` → `円の半径を指定`, `Specify second point` → `2 点目を指定`).
- `Enter X` → `X を入力` (`Enter value` → `値を入力`).
- `Select X` → `X を選択` (`Select objects` → `オブジェクトを選択`).
- A prompt ending in ` or` ends in ` または` (`Select first object or` → `1 番目のオブジェクトを選択 または`).
- ` or [` → ` または [`.
- Questions: `Fix any errors detected?` → `検出されたエラーを修正しますか？`.
- `(or press ENTER)` → `(または Enter キーを押す)`.

## Menus / general UI
File ファイル · Edit 編集 · View (menu) 表示 · View (a named view object) ビュー · Insert 挿入 ·
Format 形式 · Tools ツール · Draw (menu, ribbon panel) 作成 · Dimension (menu) 寸法 ·
Modify 修正 · Window ウィンドウ · Help ヘルプ · Parametric パラメトリック · Express Express
New 新規作成 · Open 開く · Open Recent 最近使用したファイル · Save 上書き保存 ·
Save As... 名前を付けて保存... · Close 閉じる · Quit / Exit 終了 · Plot / Print 印刷 ·
Plot Preview 印刷プレビュー · Export 書き出し · Import 読み込み · Undo 元に戻す · Redo やり直し ·
Cut 切り取り · Copy (clipboard) コピー · Copy with Base Point 基点コピー · Paste 貼り付け ·
Delete / Erase 削除 · Select All すべて選択 · Find 検索 · Replace 置換 ·
Options オプション · Preferences / Settings 設定 · User Preferences 基本設定 · OK OK ·
Cancel キャンセル · Apply 適用 · Yes はい · No いいえ · Reset リセット · Default 既定 ·
Current 現在 · Properties プロパティ · Name 名前 · Description 説明 · Rename 名前変更 ·
Show 表示 · Hide 非表示 · Lock ロック · Unlock ロック解除 · On オン · Off オフ ·
Freeze フリーズ · Thaw フリーズ解除 · Browse... 参照... · Command コマンド ·
Command line コマンド ライン · Option オプション · Error エラー · Warning 警告 ·
Tooltip ツールチップ · Palette パレット · Toolbar ツールバー · Ribbon リボン ·
Status bar ステータス バー · Tab タブ · Start tab スタート タブ · Recent 最近使用した ·
Contributors 貢献者 · Interface Language 表示言語 · Auto (system language) 自動 (システムの言語) ·
Theme テーマ · Light ライト · Dark ダーク · Shortcut ショートカット キー ·
Shortcut menu ショートカット メニュー · Accessibility アクセシビリティ · Update 更新 ·
Check for updates 更新を確認 · Workspace ワークスペース · Tool Sets ツール セット ·
Command Reference コマンド リファレンス · About MonCad MonCad について · Account アカウント ·
Untitled 無題 · Drawing (the document) 図面 · Template テンプレート · Folder フォルダ

## Objects
Line 線分 · Construction Line 構築線 · Ray 放射線 · Multiline マルチライン · Polyline ポリライン ·
3D Polyline 3D ポリライン · Circle 円 · Arc 円弧 · Ellipse 楕円 · Elliptical Arc 楕円弧 ·
Rectangle 長方形 · Polygon ポリゴン · Point 点 · Spline スプライン · Helix らせん ·
Donut ドーナツ · Revision Cloud 雲マーク · Wipeout ワイプアウト · Region リージョン ·
Boundary 境界 · Hatch ハッチング · Gradient グラデーション · Text 文字 ·
Multiline Text / MText マルチ テキスト · Single Line Text 文字記入 · Table 表 · Block ブロック ·
Attribute 属性 · Attribute Definition 属性定義 · Block Reference ブロック参照 · Xref 外部参照 ·
Raster Image ラスター イメージ · Underlay アンダーレイ · Viewport ビューポート ·
Layout レイアウト · Model モデル · Model space モデル空間 · Paper space ペーパー空間 ·
Leader 引出線 · Multileader マルチ引出線 · Dimension (object) 寸法 · Linear 長さ寸法 ·
Aligned 平行寸法 · Angular 角度寸法 · Radius 半径 (dimension: 半径寸法) ·
Diameter 直径 (dimension: 直径寸法) · Arc Length 弧長寸法 · Ordinate 座標寸法 ·
Jogged 折り曲げ半径寸法 · Baseline 並列寸法 · Continue (dimension) 直列寸法 ·
Quick Dimension クイック寸法記入 · Tolerance 幾何公差 · Center Mark 中心マーク ·
Centerline 中心線 · Inspection 検査寸法 · Dimension Break 寸法マスク · Dimension Space 寸法スペース ·
Solid (3D) ソリッド · 2D Solid 2D 塗り潰し · 3D Face 3D 面 · Mesh メッシュ · Surface サーフェス ·
Box 直方体 · Cylinder 円柱 · Cone 円錐 · Sphere 球 · Torus トーラス · Wedge くさび ·
Pyramid 角錐 · Polysolid ポリソリッド · Object オブジェクト · Entity オブジェクト · Vertex 頂点 ·
Segment セグメント · Layer 画層 · Linetype 線種 · Lineweight 線の太さ · Color 色 ·
Transparency 透過性 · Plot Style 印刷スタイル · Text Style 文字スタイル ·
Dimension Style 寸法スタイル · Table Style 表スタイル · Multileader Style マルチ引出線スタイル ·
Style スタイル · Units 単位 · Limits 図面範囲 · Extents オブジェクト範囲 · Group グループ ·
Selection 選択 · Selection set 選択セット · Field フィールド · Hyperlink ハイパーリンク ·
Annotative 異尺度対応 · Annotation scale 注釈尺度 · Thickness 厚さ · Elevation 高度

## Modify / 3D
Move 移動 · Copy (object) 複写 · Rotate 回転 · Scale 尺度変更 (the factor: 尺度) · Mirror 鏡像 ·
Offset オフセット · Trim トリム · Extend 延長 · Stretch ストレッチ · Lengthen 長さ変更 ·
Break 部分削除 · Break at Point 点で部分削除 · Join 結合 · Fillet フィレット · Chamfer 面取り ·
Blend Curves ブレンド曲線 · Explode 分解 · Array 配列複写 · Rectangular Array 矩形状配列複写 ·
Polar Array 円形状配列複写 · Path Array パス配列複写 · Align 位置合わせ ·
Match Properties プロパティ コピー · Grip グリップ · Purge 名前削除 · Audit 監査 · Recover 修復 ·
Divide ディバイダ · Measure (command) メジャー · Draw Order 表示順序 · Bring to Front 最前面へ移動 ·
Send to Back 最背面へ移動 · Extrude 押し出し · Revolve 回転体 · Sweep スイープ · Loft ロフト ·
Union 和 · Subtract 差 · Intersect 交差 · Slice スライス · Thicken 厚さ付加 ·
Presspull 押し出し/押し込み · 3D Move 3D 移動 · 3D Rotate 3D 回転 · 3D Mirror 3D 鏡像 ·
3D Array 3D 配列複写 · 3D Align 3D 位置合わせ

## View / drafting aids
Zoom ズーム · Zoom In 拡大 · Zoom Out 縮小 · Zoom Extents オブジェクト範囲ズーム ·
Zoom All 図面全体 · Zoom Window 窓 · Zoom Previous 前画面 · Realtime リアルタイム ·
Pan 画面移動 · Regen / Regenerate 再作成 · Regen All 全再作成 · Redraw 再描画 ·
Named Views ビュー管理 · Orbit オービット · Visual Style 表示スタイル ·
2D Wireframe 2D ワイヤフレーム · Wireframe ワイヤフレーム · Hidden 隠線処理 ·
Realistic リアリスティック · Conceptual コンセプト · Shaded シェーディング ·
Shaded with Edges エッジ付きシェーディング · Sketchy スケッチ · X-Ray X 線 ·
Top 平面図 · Bottom 底面図 · Left 左側面図 · Right 右側面図 · Front 正面図 · Back 背面図 ·
SW Isometric 南西アイソメ (SE 南東, NE 北東, NW 北西) · Plan View プラン ビュー ·
Isometric アイソメ · Snap スナップ · Grid グリッド · Ortho 直交モード ·
Polar Tracking 極トラッキング · Object Snap オブジェクト スナップ ·
Object Snap Tracking オブジェクト スナップ トラッキング · Dynamic Input ダイナミック入力 ·
Drafting Settings 作図補助設定 · Endpoint 端点 · Midpoint 中点 · Center 中心 ·
Geometric Center 図心 · Node 点 · Quadrant 四半円点 · Intersection 交点 · Extension 延長 ·
Insertion 挿入基点 · Perpendicular 垂線 · Tangent 接線 · Nearest 近接点 ·
Apparent Intersection 仮想交点 · Parallel 平行 · Mid Between 2 Points 2 点間中点 ·
Coordinates 座標 · Angle 角度 · Distance 距離 · Length 長さ · Width 幅 · Height 高さ ·
Area 面積 · Perimeter 周長 · Base point 基点 · Start point 始点 · End point 終点 ·
First corner 一方のコーナー · Opposite corner もう一方のコーナー · Insertion point 挿入位置 ·
Scale factor 尺度 · Displacement 移動距離 · Origin 原点 · Axis 軸 · Direction 方向 ·
Crosshair クロスヘア カーソル · Pickbox ピックボックス · Aperture 照準 · AutoSnap オートスナップ

## Selection
Window 窓選択 · Crossing 交差選択 · Fence フェンス · Window Polygon ポリゴン窓 ·
Crossing Polygon ポリゴン交差 · Lasso 投げ縄 · Previous 前回 · Last 最後 · All すべて ·
Add 追加 · Remove 除外 · Quick Select クイック選択 · Select Similar 類似オブジェクトを選択 ·
Selection Cycling 選択の循環 · Selection Preview 選択プレビュー

## Layers
Layer Properties Manager 画層プロパティ管理 · Layer States Manager 画層状態管理 · Layer state 画層状態 ·
Status 状態 · Plot 印刷 · New VP Freeze 新規 VP フリーズ · Make Current 現在に設定 ·
Isolate 選択表示 · Layer Off 画層非表示 · Freeze/Thaw in all viewports すべてのビューポートでフリーズ/フリーズ解除

## Text, dimensions, hatch, plot
Font フォント · Width factor 幅係数 · Oblique angle 傾斜角度 · Justify 位置合わせ · Bold 太字 ·
Italic 斜体 · Underline 下線 · Overline 上線 · Strikethrough 取り消し線 · Stack スタック ·
Paragraph 段落 · Columns 段組み · Symbol 記号 · Find and Replace 検索と置換 ·
Spell Check スペル チェック · Dimension Style Manager 寸法スタイル管理 · Dimension line 寸法線 ·
Extension line 寸法補助線 · Arrowhead 矢印 · Arrow size 矢印のサイズ · Text height 文字の高さ ·
Precision 精度 · Primary Units 基本単位 · Alternate Units 変換単位 · Tolerances 許容差 ·
Fit フィット · Suppress 省略 · Leading zeros 先頭の 0 · Trailing zeros 末尾の 0 ·
Hatch and Gradient ハッチングとグラデーション · Pattern パターン · Spacing 間隔 ·
Pick points 点をクリック · Islands 島の検出 · Island 島 · Associative 自動調整 ·
Gap tolerance 許容差 · Inherit Properties プロパティを継承 · Page Setup ページ設定 ·
Page Setup Manager ページ設定管理 · Plotter プロッタ · Plotter Manager プロッタ管理 ·
Paper size 用紙サイズ · Plot area 印刷対象 · Plot scale 印刷尺度 · Plot offset 印刷オフセット ·
Center the plot 印刷の中心 · Fit to paper 用紙にフィット · Plot style table 印刷スタイル テーブル ·
Portrait 縦 · Landscape 横 · Display (plot area) 表示画面

## Units
Decimal 10 進表記 · Architectural 建築図面表記 · Engineering 工業図面表記 · Fractional 分数表記 ·
Scientific 指数表記 · Decimal Degrees 度 · Deg/Min/Sec 度/分/秒 · Grads グラード ·
Radians ラジアン · Surveyor's Units 測量単位 · Clockwise 時計回り · Insertion scale 挿入尺度 ·
Millimeters ミリメートル · Centimeters センチメートル · Meters メートル · Kilometers キロメートル ·
Inches インチ · Feet フィート · Yards ヤード · Miles マイル · Microns ミクロン · Unitless 単位なし

## Parametric
Constraint 拘束 · Geometric constraint 幾何拘束 · Dimensional constraint 寸法拘束 ·
Coincident 一致 · Collinear 同一線上 · Concentric 同心円 · Fix 固定 · Parallel 平行 ·
Perpendicular 垂直 · Horizontal 水平 · Vertical 鉛直 · Tangent 正接 · Smooth 滑らか ·
Symmetric 対称 · Equal 等しい · Infer constraints 推測拘束 · AutoConstrain 自動拘束 ·
Parameters Manager パラメータ管理 · Constraint Bar 拘束バー · Expression 式 ·
User parameter ユーザ パラメータ · Block Editor ブロック エディタ · Dynamic block ダイナミック ブロック

## Inquiry
Inquiry 問い合わせ · Distance 距離 · Area 面積 · Volume 体積 · List リスト表示 · ID Point 位置表示 ·
Mass Properties 領域/質量特性 · Set Variable システム変数設定
