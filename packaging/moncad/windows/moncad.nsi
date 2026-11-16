; MonCad Windows installer (NSIS 3, Unicode). Per-user: no administrator rights needed.
;
;   makensis -DVERSION=0.21.0 -DBINDIR=<folder with moncad.exe and moncad-cli.exe> \
;            -DOUTFILE=<dist>/moncad-0.21.0-windows-x64-setup.exe packaging/moncad/windows/moncad.nsi
;
; Installs to %LOCALAPPDATA%\Programs\MonCad, adds a Start menu entry (and, if ticked, a desktop
; shortcut), registers an uninstaller under "Apps & features", and writes "AI-connect.txt" with the
; exact commands that connect Claude or ChatGPT to this copy of moncad-cli.exe.
; Silent install: moncad-<ver>-windows-x64-setup.exe /S   (add /DESKTOP=1 for the desktop shortcut)

Unicode true
SetCompressor /SOLID lzma
RequestExecutionLevel user

!ifndef VERSION
  !define VERSION "0.0.0"
!endif
!ifndef BINDIR
  !error "pass -DBINDIR=<folder with moncad.exe and moncad-cli.exe>"
!endif
!ifndef YEAR
  !define YEAR "2026"
!endif
!ifndef OUTFILE
  !define OUTFILE "moncad-${YEAR}-${VERSION}-windows-x64-setup.exe"
!endif
; makensis works in the script's folder, so ROOT (the repository) is best passed as an absolute path.
; Path separator for compile-time paths: makensis on Windows needs "\" in File patterns, makensis on
; macOS/Linux needs "/".
!ifdef NSIS_WIN32_MAKENSIS
  !define SEP "\"
!else
  !define SEP "/"
!endif
!ifndef ROOT
  !define ROOT "${__FILEDIR__}${SEP}..${SEP}..${SEP}.."
!endif
!define UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\MonCad"
; The id Windows notifications (news, courses, offers) are shown under; the app uses the same one.
!define AUMID "MonHydro.MonCad"
!define FULLNAME "MonCad ${YEAR}"

Name "${FULLNAME}"
OutFile "${OUTFILE}"
InstallDir "$LOCALAPPDATA\Programs\MonCad"
InstallDirRegKey HKCU "Software\MonCad" "InstallDir"
BrandingText "${FULLNAME} (${VERSION})"
VIProductVersion "0.0.0.0"
VIAddVersionKey /LANG=1033 "ProductName" "${FULLNAME}"
VIAddVersionKey /LANG=1033 "FileDescription" "${FULLNAME} setup"
VIAddVersionKey /LANG=1033 "ProductVersion" "${VERSION}"
VIAddVersionKey /LANG=1033 "FileVersion" "${VERSION}"
VIAddVersionKey /LANG=1033 "LegalCopyright" "MonCad, based on CADCraft by the ArtCraft team. MIT OR Apache-2.0."

!include "MUI2.nsh"
!include "FileFunc.nsh"

!define MUI_ICON "${ROOT}${SEP}assets${SEP}app-icon${SEP}cadcraft.ico"
!define MUI_UNICON "${ROOT}${SEP}assets${SEP}app-icon${SEP}cadcraft.ico"
!define MUI_ABORTWARNING
!define MUI_LANGDLL_ALLLANGUAGES
!define MUI_FINISHPAGE_RUN "$INSTDIR\moncad.exe"
!define MUI_FINISHPAGE_SHOWREADME "$INSTDIR\AI-connect.txt"
!define MUI_FINISHPAGE_SHOWREADME_NOTCHECKED
!define MUI_FINISHPAGE_SHOWREADME_TEXT "$(ShowAiText)"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "${ROOT}${SEP}LICENSE-APACHE"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "Mongolian"
!insertmacro MUI_LANGUAGE "English"

LangString SecApp ${LANG_MONGOLIAN} "${FULLNAME} програм"
LangString SecApp ${LANG_ENGLISH} "${FULLNAME} application"
LangString SecDesktop ${LANG_MONGOLIAN} "Ширээний компьютер дээр товчлол"
LangString SecDesktop ${LANG_ENGLISH} "Desktop shortcut"
LangString ShowAiText ${LANG_MONGOLIAN} "Claude / ChatGPT-тэй холбох заавар харах"
LangString ShowAiText ${LANG_ENGLISH} "Show how to connect Claude / ChatGPT"

Function .onInit
  ; Ask for the installer language (Mongolian or English). A silent install never asks: the jump
  ; goes to a label because MUI_LANGDLL_DISPLAY expands to many instructions (IfSilent +2 would
  ; skip only the first, and /S would wait on the dialog forever).
  IfSilent skip_language
  !insertmacro MUI_LANGDLL_DISPLAY
  skip_language:
FunctionEnd

Section "!$(SecApp)" SecMain
  SectionIn RO
  SetOutPath "$INSTDIR"
  File "${BINDIR}${SEP}moncad.exe"
  File "${BINDIR}${SEP}moncad-cli.exe"
  File /oname=moncad.ico "${ROOT}${SEP}assets${SEP}app-icon${SEP}cadcraft.ico"
  File "${ROOT}${SEP}LICENSE-APACHE"
  File "${ROOT}${SEP}LICENSE-MIT"
  File "${ROOT}${SEP}NOTICE"
  File /oname=README-mn.md "${ROOT}${SEP}docs${SEP}moncad${SEP}INSTALL-mn.md"
  File /oname=AI-guide-mn.md "${ROOT}${SEP}docs${SEP}moncad${SEP}AI-mn.md"

  ; Ready-to-paste commands with this install's real path.
  FileOpen $0 "$INSTDIR\AI-connect.txt" w
  FileWrite $0 "${FULLNAME} - AI connection / AI холболт$\r$\n$\r$\n"
  FileWrite $0 "Claude Code (terminal):$\r$\n"
  FileWrite $0 '  claude mcp add moncad -- "$INSTDIR\moncad-cli.exe" mcp --connect 127.0.0.1:7979 --launch$\r$\n$\r$\n'
  FileWrite $0 "Claude Desktop (claude_desktop_config.json):$\r$\n"
  FileWrite $0 '  { "mcpServers": { "moncad": { "command": "$INSTDIR\moncad-cli.exe", "args": ["mcp", "--connect", "127.0.0.1:7979", "--launch"] } } }$\r$\n'
  FileWrite $0 "  (JSON: write every \ in the path as \\)$\r$\n$\r$\n"
  FileWrite $0 "ChatGPT desktop app: Settings > MCP servers > Add server > STDIO$\r$\n"
  FileWrite $0 '  command: "$INSTDIR\moncad-cli.exe"   arguments: mcp --connect 127.0.0.1:7979 --launch$\r$\n$\r$\n'
  FileWrite $0 "Codex CLI:$\r$\n"
  FileWrite $0 '  codex mcp add moncad -- "$INSTDIR\moncad-cli.exe" mcp --connect 127.0.0.1:7979 --launch$\r$\n$\r$\n'
  FileWrite $0 "Without a window (headless, the AI draws and saves files): replace the arguments with just: mcp$\r$\n"
  FileClose $0

  ; Shortcuts carry the year; an older year's shortcuts are replaced.
  CreateDirectory "$SMPROGRAMS\MonCad"
  Delete "$SMPROGRAMS\MonCad\*.lnk"
  CreateShortcut "$SMPROGRAMS\MonCad\${FULLNAME}.lnk" "$INSTDIR\moncad.exe" "" "$INSTDIR\moncad.exe" 0
  CreateShortcut "$SMPROGRAMS\MonCad\${FULLNAME} (AI).lnk" "$INSTDIR\moncad.exe" "--control 7979" "$INSTDIR\moncad.exe" 0
  CreateShortcut "$SMPROGRAMS\MonCad\Uninstall ${FULLNAME}.lnk" "$INSTDIR\uninstall.exe"

  ; Windows notifications from MonCad show its name and icon (an unpackaged app registers its id).
  WriteRegStr HKCU "Software\Classes\AppUserModelId\${AUMID}" "DisplayName" "${FULLNAME}"
  WriteRegStr HKCU "Software\Classes\AppUserModelId\${AUMID}" "IconUri" "$INSTDIR\moncad.ico"

  WriteUninstaller "$INSTDIR\uninstall.exe"
  WriteRegStr HKCU "Software\MonCad" "InstallDir" "$INSTDIR"
  WriteRegStr HKCU "${UNINST_KEY}" "DisplayName" "${FULLNAME}"
  WriteRegStr HKCU "${UNINST_KEY}" "DisplayVersion" "${VERSION}"
  WriteRegStr HKCU "${UNINST_KEY}" "Publisher" "MonCad"
  WriteRegStr HKCU "${UNINST_KEY}" "DisplayIcon" "$INSTDIR\moncad.exe"
  WriteRegStr HKCU "${UNINST_KEY}" "InstallLocation" "$INSTDIR"
  WriteRegStr HKCU "${UNINST_KEY}" "UninstallString" '"$INSTDIR\uninstall.exe"'
  WriteRegStr HKCU "${UNINST_KEY}" "QuietUninstallString" '"$INSTDIR\uninstall.exe" /S'
  WriteRegDWORD HKCU "${UNINST_KEY}" "NoModify" 1
  WriteRegDWORD HKCU "${UNINST_KEY}" "NoRepair" 1
  ${GetSize} "$INSTDIR" "/S=0K" $0 $1 $2
  WriteRegDWORD HKCU "${UNINST_KEY}" "EstimatedSize" "$0"
SectionEnd

Section /o "$(SecDesktop)" SecDesk
  CreateShortcut "$DESKTOP\${FULLNAME}.lnk" "$INSTDIR\moncad.exe" "" "$INSTDIR\moncad.exe" 0
SectionEnd

Function .onInstSuccess
  ; /DESKTOP=1 on a silent install adds the desktop shortcut too.
  ${GetParameters} $0
  ${GetOptions} $0 "/DESKTOP=" $1
  StrCmp $1 "1" 0 +2
  CreateShortcut "$DESKTOP\${FULLNAME}.lnk" "$INSTDIR\moncad.exe" "" "$INSTDIR\moncad.exe" 0
FunctionEnd

Section "Uninstall"
  Delete "$INSTDIR\moncad.exe"
  Delete "$INSTDIR\moncad-cli.exe"
  Delete "$INSTDIR\moncad.ico"
  Delete "$INSTDIR\LICENSE-APACHE"
  Delete "$INSTDIR\LICENSE-MIT"
  Delete "$INSTDIR\NOTICE"
  Delete "$INSTDIR\README-mn.md"
  Delete "$INSTDIR\AI-guide-mn.md"
  Delete "$INSTDIR\AI-connect.txt"
  Delete "$INSTDIR\uninstall.exe"
  RMDir "$INSTDIR"
  Delete "$SMPROGRAMS\MonCad\*.lnk"
  RMDir "$SMPROGRAMS\MonCad"
  Delete "$DESKTOP\MonCad.lnk"
  Delete "$DESKTOP\MonCad *.lnk"
  DeleteRegKey HKCU "Software\Classes\AppUserModelId\${AUMID}"
  DeleteRegKey HKCU "${UNINST_KEY}"
  DeleteRegKey HKCU "Software\MonCad"
  ; Drawings and the settings folder (%APPDATA%\MonCad) are the user's and stay.
SectionEnd
