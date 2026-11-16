//! MonCad branding: the product name, its links, and where its releases come from.
//!
//! MonCad is based on CADCraft (MIT OR Apache-2.0) by the ArtCraft team. The ArtCraft marks are not
//! licensed for forks (docs/brand/LICENSE-brand.txt upstream), so MonCad uses its own name and icon
//! and credits CADCraft in plain text only.

/// The product name shown to people, without the year: folders, messages, the MCP server.
pub const NAME: &str = "MonCad";
/// The release year shown with the name. Next year change it, [`FULL_NAME`], and `ProductName`
/// in apps/cadcraft/build.rs; folders and executables stay without the year, so settings carry over.
pub const YEAR: &str = "2026";
/// What the window title, the start page, About and the installer show.
pub const FULL_NAME: &str = "MonCad 2026";
/// The id Windows shows MonCad's notifications under; the installer registers it.
pub const AUMID: &str = "MonHydro.MonCad";

/// MonCad's website (About window, macOS app menu).
pub const WEBSITE: &str = "https://cad.monhydro.com";
pub const WEBSITE_LABEL: &str = "cad.monhydro.com";

/// The project MonCad is based on.
pub const UPSTREAM: &str = "https://github.com/storytold/cadcraft";
pub const UPSTREAM_LABEL: &str = "github.com/storytold/cadcraft";

/// GitHub `owner/repo` of MonCad's own releases, for Help ▸ Check for Updates.
pub const RELEASE_REPO: &str = "Mandakha1/moncad";

/// MonCad's source code.
pub const SOURCE: &str = "https://github.com/Mandakha1/moncad";
pub const SOURCE_LABEL: &str = "github.com/Mandakha1/moncad";

/// Whether to look for a newer MonCad when the app starts (on since the first published release,
/// v0.21.0 "MonCad 2026"); Help ▸ Check for Updates works either way.
pub const UPDATES_AT_START: bool = true;

#[cfg(test)]
mod tests {
    #[test]
    fn the_full_name_is_the_name_and_the_year() {
        assert_eq!(super::FULL_NAME, format!("{} {}", super::NAME, super::YEAR));
    }
}
