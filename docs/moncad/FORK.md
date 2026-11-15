# MonCad fork notes (for maintainers)

MonCad is a fork of [CADCraft](https://github.com/storytold/cadcraft) (MIT OR Apache-2.0), taken from
upstream `main` at `7064a685` (2026-11-15, CADCraft 0.21.0-dev, after v0.20.1).

## What MonCad changes

| Area | Change | Files |
|---|---|---|
| Name | Binaries `moncad` / `moncad-cli`; window title, macOS app menu, About, update prompt, MCP `serverInfo` and messages say MonCad | `apps/cadcraft/Cargo.toml`, `apps/cadcraft-cli/Cargo.toml`, `apps/cadcraft/src/main.rs`, `apps/cadcraft/src/native_menu.rs`, `crates/ui-egui/src/{brand,dialogs,lib}.rs`, `crates/mcp/src/*` |
| Brand files | ArtCraft logos (`docs/brand/`) removed — their terms forbid them in forks; plain-text "based on CADCraft" credit in About, NOTICE, README | `NOTICE`, `ATTRIBUTION.md`, `crates/ui-egui/src/dialogs.rs` |
| Icon | New MonCad icon drawn by `packaging/moncad/make_icon.py`; upstream file names kept so packaging is unchanged | `assets/app-icon/*` |
| Settings folder | `%APPDATA%\MonCad`, `~/Library/Application Support/MonCad`, `~/.config/moncad` — never shared with an installed CADCraft | `crates/engine/src/settings_dir.rs` |
| Updates | Only MonCad's own repo (`brand::RELEASE_REPO` = `Mandakha1/moncad`); startup check off until it has a published release (`brand::UPDATES_AT_START`) — upstream's updater would replace MonCad with CADCraft | `crates/ui-egui/src/{brand,lib}.rs`, `apps/cadcraft/src/main.rs` |
| Mongolian UI | `mn` catalog (2,053 rows, nothing missing), registered in `i18n::LANGUAGES` | `crates/ui-egui/src/i18n.rs`, `crates/ui-egui/src/i18n/mn.tsv`, `docs/moncad/mn-glossary.md` |
| Mongolian keyboard | Commands typed with the Mongolian Cyrillic layout run (`лшиу` = LINE) | `crates/engine/src/keyboard.rs` |
| Metric | MEASUREINIT defaults to metric (mm, ISO-25); the headless MCP session starts metric | `crates/engine/src/settings.rs`, `crates/mcp/src/backend.rs` |
| Cyrillic text | Letters the stroke font lacks (Cyrillic) fall back to a proportional font (Arial, Segoe UI, DejaVu…) instead of a CJK face with full-width Cyrillic that doubled label widths | `crates/fonts/src/ttf.rs` (`CYRILLIC_FALLBACK`) |
| AI launch | `moncad-cli mcp --connect … --launch` opens the `moncad` binary next to the CLI on Windows and Linux too (upstream: macOS only) | `crates/mcp/src/backend.rs` |
| Windows installer | Per-user NSIS installer (Mongolian + English), Start menu "MonCad (AI)" entry, `AI-connect.txt` with ready commands | `packaging/moncad/windows/moncad.nsi`, `packaging/moncad/build-windows.sh` |
| macOS package | `packaging/macos/package.sh` builds `MonCad.app` (with `moncad-cli` inside) and `moncad-<ver>-macos-<arch>.dmg`; bundle id `com.monhydro.moncad`. Not yet: the DMG background still shows the old icon and its Finder layout (`dmg-layout.DS_Store`) points at a volume named CADCraft — regenerate with `packaging/macos/dmg/generate.py` | `packaging/macos/*` |
| Not yet rebranded | Linux and FreeBSD packages (`packaging/linux`, `packaging/freebsd`), the web build (`apps/cadcraft-web`) | |
| CI | Upstream workflows (storytold actions, signing secrets) parked in `packaging/upstream-ci/`; MonCad's own `.github/workflows/moncad.yml` | |

Internal names stay upstream's on purpose: crate names (`cadcraft-*`), package names (`-p cadcraft`),
`CADCRAFT_*` environment variables, the `cadcraft://` MCP resource URIs, the control-token file name,
the built-in plotter names "CADCraft PDF/PNG/SVG" (stored in drawings' page setups) and DXF
application names. Renaming them would break files and make every upstream merge conflict.

## Pulling upstream changes

```bash
git fetch upstream
git merge upstream/main
# conflicts are usually in the files listed above; keep MonCad's side for names, upstream's for logic
cargo test -p cadcraft-ui-egui --lib i18n            # catalogs, fonts, live switching
cargo test --workspace --lib -q -- --ignored --exact i18n::tests::translation_gaps --nocapture | grep -A50 '^mn:'
```

New upstream UI strings show in English in the Mongolian UI until rows are added to `mn.tsv`; the
gap report above lists them. Any new upstream string containing "CADCraft" should be rebranded in
the source and in every catalog (`grep -rn CADCraft crates/ui-egui/src`).

## Building

```bash
cargo run --release -p cadcraft                      # the desktop app (binary: moncad)
cargo build --release -p cadcraft -p cadcraft-cli    # target/release/moncad, moncad-cli
packaging/moncad/build-windows.sh                    # macOS/Linux host → Windows x64 setup.exe (mingw-w64 + NSIS)
```
