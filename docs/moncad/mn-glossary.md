# MonCad — Mongolian (Cyrillic) UI glossary and rules

Audience: Mongolian civil / heating (HVAC) engineers and AutoCAD students. They know AutoCAD,
often in English or Russian. Write natural, short, modern Mongolian software language.
Polite neutral register ("та" implied, never "чи"). No Russian words where a common Mongolian
CAD word exists, but keep established loanwords (Блок, Сплайн, Масштаб, Штрих, Полилайн, Команд).

## Hard format rules (a test enforces these)
1. Keep every `{placeholder}` exactly (same names, same count). You may move it.
2. If the English ends with `...` the translation ends with `...`; if it ends with `…` it ends with `…`.
   If English does NOT end with an ellipsis, the translation must NOT end with one.
3. Mirror leading/trailing spaces of the English exactly ("  Cancel  " → "  Цуцлах  ", " or [" → " эсвэл [").
4. Never use a TAB or newline in a translation. `\n` written literally in English stays literally `\n`.
5. Keyboard shortcuts / symbols stay as they are: (⇧⌘Z), Ctrl+S, F3, ⌘, ⇧, ↵, %, °, etc.
6. Product/format/tech names stay: MonCad, DXF, DWG, PDF, SVG, PNG, AutoLISP, LSP, MCP, OLE, UCS, WCS,
   OpenGL, GPU, Git, GitHub, URL, ID, CTB, STB, SHX, TrueType, ISO, ANSI, JIS, DIN.
7. ALL-CAPS English → ALL-CAPS Mongolian (TOP → ДЭЭРЭЭС), unless it is a command name / variable
   name / keyword (LINE, PLINE, OSMODE, DIMSCALE, BYLAYER, BYBLOCK…) — those stay in English.
8. Command NAMES typed at the prompt and clickable option keywords inside `[...]` stay in English
   if they appear literally (e.g. "[Undo/Close]" text inside a key stays). Words outside brackets are translated.
9. `@plural` rows: ONE form only (Mongolian nouns don't inflect after a numeral): "{n} commit" → "{n} коммит".
10. `menu` context rows are top menu headings: View → Харах, Draw → Зурах, Dimension → Хэмжээс, Modify → Өөрчлөх.
11. Use proper Mongolian letters Ө ө Ү ү (U+04E8/04E9/04AE/04AF), never Latin O/Y or Ukrainian/Kazakh look-alikes.

## Prompts (command line)
- "Specify X" → "X заана уу"  (e.g. "Specify center point" → "Төв цэгийг заана уу")
- "Enter X" → "X оруулна уу"  ("Enter radius" → "Радиус оруулна уу")
- "Select X" → "X сонгоно уу"  ("Select objects" → "Объектуудыг сонгоно уу")
- A prompt ending in " or" → ends in " эсвэл" ("Specify Z scale factor or" → "Z масштабын коэффициент заана уу эсвэл").
- " or [" → " эсвэл ["
- Questions: "Fix any errors detected?" → "Илэрсэн алдааг засах уу?"

## Menus / general UI
File Файл · Edit Засах · View (menu) Харах · View (object/named view) Харагдац · Insert Оруулах ·
Format Формат · Tools Хэрэгсэл · Draw Зурах · Dimension Хэмжээс · Modify Өөрчлөх · Window Цонх ·
Help Тусламж · Parametric Параметрт · Express Express
New Шинэ · Open Нээх · Open Recent Сүүлд нээсэн · Save Хадгалах · Save As... Өөрөөр хадгалах... ·
Close Хаах · Quit Гарах · Plot/Print Хэвлэх · Export Экспортлох · Import Импортлох ·
Undo Буцаах · Redo Дахин хийх · Cut Хайчлах · Copy (clipboard & object) Хуулах · Paste Буулгах ·
Delete / Erase Устгах · Select All Бүгдийг сонгох · Find Хайх · Replace Солих ·
Options / Preferences / Settings Тохиргоо · OK OK · Cancel Цуцлах · Apply Хэрэглэх · Yes Тийм · No Үгүй ·
Reset Анхны байдалд · Default Үндсэн · Current Идэвхтэй · Properties Шинж чанар · Name Нэр ·
Description Тайлбар · Rename Нэр өөрчлөх · Show Харуулах · Hide Нуух · Lock Түгжих · Unlock Түгжээ тайлах ·
On Асаалттай / Асаах · Off Унтраалттай / Унтраах · Freeze Хөлдөөх · Thaw Гэсгээх · Browse... Хайх... ·
Command Команд · Command line Командын мөр · Option Сонголт · Error Алдаа · Warning Анхааруулга ·
Tooltip Зөвлөмж · Palette Самбар · Toolbar Хэрэгслийн мөр · Ribbon Тууз · Status bar Төлөвийн мөр ·
Tab Таб · Start page Эхлэл хуудас · Recent Сүүлийн · Contributors Хувь нэмэр оруулагчид ·
Interface Language Интерфейсийн хэл · Theme Загвар (өнгө) · Light/Dark Цайвар/Бараан · Shortcut Товчлол ·
Accessibility Хүртээмж · Update Шинэчлэл · Check for updates Шинэчлэл шалгах

## Objects
Line Шугам · Construction Line Туслах шугам · Ray Цацраг · Polyline Полилайн · 3D Polyline 3D полилайн ·
LwPolyline LW полилайн · Circle Тойрог · Arc Нум · Ellipse Эллипс · Elliptical Arc Эллипс нум ·
Rectangle Тэгш өнцөгт · Polygon Олон өнцөгт · Point Цэг · Spline Сплайн · Helix Спираль · Donut Цагираг ·
Revision Cloud Засварын үүл · Wipeout Далдлагч · Region Муж · Boundary Хүрээ · Hatch Штрих ·
Gradient Градиент · Text Текст · Multiline Text / MText Олон мөрт текст · Single Line Text Нэг мөрт текст ·
Table Хүснэгт · Block Блок · Attribute Атрибут · Block Reference Блокийн оруулга · Xref Гадаад холбоос ·
Image / Raster Image Растер зураг · Viewport Харах цонх · Layout Хуудас · Model Модель ·
Model space Модель орон зай · Paper space Цаасан орон зай · Leader Заагч шугам · Multileader Олон заагч ·
Dimension (object) Хэмжээс · Linear Шугаман · Aligned Зэрэгцүүлсэн · Angular Өнцгийн · Radius Радиус ·
Diameter Диаметр · Arc Length Нумын урт · Ordinate Координатын · Baseline Суурь шугамаас · Continue Үргэлжлүүлэх ·
Tolerance Хүлцэл · Center Mark Төвийн тэмдэг · Solid (3D) Цул бие · 2D Solid 2D дүүргэлт · 3D Face 3D гадаргуу ·
Mesh Тор гадаргуу · Surface Гадаргуу · Box Хайрцаг · Cylinder Цилиндр · Cone Конус · Sphere Бөмбөрцөг ·
Torus Тороид · Wedge Шаантаг · Pyramid Пирамид · Object Объект · Entity Объект · Vertex Орой · Segment Хэрчим ·
Layer Давхарга · Linetype Шугамын төрөл · Lineweight Шугамын зузаан · Color Өнгө · Transparency Тунгалаг байдал ·
Plot Style Хэвлэлтийн хэв маяг · Text Style Текстийн хэв маяг · Dimension Style Хэмжээсийн хэв маяг ·
Style Хэв маяг · Units Нэгж · Drawing Зураг · Limits Хязгаар · Group Бүлэг · Selection Сонголт · Field Талбар

## Modify / 3D
Move Зөөх · Copy Хуулах · Rotate Эргүүлэх · Scale Масштаблах · Mirror Толин тусгал · Offset Зэрэгцээ хуулах ·
Trim Тайрах · Extend Сунгах · Stretch Сунгаж татах · Lengthen Уртасгах · Break Тасдах · Join Нэгтгэх ·
Fillet Булан дугуйлах · Chamfer Ташуу огтлох · Explode Задлах · Array Массив · Rectangular Array Тэгш өнцөгт массив ·
Polar Array Тойрог массив · Path Array Замын массив · Align Тэгшлэх · Match Properties Шинж чанар хуулах ·
Grip Бариул · Purge Цэвэрлэх · Audit Шалгах · Recover Сэргээх · Divide Хуваах · Measure Хэмжих ·
Extrude Шахаж өргөх · Revolve Эргүүлж үүсгэх · Sweep Замаар үүсгэх · Loft Огтлолоор үүсгэх ·
Union Нэгтгэх · Subtract Хасах · Intersect Огтлолцол · Thickness Зузаан · Elevation Өндөршил

## View / drafting aids
Zoom Томруулах · Zoom In Томруулах · Zoom Out Жижигрүүлэх · Zoom Extents Бүгдийг харуулах ·
Pan Гүйлгэх · Regenerate Дахин үүсгэх · Orbit Тойруулан харах · Visual Style Харагдацын хэв маяг ·
2D Wireframe 2D утсан хүрээ · Wireframe Утсан хүрээ · Hidden Далд шугамтай · Shaded Сүүдэртэй · Realistic Бодит ·
Top Дээрээс · Bottom Доороос · Left Зүүнээс · Right Баруунаас · Front Урдаас · Back Араас ·
Isometric Изометр · ViewCube Харах куб · Snap Барилт · Grid Тор · Ortho Орто · Polar Tracking Туйлын мөрдөлт ·
Object Snap Объектын барилт · Object Snap Tracking Объектын барилтын мөрдөлт · Endpoint Төгсгөл ·
Midpoint Дунд цэг · Center Төв · Node Зангилаа · Quadrant Квадрант · Intersection Огтлолцол ·
Extension Үргэлжлэл · Insertion Оруулах цэг · Perpendicular Перпендикуляр · Tangent Шүргэгч ·
Nearest Хамгийн ойр · Apparent Intersection Харагдах огтлолцол · Parallel Параллель ·
Dynamic Input Динамик оруулга · UCS UCS · Coordinates Координат · Angle Өнцөг · Distance Зай ·
Length Урт · Width Өргөн · Height Өндөр · Area Талбай · Perimeter Периметр · Base point Суурь цэг ·
Start point Эхлэх цэг · End point Төгсгөлийн цэг · Scale factor Масштабын коэффициент

## Parametric
Constraint Хязгаарлалт · Geometric Геометрийн · Dimensional Хэмжээсийн · Coincident Давхцах ·
Collinear Нэг шулуун дээр · Concentric Нэг төвтэй · Fix Бэхлэх · Horizontal Хэвтээ · Vertical Босоо ·
Equal Тэнцүү · Symmetric Тэгш хэмтэй · Smooth Гөлгөр · Parameter Параметр · Expression Илэрхийлэл
