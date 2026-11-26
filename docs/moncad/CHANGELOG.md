# MonCad changelog

## 0.23.0 — 2026-11-26

- 日本語UIを追加 (Japanese interface): a complete `ja` catalog, 2,056 rows, nothing falls back to
  English, in the terms of the Japanese edition of AutoCAD (線分, 画層, 複写, 尺度, 寸法, 印刷…).
  **Auto** picks it on a Japanese Windows; **編集 → 表示言語** switches without a restart. Glyphs come
  from Yu Gothic (every Windows 10/11), Meiryo, Hiragino or Noto Sans CJK; without a Japanese font
  the interface stays English and says why. Rows for a native speaker to check: `docs/ja-review.md`.
- The Gmail sign-in window, the Account window and the browser page after sign-in are in Japanese
  in the Japanese interface; they link to the Japanese privacy notice.
- News cards: feed texts may carry `ja`; the Japanese interface shows only items with a Japanese
  title, so Mongolian courses are not offered to Japanese users.
- Windows installer: Japanese (preselected on a Japanese system) and `README-ja.md`.
- Japanese landing and download page: <https://jp.monhydro.com>.

## 0.22.0 — 2026-11-16

- Gmail sign-in on the first start; install registration at monhydro.com; docs say what is recorded.

## 0.21.0 — 2026-11-16

- First MonCad 2026 release: Mongolian interface, Mongolian keyboard-layout commands, metric
  default, Windows installer, news feed, update check against MonCad's own releases.
