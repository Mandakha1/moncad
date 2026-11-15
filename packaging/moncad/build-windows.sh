#!/usr/bin/env bash
# Build the MonCad Windows x64 installer and portable zip from macOS or Linux.
#
#   brew install mingw-w64 makensis         # or apt install mingw-w64 nsis
#   rustup target add x86_64-pc-windows-gnu
#   packaging/moncad/build-windows.sh       # → dist/moncad-<ver>-windows-x64-setup.exe, -portable.zip
#
# On Windows itself the GitHub workflow (.github/workflows/moncad.yml) builds the same installer
# with the MSVC toolchain.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
TARGET="${TARGET:-x86_64-pc-windows-gnu}"
VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)"
DIST="$ROOT/dist"
# makensis crashes (std::bad_alloc) under the C locale on macOS, hence LC_ALL below.
command -v makensis >/dev/null || { echo "error: makensis (NSIS) not found" >&2; exit 1; }

# CADCRAFT_REQUIRE_WINRES: fail instead of shipping an exe without its icon and version info.
CADCRAFT_REQUIRE_WINRES=1 cargo build --release --locked -p cadcraft -p cadcraft-cli --target "$TARGET"
BIN="$ROOT/target/$TARGET/release"
mkdir -p "$DIST"

SETUP="$DIST/moncad-$VERSION-windows-x64-setup.exe"
LC_ALL=en_US.UTF-8 makensis -V2 -DROOT="$ROOT" -DVERSION="$VERSION" -DBINDIR="$BIN" -DOUTFILE="$SETUP" packaging/moncad/windows/moncad.nsi

STAGE="$ROOT/target/moncad-portable/moncad-$VERSION-windows-x64-portable"
rm -rf "$STAGE" && mkdir -p "$STAGE"
cp "$BIN/moncad.exe" "$BIN/moncad-cli.exe" LICENSE-APACHE LICENSE-MIT NOTICE "$STAGE/"
cp docs/moncad/INSTALL-mn.md "$STAGE/README-mn.md"
cp docs/moncad/AI-mn.md "$STAGE/AI-guide-mn.md"
ZIP="$DIST/moncad-$VERSION-windows-x64-portable.zip"
rm -f "$ZIP"
(cd "$(dirname "$STAGE")" && zip -qr "$ZIP" "$(basename "$STAGE")")

(cd "$DIST" && shasum -a 256 "$(basename "$SETUP")" "$(basename "$ZIP")" > "SHA256SUMS-windows.txt")
ls -la "$SETUP" "$ZIP"
cat "$DIST/SHA256SUMS-windows.txt"
