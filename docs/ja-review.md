# Japanese interface: rows for a native speaker to check

MonCad 0.23.0 adds a complete Japanese catalog (`crates/ui-egui/src/i18n/ja.tsv`, 2,056 rows,
nothing falls back to English). It was written from the English strings with AI assistance
(Claude Code), using the terms of the Japanese edition of AutoCAD (`docs/moncad/ja-glossary.md`).
No native Japanese speaker has reviewed it yet. These are the rows the translators were least
sure of. Please check them first; any other row may be wrong too.

To change a row, edit the third column of `ja.tsv` (keep `{placeholders}`, trailing `...`/`…`
and leading/trailing spaces as in the English), then run
`cargo test -p cadcraft-ui-egui --lib i18n`.

## One English key, two meanings

These keys are shared by two places in the code, so one Japanese word has to serve both. A good
fix splits the key in the source (a `menu` row, or a different English label), not the catalog.

| English | Used as | Japanese now | The other place would want |
|---|---|---|---|
| `Copy` | Modify > Copy (COPY) and Edit > Copy (COPYCLIP) | `複写` | `コピー` for the clipboard |
| `Scale` | Modify > Scale (SCALE) and the Scale property fields | `尺度` | `尺度変更` for the command |
| `Tangent` | object snap and geometric constraint | `接線` | `正接` for the constraint |
| `Perpendicular` | object snap and geometric constraint | `垂線` | `垂直` for the constraint |
| `Linear` | linear dimension and the gradient shape | `長さ寸法` | `線形` for the gradient |
| `Continue` | continued dimension and Draw > Arc > Continue | `直列寸法` | `連続` for the arc |
| `From` | snap override and QuickCalc unit converter | `基点設定` | `変換元` in the converter |
| `Save` | File > Save and the button in the save-changes dialog | `上書き保存` | `保存` on the button |

The Window menu heading already has its own `menu` row (`ウィンドウ`), so View > Zoom > Window
and the plot-area option read `窓`.

## Rows the translators were unsure of

| Row | English | Japanese | Why |
|---|---|---|---|
| 7 | `*VARIES*` | `*さまざま*` | Taken from how the AutoCAD JP Properties palette shows it; please confirm. |
| 33 | `<Offset>` | `<オフセット>` | **Changed after review:** now `<オフセット>`. Left as is under glossary rule 8. This is the whole FROM prompt, not a typed default, so AutoCAD JP would show `<オフセット>`. |
| 41 | `About` | `バージョン情報` | Tab in the About dialog, next to Contributors and Models. `概要` would also work. |
| 65 | `Align Text` | `寸法値位置合わせ` | Used only as the Dimension > Align Text submenu (DIMTEDIT). This is wrong if the same key is later reused for TEXTALIGN. |
| 82 | `Annotation Visibility` | `注釈オブジェクトの表示` | ANNOALLVISIBLE status-bar toggle; I am not sure of the exact AutoCAD JP tooltip. |
| 85 | `Annotative Update` | `異尺度対応の更新` | ANNOUPDATE label; I am not sure of the exact JP term. |
| 100 | `Associate` | `関連付け` | No use found in the code. The glossary has only Associative = 自動調整. |
| 103 | `Associative dimension priority` | `自動調整寸法の優先度` | QDIM settings prompt; I am not sure of the exact JP prompt. |
| 121 | `Back` | `背面` | In the code it is only a keyword (UNDO Back = 後退, DRAWORDER Back = 最背面), and it shows in the right-click menu. The glossary's 背面図 is for the view, which the code does not have. |
| 129–130 | `Batch Publish(...)` | `一括パブリッシュ(...)` | AutoCAD for Mac term; I could not confirm the JP wording. |
| 149 | `Bottom` | `下端` | Used only as MLINE justification (上端/ゼロ/下端). The glossary's 底面図 is for the view. |
| 159–161 | `Break-line Symbol` / `Breakline ...` | `破断線記号` / `破断線の延長距離` / `破断線記号のサイズ` | Express Tools BREAKLINE; I do not know the official JP name. |
| 191 | `Centered` | `中央` | Hatch dialog gradient checkbox; I am not sure whether JP uses 中央 or 中心. |
| 225 | `Closed` | `閉じる` | Polyline property label (as in the AutoCAD JP Properties palette). It is the same text as the Close button. |
| 263 | `Constrained Orbit` | `拘束オービット` | I am not sure of the JP menu term. |
| 267 | `contains authoring elements. Open in Block Editor?` | `にはオーサリング要素が含まれています。ブロック エディタで開きますか？` | Shown after the block name (`{name} {msg}`). I am not sure whether JP AutoCAD says オーサリング要素. |
| 269 | `Continue` | `直列寸法` | One key serves DIMCONTINUE (palette and Dimension menu) and Draw > Arc > Continue, where 直列寸法 is wrong. |
| 280 | `Copy` | `複写` | One key serves both Modify > Copy and the palette tool (複写), and Edit > Copy / Ctrl+C on the clipboard (コピー in AutoCAD JP). The source should split them. |
| 286 | `Copy Selection` | `複写選択` | Right-click menu item, from memory of the AutoCAD JP right-click menu. |
| 291 | `Count Table` | `カウント表` | COUNTTABLE; the JP UI may say カウント テーブル. |
| 333 | `Defined width` | `定義された幅` | MTEXT property label; I am not sure of the exact JP palette wording. |
| 335 | `Degree` | `度` | MTEXT symbol tooltip (°). It is also the SPLINE keyword `Degree`, which is 次数. |
| 375 | `Display` | `表示` | Options dialog tab. The plot-area value `Display` (表示画面) uses the same key, but `managers.rs` does not translate it yet. |
| 387 | `Drafting` | `製図` | Options tab (AutoCAD JP: 作図補助) and the Tool Sets tab Drafting/Modeling share this key. |
| 388 | `Drafting mode?` | `作図モード？` | ISODRAFT prompt; I am not sure of the JP wording. |
| 398 | `Drawing as bytes` | `図面 (バイト列)` | MCP/automation command label, with no AutoCAD equivalent. |
| 399 | `Drawing Exchange Binary...` | `図面交換バイナリ...` | Insert menu item (DXB), from memory of the classic JP menu. |
| 403 | `Drawing Units` | `単位管理` | Dialog title, as in AutoCAD JP UNITS. The literal translation would be 図面単位. |
| 407 | `DView` | `DView` | Kept as the command name. AutoCAD JP may show ダイナミック ビュー. |
| 418 | `Dynamic Dimensions` | `ダイナミック寸法` | DCDISPLAY (dynamic dimensional constraints). Not sure whether JP AutoCAD says ダイナミック寸法 or 動的拘束 here. Row 803 uses the same term. |
| 451 | `ENTER` | `Enter キー` | Right-click options, Command Mode. JP AutoCAD may show only "Enter". |
| 450 | `Enter` | `Enter` | Context menu item, left in Latin as AutoCAD JP does ("Enter(E)"). |
| 472 | `Enter an implied edge extension mode` | `暗黙のエッジ延長モードを入力` | TRIM/EXTEND Edge mode. I don't know the exact JP AutoCAD wording. |
| 509 | `Enter dimension value or [name=expression]` | `寸法値を入力 または [名前=式]` | **Changed after review:** now `寸法値を入力 または [名前=式]`. Bracket kept verbatim per rule 8, but [name=expression] is a hint, not a keyword. [名前=式] may read better. |
| 548 | `Enter maximum part height` | `分割部分の最大高さを入力` | Table breaks. There is no standard term for a table "part". |
| 588 | `Enter new order` | `新しい次数を入力` | SPLINEDIT elevate order: order = degree + 1, but JP AutoCAD seems to use 次数 for both. |
| 675 | `Enter X scale factor, specify opposite corner` | `X 方向の尺度を入力、もう一方のコーナーを指定` | Uses a comma list like JP INSERT. Check how it reads once " または [Corner/XYZ]" is added. |
| 685 | `Evaluate` | `計算` | Tooltip of the QuickCalc "=" button. 評価 would be the literal term. |
| 706 | `Face` | `面` | No usage found in the Rust sources (possibly a stale key), so the meaning is a guess. |
| 718 | `First` | `初回` | Short column or sort label for "First commit" in Credits. |
| 740 | `From` | `基点設定` | One key serves two uses: the snap-override menu (FROM = 基点設定) and the QuickCalc unit-conversion "From"/"To" label (変換元 would fit there). The two may need separate keys. |
| 742 | `Front` | `正面図` | No usage found in the sources. If this is a ViewCube face label, JP uses 前. |
| 753 | `Global` | `一括` | Modify > Object > Attribute > Global (-ATTEDIT). I believe JP AutoCAD uses 一括. |
| 755 | `Grab bag` | `一覧` | Credits view toggle (free-flowing names vs "Table"). There is no fixed term. |
| 789 | `Hatch Pick` | `ハッチング選択` | HATCHPICK command name. The prompt picks points/objects/origin; ハッチング ピック is an alternative. |
| 798 | `Hemispherical` | `半球` | Gradient pattern name (HEMISPHERICAL). 半球状 is an alternative. |
| 799 | `Hidden` | `隠線処理` | No usage found. Treated as the visual style, as the Mongolian hint suggests. If it is the linetype HIDDEN or a layer, it should stay as it is. |
| 807 | `Hide Others` | `ほかを隠す` | Assumed to be the macOS app menu (standard JP macOS wording). Not found in the sources. |
| 819 | `Implied Loops` | `暗黙のループ` | LOOPS inquiry command (closed regions formed by crossing curves). There is no established AutoCAD term. |
| 857, 858 | `Interface Theme / Interface theme` | `配色テーマ` | AutoCAD JP Options term; glossary only lists Theme テーマ |
| 856 | `Interface scale` | `インタフェースの拡大率` | UI zoom percentage (125 %); no AutoCAD term |
| 880 | `Justify` | `両端揃え` | Only used as the MText paragraph alignment button; glossary gives 位置合わせ (TEXT justification) |
| 933 | `Linear` | `長さ寸法` | Same key also labels the gradient shape "Linear" in the hatch dialog, where 線形 would fit |
| 1181 | `Perpendicular` | `垂線` | Same key is the osnap mode (垂線) and the geometric constraint button (垂直); chose the osnap |
| 875 | `Jogged` | `折り曲げ半径寸法` | Glossary term; the AutoCAD JP Dimension menu item may be just 折り曲げ |
| 876 | `Jogged Linear` | `寸法線折り曲げ` | DIMJOGLINE; not sure of the exact AutoCAD JP label |
| 1121 | `Oops` | `復活` | OOPS command; I believe AutoCAD JP calls it 復活 |
| 1029 | `Multiple Insert` | `複数挿入` | MINSERT; not sure of the AutoCAD JP name (配列挿入?) |
| 1154 | `Override` | `優先` | Dimension > Override (DIMOVERRIDE) |
| 1112 | `Oblique` | `傾斜` | Dimension > Oblique (DIMEDIT) |
| 967 | `LwPolyline` | `LW ポリライン` | Entity type name; AutoCAD shows ポリライン / ライトウェイト ポリライン |
| 1205 | `Plot Window` | `印刷範囲の窓` | PLOTWINDOW: picks the "Window" plot area; no AutoCAD label |
| 1197 | `Plot style attached to` | `印刷スタイルの適用先` | Properties palette row; AutoCAD JP wording may differ |
| 1203 | `Plot table type` | `印刷テーブルの種類` | Properties palette row |
| 872 | `Isometric plane?` | `アイソメ平面は？` | Question-style prompt with Left/Top/Right keywords |
| 1033 | `N` | `北` | ViewCube compass letter; kept short, as on the JP ViewCube |
| 1031 | `My` | `マイ` | "All / My" property-set switch |
| 1155 | `overrides built-in` | `組み込みより優先` | Alias editor status: a user alias replaces a built-in one |
| 1100 | `Object snap aperture` | `オブジェクト スナップの照準` | Aperture size setting |
| 1216 | `Polar distance` | `極距離` | PolarSnap spacing in Drafting Settings |
| 890 | `Largest array` | `配列複写の最大数` | MAXARRAY in Options, followed by "個のオブジェクト" |
| 1109 | `objects` | `個のオブジェクト` | Only used as a suffix after a number (MAXARRAY field) |
| 969 | `Major line every` | `主線の間隔` | Grid major-line frequency |
| 1182 | `Phantom` | `二点鎖線` | Linetype description; if it is the PHANTOM linetype name it should stay Latin |
| 891 | `Last` | `最後` | Credits column header for "Last commit" (date); glossary selection term 最後 |
| 1141 | `Orientation` | `図面の方向` | Page setup orientation |
| 942, 945 | `Linetype... / Lineweight...` | `線種設定... / 線の太さ設定...` | AutoCAD JP Format menu names; glossary terms are 線種 / 線の太さ |
| 1212 | `Polar` | `極トラッキング` | Toggle name (`<Polar on>`); 極 alone used in the tracking tooltip (row 1221) |
| 1237 | `Previous` | `前画面` | The only UI use is View > Zoom > Previous. The glossary's 前回 (selection) would be wrong there, but it is right if the key is ever reused for selection. |
| 1239 | `Previous Layer` | `前の画層` | LAYERP. Not sure of the exact AutoCAD JP ribbon label. |
| 1255 | `Query Entities` | `オブジェクトの問い合わせ` | A MonCad-only inquiry command with no AutoCAD equivalent. 照会 is also possible. |
| 1256 | `Query Texts` | `文字の問い合わせ` | Same as 1255. |
| 1264 | `QuickCalc Pick` | `クイック計算 (図面から取得)` | A MonCad command that picks a point or value from the drawing for QuickCalc. No established term. |
| 1278 | `Recent` | `最近使用したファイル` | Start-tab heading over the recent-files list. The glossary's bare 最近使用した reads as unfinished on its own. |
| 1301 | `Relative` | `相対座標` | Dynamic Input radio button (the other choice is Absolute). AutoCAD JP says 相対座標 / 絶対座標, so "Absolute" in another chunk should be 絶対座標 to match. |
| 1306 | `Remove` | `削除` | Used only as a list button (aliases, plot styles), so 削除 rather than the glossary's selection term 除外. |
| 1342 | `Right` | `右` | One key is shared by MTEXT paragraph alignment (右揃え would read better), DIMTEDIT, Pan and the isometric plane. |
| 1354 | `Save` | `上書き保存` | Follows the glossary. The same key is the button in the close / save-changes dialog, where plain 保存 may read better. |
| 1375 | `Scale` | `尺度` | One key covers the Modify > Scale command (glossary: 尺度変更) and several property labels (hatch, block, palette: 尺度). I chose 尺度 because 尺度変更 on a property field is wrong. Splitting the key would fix it. |
| 1381 | `Screening` | `スクリーン` | Plot-style table column. I believe AutoCAD JP uses スクリーン; please confirm. |
| 1473 | `Select object to break dimension` | `寸法マスクを作成するオブジェクトを選択` | DIMBREAK. Follows the glossary term 寸法マスク. Not sure of the exact AutoCAD JP wording. |
| 1485 | `Select objects first, then choose a command` | `先にオブジェクトを選択してからコマンドを実行` | PICKFIRST option. AutoCAD JP calls it 名詞/動詞選択, but I kept the descriptive English. |
| 1491 | `Select objects to stretch by crossing-window or crossing-polygon...` | `交差選択またはポリゴン交差で、ストレッチするオブジェクトを選択...` | Uses the glossary's 交差選択 / ポリゴン交差. AutoCAD JP's own echo text says 交差窓または交差ポリゴン. |
| 1545 | `Sheet Books to PDF...` | `シート ブックを PDF に書き出し...` | "Sheet Books" is a MonCad concept with no AutoCAD JP term (AutoCAD has シート セット). |
| 1552 | `Show All Dynamic Dimensions` | `すべての動的寸法を表示` | Dynamic dimensional constraints. 動的拘束 or ダイナミック寸法 are also possible. |
| 1565 | `Single...` | `単一...` | Modify > Object > Attribute > Single... Not sure whether AutoCAD JP uses 単一 or 個別. |
| 1605 | `Specify block attachment` | `ブロックの接続位置を指定` | MLEADER block attachment (Center extents / Insertion point). The wording is uncertain. |
| 1606 | `Specify center location override` | `中心位置の優先を指定` | Jogged-radius center override. The wording is uncertain. |
| 1642 | `Specify distance between turns` | `回転の間隔を指定` | HELIX turn height or pitch. 1 回転の高さ is an alternative. |
| 1287 | `Red, Green, Blue` | `赤, 緑, 青` | True-color prompt that shows the typing format. I kept ASCII commas (not 、) because the user types commas. |
| 1999 | `Window` | `窓` | **Changed after review:** now `窓`. One plain row serves the Window menu heading (no `menu` row exists), View > Zoom > Window and the plot area "Window" (AutoCAD uses 窓 there). A `menu Window` row would let the plain row be 窓. |
| 1861 | `Tangent` | `接線` | One row for both the object snap (接線) and the geometric constraint (AutoCAD uses 正接 for GCTANGENT). |
| 1971 | `View` | `ビュー` | Plain row is the tool set section with Orbit and Zoom Extents (the ribbon panel is ビュー); the menu row is 表示. |
| 2029 | `Zoom All` | `図面全体ズーム` | **Changed after review:** now `図面全体ズーム`. Glossary form. It is also the command label, where 図面全体 alone may read as vague. |
| 2035 | `Zoom Previous` | `前画面ズーム` | **Changed after review:** now `前画面ズーム`. Glossary form, same concern as 2029. |
| 2038 | `Zoom Window` | `窓ズーム` | **Changed after review:** now `窓ズーム`. Glossary form, same concern; 窓ズーム would be clearer as a command name. |
| 2030 / 2033 / 2036 | `Zoom Dynamic / Zoom Object / Zoom Realtime` | `ダイナミック ズーム / オブジェクト ズーム / リアルタイム ズーム` | Not in the glossary. Uses the "X ズーム" form because these are command labels, not menu leaves. |
| 1655 | `Specify feature location` | `点の位置を指定` | DIMORDINATE. I recall AutoCAD JP saying 点の位置を指定, but the alternative is フィーチャの位置を指定. |
| 1666 / 1678 / 1760 / 1773 / 1801 | `Specify first/second/third source/destination point` | `1 番目の移動元の点を指定 …` | ALIGN wording. AutoCAD JP may use 1 つ目の移動元の点 instead. |
| 1740 / 1741 | `Specify point on object for first/second tangent of circle` | `円の 1 つ目の接点となるオブジェクト上の点を指定` | Long. The exact AutoCAD JP CIRCLE TTR wording is uncertain. |
| 1676 / 1771 / 1800 | `...: _tan to` | `...: _tan 接線` | Echo of the deferred tangent snap. I believe JP AutoCAD shows "_tan 接線". |
| 1772 / 1801 / 1654 / 1716 / 1723 | `<use first point as displacement>`, `<continue>`, `<exit>` | `kept verbatim` | Rule 8 keeps `<...>` verbatim, so English shows inside Japanese prompts. |
| 1851 | `Synchronize Multiple-scale Positions` | `複数尺度の位置を同期` | ANNORESET. I am not sure of the AutoCAD JP menu name. |
| 1869 | `Text override` | `寸法値の優先` | Dimension property. I believe this matches the AutoCAD JP Properties palette. |
| 1897 | `Trace` | `太線` | TRACE entity type name (AutoCAD JP command 太線). |
| 1902 | `True Color` | `True Color` | Kept Latin as on the AutoCAD JP color dialog tab. Use トゥルー カラー if it should be translated. |
| 1885 | `To` | `変換先` | QuickCalc unit conversion "From/To". The partner row "From" (other chunk) should be 変換元. |
| 1977 | `Viewport Layer Freeze` | `ビューポート画層フリーズ` | VPLAYER command label. Not an established AutoCAD JP name. |
| 1730 / 1681 / 1810 | `Specify number of columns / gutter width / total width` | `段数を指定 / 段間隔の幅を指定 / 全体の幅を指定` | MTEXT column options, worded from the 段組み term. |
| 1816 | `Specify zoom scale factor` | `ズームの尺度を指定` | Uses the glossary's 尺度 for "scale factor". ズーム倍率 may read more naturally. |
| 1709 | `Specify minimum length of arc` | `最小円弧長を指定` | REVCLOUD. |
| 1922 / 1840 | `Union / Subtract` | `和 / 差` | Glossary terms. Very short as stand-alone tool labels. |
| 2052–2054 | `ΔLOC / −Bin / −LOC` | `kept` | Credits table column abbreviations, kept like ko/zh. They must match +LOC / +Bin in chunk 1. |
| 1823 | `Standard` | `Standard` | No UI use found in the source. Kept as the default style name. |
