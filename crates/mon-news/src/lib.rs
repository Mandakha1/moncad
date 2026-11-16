//! News, courses and offers for MonCad and MonSolid.
//!
//! The owner publishes one small JSON file (the feed, [`FEED_URL`]); every installed copy reads it
//! when it starts and every few hours while it stays open, and shows what is new: a card in the
//! corner of the window and, on Windows, a notification (toast) for the items marked `toast`.
//! Clicking an item opens its `https` link in the browser. Nothing is sent but the request for the
//! file itself: no id, no usage data. The person can turn the cards off (see [`Seen::muted`]), and
//! `MON_NEWS=0` turns the whole feature off. `MON_NEWS_FILE=<path to a feed.json>` reads a local
//! file instead of the published feed, to preview a feed before publishing it.
//!
//! The feed (`version` 1):
//!
//! ```json
//! {"version": 1, "items": [{
//!   "id": "2026-11-course",            // unique; shown once per id
//!   "products": ["moncad", "monsolid"], // empty or absent: every product
//!   "kind": "course",                   // news | course | ad | update
//!   "title": {"mn": "…", "en": "…"},
//!   "body": {"mn": "…", "en": "…"},
//!   "link": "https://cad.monhydro.com", // https only
//!   "link_label": {"mn": "Бүртгүүлэх", "en": "Sign up"},
//!   "start": "2026-11-16", "end": "2026-12-31",  // inclusive, UTC dates, optional
//!   "toast": true                       // also a Windows notification
//! }]}
//! ```

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::{Arc, mpsc};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[cfg(windows)]
mod toast;

/// Where the feed lives: a file in MonCad's public repository, which the owner edits on GitHub.
pub const FEED_URL: &str = "https://raw.githubusercontent.com/Mandakha1/moncad/main/news/feed.json";
/// Biggest feed read.
pub const MAX_FEED: usize = 256 * 1024;
/// How often an open app reads the feed again.
pub const REFRESH: Duration = Duration::from_secs(6 * 3600);
/// After a failed read, the next try.
pub const RETRY: Duration = Duration::from_secs(30 * 60);
/// The first read waits for the window to settle.
pub const FIRST_DELAY: Duration = Duration::from_secs(6);
/// Items considered per feed (the rest are ignored).
pub const MAX_ITEMS: usize = 30;

/// An HTTPS GET returning the body, or why it failed. The app passes its own client, so TLS and
/// proxies work as in the rest of the app.
pub type Fetch = Arc<dyn Fn(&str) -> Result<Vec<u8>, String> + Send + Sync>;

/// A text in Mongolian and English; either may be missing.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Text {
    #[serde(default)]
    pub mn: String,
    #[serde(default)]
    pub en: String,
}

impl Text {
    /// The text in `lang` (`mn` or anything else for English), else in the other language.
    pub fn get(&self, lang: &str) -> &str {
        let (first, second) = if lang == "mn" { (&self.mn, &self.en) } else { (&self.en, &self.mn) };
        if first.trim().is_empty() { second.trim() } else { first.trim() }
    }

    fn is_empty(&self) -> bool {
        self.mn.trim().is_empty() && self.en.trim().is_empty()
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    #[default]
    News,
    Course,
    Ad,
    Update,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Item {
    pub id: String,
    #[serde(default)]
    pub products: Vec<String>,
    #[serde(default)]
    pub kind: Kind,
    pub title: Text,
    #[serde(default)]
    pub body: Text,
    #[serde(default)]
    pub link: Option<String>,
    #[serde(default)]
    pub link_label: Text,
    #[serde(default)]
    pub start: Option<String>,
    #[serde(default)]
    pub end: Option<String>,
    #[serde(default)]
    pub toast: bool,
}

impl Item {
    /// The link, if it is one the app may open: `https://`, no spaces, not absurdly long.
    pub fn safe_link(&self) -> Option<&str> {
        self.link.as_deref().map(str::trim).filter(|l| is_safe_link(l))
    }

    fn shown_for(&self, product: &str, today: &str) -> bool {
        let product_ok = self.products.is_empty() || self.products.iter().any(|p| p.eq_ignore_ascii_case(product));
        let started = self.start.as_deref().is_none_or(|s| valid_date(s) && s <= today);
        let not_ended = self.end.as_deref().is_none_or(|e| valid_date(e) && today <= e);
        product_ok && started && not_ended && !self.id.trim().is_empty() && !self.title.is_empty()
    }
}

/// `MON_NEWS_FILE`: a local feed to preview instead of the published one.
fn preview_file() -> Option<PathBuf> {
    std::env::var_os("MON_NEWS_FILE").filter(|p| !p.is_empty()).map(PathBuf::from)
}

pub fn is_safe_link(link: &str) -> bool {
    link.starts_with("https://") && link.len() > "https://".len() && link.len() <= 2048 && !link.chars().any(|c| c.is_whitespace() || c.is_control())
}

fn valid_date(d: &str) -> bool {
    let b = d.as_bytes();
    b.len() == 10 && b[4] == b'-' && b[7] == b'-' && b.iter().enumerate().all(|(i, c)| i == 4 || i == 7 || c.is_ascii_digit())
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Feed {
    #[serde(default)]
    pub version: u32,
    #[serde(default)]
    pub items: Vec<Item>,
}

impl Feed {
    pub fn parse(bytes: &[u8]) -> Result<Feed, String> {
        if bytes.len() > MAX_FEED {
            return Err(format!("the feed is over {} KB", MAX_FEED / 1024));
        }
        let mut feed: Feed = serde_json::from_slice(bytes).map_err(|e| format!("the feed is not valid JSON: {e}"))?;
        feed.items.truncate(MAX_ITEMS);
        Ok(feed)
    }

    /// The items for `product` today, in the feed's order.
    pub fn current<'a>(&'a self, product: &str, today: &str) -> Vec<&'a Item> {
        self.items.iter().filter(|i| i.shown_for(product, today)).collect()
    }
}

/// Today's UTC date as `YYYY-MM-DD`.
pub fn today() -> String {
    let days = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs() / 86_400) as i64;
    date_of(days)
}

/// The civil date of a day count since 1970-01-01 (Howard Hinnant's algorithm).
fn date_of(days: i64) -> String {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}")
}

/// What this person has already seen, kept in `news.json` in the app's settings folder.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Seen {
    /// Items closed or opened: their card is not shown again.
    #[serde(default)]
    pub dismissed: BTreeSet<String>,
    /// Items already sent as a Windows notification.
    #[serde(default)]
    pub toasted: BTreeSet<String>,
    /// The person turned the cards and notifications off (the News window still lists items).
    #[serde(default)]
    pub muted: bool,
}

impl Seen {
    const FILE: &'static str = "news.json";
    const MAX_IDS: usize = 500;

    pub fn load(dir: Option<&PathBuf>) -> Seen {
        dir.and_then(|d| std::fs::read(d.join(Self::FILE)).ok()).and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default()
    }

    pub fn save(&self, dir: Option<&PathBuf>) {
        let Some(dir) = dir else { return };
        let _ = std::fs::create_dir_all(dir);
        if let Ok(text) = serde_json::to_vec_pretty(self) {
            let _ = std::fs::write(dir.join(Self::FILE), text);
        }
    }

    /// Forget ids the feed no longer has, so the file stays small.
    fn prune(&mut self, feed: &Feed) {
        let live: BTreeSet<&str> = feed.items.iter().map(|i| i.id.as_str()).collect();
        for set in [&mut self.dismissed, &mut self.toasted] {
            if set.len() > Self::MAX_IDS {
                set.retain(|id| live.contains(id.as_str()));
            }
        }
    }
}

/// How an app uses the feed.
pub struct Config {
    /// `moncad` or `monsolid`: which items are for this app.
    pub product: &'static str,
    /// The name shown on cards ("MonSolid 2026").
    pub app_name: &'static str,
    /// The Windows notification id the installer registers (`MonHydro.MonSolid`).
    pub aumid: &'static str,
    pub url: String,
    /// The settings folder for [`Seen`]; `None` keeps it in memory only.
    pub store_dir: Option<PathBuf>,
    pub fetch: Option<Fetch>,
}

pub struct News {
    cfg: Config,
    enabled: bool,
    seen: Seen,
    feed: Option<Feed>,
    rx: Option<mpsc::Receiver<Result<Feed, String>>>,
    created: Instant,
    next_fetch: Option<Instant>,
    error: Option<String>,
    /// The News window (all current items) is open.
    pub list_open: bool,
}

impl News {
    pub fn new(cfg: Config) -> News {
        let enabled = (cfg.fetch.is_some() || preview_file().is_some()) && std::env::var("MON_NEWS").map_or(true, |v| v != "0");
        let seen = Seen::load(cfg.store_dir.as_ref());
        News { cfg, enabled, seen, feed: None, rx: None, created: Instant::now(), next_fetch: None, error: None, list_open: false }
    }

    /// Current items, for a menu that shows how many there are.
    pub fn items(&self) -> Vec<&Item> {
        let today = today();
        self.feed.as_ref().map(|f| f.current(self.cfg.product, &today)).unwrap_or_default()
    }

    /// Items whose card has not been closed yet.
    pub fn unread(&self) -> usize {
        self.items().iter().filter(|i| !self.seen.dismissed.contains(&i.id)).count()
    }

    pub fn open_list(&mut self) {
        self.list_open = true;
        if self.feed.is_none() {
            self.next_fetch = Some(Instant::now());
        }
    }

    /// Call once per frame: reads the feed when it is due, sends new notifications and draws the
    /// card and the News window. `lang` is the interface language id (`mn` → Mongolian texts).
    pub fn show(&mut self, ctx: &egui::Context, lang: &str) {
        if !self.enabled {
            return;
        }
        self.poll(ctx);
        self.fetch_if_due(ctx);
        self.notify(lang);
        self.card(ctx, lang);
        self.list(ctx, lang);
    }

    fn poll(&mut self, ctx: &egui::Context) {
        let Some(rx) = &self.rx else { return };
        match rx.try_recv() {
            Ok(Ok(feed)) => {
                self.seen.prune(&feed);
                self.feed = Some(feed);
                self.error = None;
                self.rx = None;
                self.next_fetch = Some(Instant::now() + REFRESH);
                ctx.request_repaint();
            }
            Ok(Err(e)) => {
                self.error = Some(e);
                self.rx = None;
                self.next_fetch = Some(Instant::now() + RETRY);
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                self.rx = None;
                self.next_fetch = Some(Instant::now() + RETRY);
            }
        }
    }

    fn fetch_if_due(&mut self, ctx: &egui::Context) {
        if self.rx.is_some() {
            return;
        }
        let due = self.next_fetch.unwrap_or(self.created + FIRST_DELAY);
        let now = Instant::now();
        if now < due {
            ctx.request_repaint_after(due - now);
            return;
        }
        let fetch: Fetch = match (preview_file(), self.cfg.fetch.clone()) {
            (Some(path), _) => Arc::new(move |_: &str| std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))),
            (None, Some(fetch)) => fetch,
            (None, None) => return,
        };
        let (tx, rx) = mpsc::channel();
        let url = self.cfg.url.clone();
        let ctx = ctx.clone();
        let spawned = std::thread::Builder::new().name("news".into()).spawn(move || {
            let _ = tx.send(fetch(&url).and_then(|body| Feed::parse(&body)));
            ctx.request_repaint();
        });
        match spawned {
            Ok(_) => self.rx = Some(rx),
            Err(e) => {
                self.error = Some(e.to_string());
                self.next_fetch = Some(now + RETRY);
            }
        }
    }

    /// Windows notifications for new `toast` items, once each.
    fn notify(&mut self, lang: &str) {
        if self.seen.muted {
            return;
        }
        let fresh: Vec<Item> = self
            .items()
            .into_iter()
            .filter(|i| i.toast && !self.seen.toasted.contains(&i.id) && !self.seen.dismissed.contains(&i.id))
            .cloned()
            .collect();
        if fresh.is_empty() {
            return;
        }
        for item in &fresh {
            self.seen.toasted.insert(item.id.clone());
            #[cfg(windows)]
            toast::show(self.cfg.aumid, self.cfg.app_name, item.title.get(lang), item.body.get(lang), item.safe_link());
            #[cfg(not(windows))]
            let _ = (lang, self.cfg.aumid);
        }
        self.seen.save(self.cfg.store_dir.as_ref());
    }

    fn dismiss(&mut self, id: &str) {
        self.seen.dismissed.insert(id.to_owned());
        self.seen.save(self.cfg.store_dir.as_ref());
    }

    /// The card in the window's lower right corner: the first item not closed yet.
    fn card(&mut self, ctx: &egui::Context, lang: &str) {
        if self.seen.muted {
            return;
        }
        let Some(item) = self.items().into_iter().find(|i| !self.seen.dismissed.contains(&i.id)).cloned() else { return };
        let t = words(lang);
        let mut close = false;
        let mut mute = false;
        egui::Window::new(format!("{} · {}", kind_label(item.kind, lang), self.cfg.app_name))
            .id(egui::Id::unique("mon_news_card"))
            .anchor(egui::Align2::RIGHT_BOTTOM, [-16.0, -44.0])
            .collapsible(false)
            .resizable(false)
            .default_width(340.0)
            .show(ctx, |ui| {
                ui.set_max_width(340.0);
                item_body(ui, &item, lang);
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if let Some(link) = item.safe_link() {
                        let label = if item.link_label.is_empty() { t.more } else { item.link_label.get(lang) };
                        if ui.button(label).clicked() {
                            ctx.open_url(egui::OpenUrl::new_tab(link));
                            close = true;
                        }
                    }
                    if ui.button(t.close).clicked() {
                        close = true;
                    }
                    if ui.small_button(t.mute).on_hover_text(t.mute_tip).clicked() {
                        mute = true;
                    }
                });
            });
        if close {
            self.dismiss(&item.id);
        }
        if mute {
            self.seen.muted = true;
            self.seen.save(self.cfg.store_dir.as_ref());
        }
    }

    /// The News window: every current item, and the switch for cards and notifications.
    fn list(&mut self, ctx: &egui::Context, lang: &str) {
        if !self.list_open {
            return;
        }
        let t = words(lang);
        let items: Vec<Item> = self.items().into_iter().cloned().collect();
        let mut open = true;
        let mut shown = !self.seen.muted;
        egui::Window::new(t.news_title).id(egui::Id::unique("mon_news_list")).open(&mut open).default_width(420.0).show(ctx, |ui| {
            if items.is_empty() {
                ui.label(if self.rx.is_some() { t.loading } else if self.error.is_some() { t.offline } else { t.empty });
            }
            egui::ScrollArea::vertical().max_height(420.0).show(ui, |ui| {
                for item in &items {
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.small(kind_label(item.kind, lang));
                        item_body(ui, item, lang);
                        if let Some(link) = item.safe_link() {
                            let label = if item.link_label.is_empty() { t.more } else { item.link_label.get(lang) };
                            ui.hyperlink_to(label, link);
                        }
                    });
                }
            });
            ui.separator();
            ui.checkbox(&mut shown, t.show_switch);
            ui.small(t.privacy);
        });
        if shown == self.seen.muted {
            self.seen.muted = !shown;
            self.seen.save(self.cfg.store_dir.as_ref());
        }
        self.list_open = open;
    }
}

fn item_body(ui: &mut egui::Ui, item: &Item, lang: &str) {
    ui.add(egui::Label::new(egui::RichText::new(item.title.get(lang)).strong()).wrap());
    let body = item.body.get(lang);
    if !body.is_empty() {
        ui.add(egui::Label::new(body).wrap());
    }
}

/// The few words the feature needs, in its own two languages.
struct Words {
    news_title: &'static str,
    more: &'static str,
    close: &'static str,
    mute: &'static str,
    mute_tip: &'static str,
    show_switch: &'static str,
    privacy: &'static str,
    loading: &'static str,
    offline: &'static str,
    empty: &'static str,
}

fn words(lang: &str) -> Words {
    if lang == "mn" {
        Words {
            news_title: "Мэдээ, сургалт, санал",
            more: "Дэлгэрэнгүй",
            close: "Хаах",
            mute: "Дахиж бүү харуул",
            mute_tip: "Шинэ мэдээний карт, Windows мэдэгдлийг унтраана. Тусламж цэсний «Мэдээ»-нээс дахин асааж болно.",
            show_switch: "Шинэ мэдээ, сургалт, саналыг карт болон Windows мэдэгдлээр харуулах",
            privacy: "Мэдээний файлыг л татна; таны тухай ямар ч мэдээлэл илгээхгүй.",
            loading: "Ачаалж байна…",
            offline: "Мэдээг татаж чадсангүй (интернэт холболтоо шалгана уу).",
            empty: "Одоогоор шинэ мэдээ алга.",
        }
    } else {
        Words {
            news_title: "News, courses and offers",
            more: "Learn more",
            close: "Close",
            mute: "Don't show again",
            mute_tip: "Turns off news cards and Windows notifications. Turn them back on from News in the Help menu.",
            show_switch: "Show new news, courses and offers as cards and Windows notifications",
            privacy: "Only the news file is downloaded; nothing about you is sent.",
            loading: "Loading…",
            offline: "The news could not be loaded (check the internet connection).",
            empty: "No news right now.",
        }
    }
}

/// The label that says what an item is; an ad is always labelled as one.
pub fn kind_label(kind: Kind, lang: &str) -> &'static str {
    match (kind, lang == "mn") {
        (Kind::News, true) => "Мэдээ",
        (Kind::Course, true) => "Сургалт",
        (Kind::Ad, true) => "Зар",
        (Kind::Update, true) => "Шинэчлэл",
        (Kind::News, false) => "News",
        (Kind::Course, false) => "Course",
        (Kind::Ad, false) => "Ad",
        (Kind::Update, false) => "Update",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FEED: &str = r#"{"version": 1, "items": [
        {"id": "a", "products": ["moncad"], "kind": "course", "title": {"mn": "Сургалт", "en": "Course"},
         "link": "https://cad.monhydro.com", "start": "2026-11-01", "end": "2026-11-30", "toast": true},
        {"id": "b", "kind": "ad", "title": {"en": "Only English"}, "link": "http://insecure.example"},
        {"id": "c", "products": ["monsolid"], "title": {"mn": "Зөвхөн MonSolid"}},
        {"id": "", "title": {"mn": "id-гүй"}},
        {"id": "e", "title": {"mn": "Хугацаа буруу"}, "start": "16/11/2026"}
    ]}"#;

    #[test]
    fn items_are_chosen_by_product_and_date() {
        let feed = Feed::parse(FEED.as_bytes()).unwrap();
        let ids = |p: &str, d: &str| feed.current(p, d).iter().map(|i| i.id.as_str()).collect::<Vec<_>>();
        assert_eq!(ids("moncad", "2026-11-16"), ["a", "b"]);
        assert_eq!(ids("MonSolid", "2026-11-16"), ["b", "c"]);
        assert_eq!(ids("moncad", "2026-12-01"), ["b"], "after its end date");
        assert_eq!(ids("moncad", "2026-10-31"), ["b"], "before its start date");
    }

    #[test]
    fn texts_fall_back_to_the_other_language_and_links_must_be_https() {
        let feed = Feed::parse(FEED.as_bytes()).unwrap();
        assert_eq!(feed.items[0].title.get("mn"), "Сургалт");
        assert_eq!(feed.items[0].title.get("en"), "Course");
        assert_eq!(feed.items[1].title.get("mn"), "Only English");
        assert_eq!(feed.items[0].safe_link(), Some("https://cad.monhydro.com"));
        assert_eq!(feed.items[1].safe_link(), None);
        assert!(!is_safe_link("https://a b") && !is_safe_link("https://") && !is_safe_link("javascript:alert(1)"));
    }

    #[test]
    fn a_broken_or_huge_feed_is_an_error() {
        assert!(Feed::parse(b"not json").is_err());
        assert!(Feed::parse(&vec![b' '; MAX_FEED + 1]).is_err());
        assert_eq!(Feed::parse(br#"{"items": []}"#).unwrap().items.len(), 0);
    }

    #[test]
    fn dates_are_civil_utc_dates() {
        assert_eq!(date_of(0), "1970-01-01");
        assert_eq!(date_of(20_408), "2025-11-16");
        assert_eq!(date_of(20_773), "2026-11-16");
        assert_eq!(date_of(19_782), "2024-02-29");
        assert!(valid_date(&today()));
    }

    #[test]
    fn seen_items_survive_a_restart() {
        let dir = std::env::temp_dir().join(format!("mon-news-test-{}", std::process::id()));
        let mut seen = Seen::default();
        seen.dismissed.insert("a".into());
        seen.muted = true;
        seen.save(Some(&dir));
        assert_eq!(Seen::load(Some(&dir)), seen);
        assert_eq!(Seen::load(None), Seen::default());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn ads_are_labelled_as_ads() {
        assert_eq!(kind_label(Kind::Ad, "mn"), "Зар");
        assert_eq!(kind_label(Kind::Ad, "en"), "Ad");
    }
}
