//! MonCad branding: the product name, its links, and where its releases come from.
//!
//! MonCad is based on CADCraft (MIT OR Apache-2.0) by the ArtCraft team. The ArtCraft marks are not
//! licensed for forks (docs/brand/LICENSE-brand.txt upstream), so MonCad uses its own name and icon
//! and credits CADCraft in plain text only.

/// The product name shown to people.
pub const NAME: &str = "MonCad";

/// MonCad's website (About window, macOS app menu).
pub const WEBSITE: &str = "https://cad.monhydro.com";
pub const WEBSITE_LABEL: &str = "cad.monhydro.com";

/// The project MonCad is based on.
pub const UPSTREAM: &str = "https://github.com/storytold/cadcraft";
pub const UPSTREAM_LABEL: &str = "github.com/storytold/cadcraft";

/// GitHub `owner/repo` of MonCad's own releases, for Help ▸ Check for Updates. Set this to the
/// published MonCad repository before the first release; until then
/// [`UPDATES_AT_START`] stays false so nothing is checked at startup.
pub const RELEASE_REPO: &str = "moncad/moncad";

/// Whether to look for a newer MonCad when the app starts. Off until [`RELEASE_REPO`] has releases.
pub const UPDATES_AT_START: bool = false;
