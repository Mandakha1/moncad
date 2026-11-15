# MonCad-ийг Claude болон ChatGPT-ээр удирдах

MonCad нь **MCP** (Model Context Protocol) сервертэй. Энэ нь Claude, ChatGPT зэрэг AI туслахад
MonCad-ийн командын мөрөнд бичих, зураг шалгах, зургийн PNG харах, DXF/PDF хадгалах эрх өгнө.
AI яг инженер шиг `LINE`, `CIRCLE`, `RECTANG`, `OFFSET`, `DIMLINEAR` командуудыг ашиглана.

Хоёр горим бий:

| Горим | Юу болох | Командын аргумент |
|---|---|---|
| **Цонхтой (live)** | AI нээлттэй MonCad цонхонд зурна, та шууд харна | `mcp --connect 127.0.0.1:7979 --launch` |
| **Цонхгүй (headless)** | AI санах ойд зураад DXF/PDF/PNG болгон хадгална | `mcp` |

`--launch` нь MonCad нээлттэй биш үед түүнийг 7979 портоор автоматаар нээнэ (Windows, macOS, Linux).
MonCad-ийг өөрөө нээх бол: Start цэс → **MonCad (AI)** эсвэл `moncad --control 7979`.

Доорх жишээнд `moncad-cli`-ийн бүтэн замыг ашиглана. Windows суулгагч энэ замтай бэлэн
командуудыг `%LOCALAPPDATA%\Programs\MonCad\AI-connect.txt` файлд бичдэг — тэндээс хуулж болно.

- Windows: `%LOCALAPPDATA%\Programs\MonCad\moncad-cli.exe`
- macOS: `/Applications/MonCad.app/Contents/MacOS/moncad-cli`

## 1. Claude Code (терминал)

```bash
claude mcp add moncad -- "C:\Users\<нэр>\AppData\Local\Programs\MonCad\moncad-cli.exe" mcp --connect 127.0.0.1:7979 --launch
```

Дараа нь Claude-д: «MonCad дээр 6×9 м хэмжээтэй 2 өрөөтэй байрны план зур, хананы зузаан 250 мм,
хэмжээс тавь» гэж бичнэ.

## 2. Claude Desktop

Settings → Developer → **Edit Config** (`claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "moncad": {
      "command": "C:\\Users\\<нэр>\\AppData\\Local\\Programs\\MonCad\\moncad-cli.exe",
      "args": ["mcp", "--connect", "127.0.0.1:7979", "--launch"]
    }
  }
}
```

JSON дотор `\` тэмдэг бүрийг `\\` гэж бичнэ. Хадгалаад Claude Desktop-ийг дахин асаана.

## 3. ChatGPT desktop апп

ChatGPT desktop апп нь Codex-тэй ижил MCP тохиргоог ашигладаг (OpenAI-ийн баримт бичгийн дагуу):

1. **Settings → MCP servers → Add server**
2. Name: `moncad`, transport: **STDIO**
3. Command: `moncad-cli.exe`-ийн бүтэн зам, arguments: `mcp --connect 127.0.0.1:7979 --launch`
4. **Save**, дараа нь **Restart**

Эсвэл `~/.codex/config.toml` файлд (ChatGPT desktop, Codex CLI, IDE өргөтгөл бүгд уншина):

```toml
[mcp_servers.moncad]
command = 'C:\Users\<нэр>\AppData\Local\Programs\MonCad\moncad-cli.exe'
args = ["mcp", "--connect", "127.0.0.1:7979", "--launch"]
```

Codex CLI:

```bash
codex mcp add moncad -- "C:\Users\<нэр>\AppData\Local\Programs\MonCad\moncad-cli.exe" mcp --connect 127.0.0.1:7979 --launch
```

> chatgpt.com вэб хувилбар таны компьютер дээрх програмыг ажиллуулж чадахгүй (зөвхөн интернэтээр
> хандах MCP сервер холбодог). MonCad-ийг desktop апп эсвэл Codex-оор холбоно.

## AI-д өгөх зөвлөмж (prompt)

- «Шинэ зураг нээ (мм). ХАНА, ХААЛГА, ЦОНХ, ХЭМЖЭЭС гэсэн давхарга үүсгэ.»
- «Халаалтын систем: шулуун шугам Т1 (улаан, 1-р давхарга), буцах шугам Т2 (цэнхэр), радиатор бүрийг
  1000×500 тэгш өнцөгтөөр зур, текстээр «Р-1», «Р-2» гэж бич.»
- «Зурсны дараа `render`-ээр харж, алдаа байвал засаад DXF болгож D:\Зураг\plan.dxf хадгал.»

AI ажлаа `inspect_drawing`, `query_entities`, `render` хэрэгслээр шалгадаг тул зураг зурсны дараа
«зургаа шалгаад харуул» гэж хүсэхэд л хангалттай.

## Аюулгүй байдал

- Control порт зөвхөн `127.0.0.1` (таны компьютер) дээр сонсдог бөгөөд хандах токеныг зөвхөн таны
  хэрэглэгчийн хавтаст хадгалдаг — өөр хүн сүлжээгээр холбогдож чадахгүй.
- AI файл хадгалах, нээх эрхтэй тул чухал зургаа нөөцлөөрэй.
