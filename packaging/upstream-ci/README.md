# Upstream CADCraft workflows (parked)

These are CADCraft's GitHub Actions workflows as of the fork (2026-11-15). They call
`storytold/craft-common-actions` and need the ArtCraft team's signing secrets, so they cannot run in
MonCad's repository. They are kept here, unchanged, as a reference for porting steps (signing,
macOS DMG, Linux packages) into `.github/workflows/moncad.yml`.

`dependabot.yml` is upstream's Dependabot config, parked here on 2026-11-16 so Dependabot does not
open version-update PRs in MonCad's repository. MonCad takes dependency updates when it merges from
upstream. To turn Dependabot back on, move the file back to `.github/dependabot.yml`.
