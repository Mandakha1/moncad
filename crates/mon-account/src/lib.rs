//! Gmail sign-in and install registration for MonCad, Mon3D and MonBIM.
//!
//! The owner wants to know who uses the programs, on what computers, and to be able to email
//! them news and courses. So every install signs in once with a Google (Gmail) account at
//! monhydro.com:
//!
//! * Each install gets its own code (`MC-…` for MonCad, `M3-…` for Mon3D, `MB-…` for MonBIM),
//!   kept in `account.json` in the app's settings folder and shown in Help → Account.
//! * On every start the app reports that code, its version and the computer to
//!   `https://monhydro.com/api/apps/ping`: the Windows edition and build, maker and model,
//!   processor, memory, screen size, interface language, and a hash of the Windows machine id
//!   (it tells a reinstall on the same PC from a new PC). Never files, drawings or anything typed.
//! * Until the install is signed in, a window covers the app. "Sign in with Gmail" opens the
//!   browser at `monhydro.com/apps/login`, where the person signs in with Google and chooses
//!   whether to get news and course emails (they can unsubscribe from every email). The page hands
//!   the result back through `http://127.0.0.1:<port>/cb`, a one-off local listener that only
//!   accepts the random `state` it was started with.
//! * Signed in once, the app works offline. If the app cannot reach monhydro.com at all, the
//!   window offers "Later" after a failed retry, so an outage never locks anyone out.
//!
//! `MON_ACCOUNT=0` turns all of it off (development, tests). `MON_ACCOUNT_API=<origin>` points it
//! at another server (a local test server) and prints the sign-in page's address;
//! `MON_ACCOUNT_OPEN=0` keeps it from opening the browser (automated tests).

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// The accounts server: monhydro.com, the same Google accounts as the website and SonorAi.
pub const API: &str = "https://monhydro.com";
/// How long a started sign-in waits for the browser.
pub const LOGIN_WAIT: Duration = Duration::from_secs(15 * 60);
/// Where the privacy note lives.
pub const PRIVACY_URL: &str = "https://monhydro.com/legal.html#apps";

/// An HTTPS GET returning the body, or why it failed (the app's own client, as for the news feed).
pub type Fetch = Arc<dyn Fn(&str) -> Result<Vec<u8>, String> + Send + Sync>;

/// How an app uses its account.
pub struct Config {
    /// `moncad`, `mon3d` or `monbim`.
    pub product: &'static str,
    /// The name shown in the window ("Mon3D 2026").
    pub app_name: &'static str,
    /// The app's version ("0.43.1").
    pub version: &'static str,
    /// The settings folder for `account.json`; `None` keeps it in memory only.
    pub store_dir: Option<PathBuf>,
    pub fetch: Option<Fetch>,
}

/// What `account.json` holds.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct State {
    /// This install's code, made on first start.
    pub install: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub name: String,
    /// The monhydro.com app token (a signed JWT); kept for later server calls.
    #[serde(default)]
    pub token: String,
    /// Unix seconds.
    #[serde(default)]
    pub signed_in_at: u64,
}

impl State {
    pub const FILE: &'static str = "account.json";

    fn load(dir: Option<&PathBuf>, product: &str) -> State {
        let mut state = dir
            .and_then(|d| std::fs::read(d.join(Self::FILE)).ok())
            .and_then(|bytes| serde_json::from_slice::<State>(&bytes).ok())
            .unwrap_or_default();
        if !valid_install(&state.install) {
            state.install = new_install_id(product);
            state.save(dir);
        }
        state
    }

    fn save(&self, dir: Option<&PathBuf>) {
        let Some(dir) = dir else { return };
        let _ = std::fs::create_dir_all(dir);
        if let Ok(text) = serde_json::to_vec_pretty(self) {
            let _ = std::fs::write(dir.join(Self::FILE), text);
        }
    }
}

/// The computer, as reported with each start.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Device {
    /// "Windows 11 Pro 24H2 (26100)".
    pub os: String,
    /// "LENOVO 21AH" (maker and model).
    pub model: String,
    pub cpu: String,
    /// Installed memory, whole GB (0: not read; reading it needs unsafe code, which the
    /// workspaces forbid).
    pub ram_gb: u32,
    /// 16 hex digits: a hash of the Windows machine id; empty where there is none.
    pub machine: String,
}

#[derive(Debug, Default, Deserialize)]
struct Pong {
    #[serde(default)]
    ok: bool,
}

struct Callback {
    token: String,
    email: String,
    name: String,
}

struct Waiting {
    rx: mpsc::Receiver<Result<Callback, String>>,
    url: String,
    cancel: Arc<AtomicBool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Server {
    Unknown,
    Reachable,
    /// The app's own request failed this many times.
    Unreachable(u32),
}

pub struct Account {
    cfg: Config,
    api: String,
    enabled: bool,
    state: State,
    ping_rx: Option<mpsc::Receiver<Result<Pong, String>>>,
    pinged: bool,
    server: Server,
    login: Option<Waiting>,
    error: Option<String>,
    /// "Later" was chosen while monhydro.com was unreachable (this run only).
    later: bool,
    /// The Account window (Help → Account) is open.
    pub panel_open: bool,
}

impl Account {
    pub fn new(cfg: Config) -> Account {
        let enabled = cfg.fetch.is_some() && std::env::var("MON_ACCOUNT").map_or(true, |v| v != "0");
        let api = std::env::var("MON_ACCOUNT_API").ok().filter(|v| v.starts_with("http")).unwrap_or_else(|| API.to_string());
        let state = State::load(cfg.store_dir.as_ref(), cfg.product);
        Account { cfg, api, enabled, state, ping_rx: None, pinged: false, server: Server::Unknown, login: None, error: None, later: false, panel_open: false }
    }

    pub fn install_id(&self) -> &str {
        &self.state.install
    }

    pub fn email(&self) -> Option<&str> {
        (!self.state.email.is_empty()).then_some(self.state.email.as_str())
    }

    pub fn signed_in(&self) -> bool {
        !self.state.email.is_empty()
    }

    /// True while the sign-in window covers the app.
    pub fn blocks(&self) -> bool {
        self.enabled && !self.signed_in() && !self.later
    }

    pub fn sign_out(&mut self) {
        let install = std::mem::take(&mut self.state.install);
        self.state = State { install, ..State::default() };
        self.state.save(self.cfg.store_dir.as_ref());
        self.later = false;
    }

    /// Call once per frame: reports the start, waits for the browser, and draws the sign-in
    /// window (while not signed in) and the Account window. `lang` is the interface language id.
    pub fn show(&mut self, ctx: &egui::Context, lang: &str) {
        if !self.enabled {
            return;
        }
        if !self.pinged {
            self.pinged = true;
            self.ping(ctx, lang);
        }
        self.poll(ctx);
        let mn = lang.starts_with("mn");
        if self.blocks() {
            self.gate(ctx, lang, mn);
        }
        if self.panel_open {
            self.panel(ctx, mn);
        }
    }

    fn ping(&mut self, ctx: &egui::Context, lang: &str) {
        let Some(fetch) = self.cfg.fetch.clone() else { return };
        let (w, h) = ctx.input(|i| {
            let ppp = i.pixels_per_point;
            i.viewport().monitor_size.map_or((0, 0), |s| ((s.x * ppp).round() as u32, (s.y * ppp).round() as u32))
        });
        let (tx, rx) = mpsc::channel();
        self.ping_rx = Some(rx);
        let base = format!(
            "{}/api/apps/ping?i={}&p={}&v={}&scr={}x{}&lang={}&u={}",
            self.api,
            enc(&self.state.install),
            enc(self.cfg.product),
            enc(self.cfg.version),
            w,
            h,
            enc(lang),
            u8::from(self.signed_in())
        );
        let ctx = ctx.clone();
        let spawned = std::thread::Builder::new().name("mon-account-ping".into()).spawn(move || {
            let d = device();
            let url = format!("{base}&os={}&dev={}&cpu={}&ram={}&mh={}", enc(&d.os), enc(&d.model), enc(&d.cpu), d.ram_gb, enc(&d.machine));
            let result = fetch(&url).and_then(|body| serde_json::from_slice::<Pong>(&body).map_err(|e| e.to_string()));
            let _ = tx.send(result);
            ctx.request_repaint();
        });
        if spawned.is_err() {
            self.ping_rx = None;
        }
    }

    fn poll(&mut self, ctx: &egui::Context) {
        if let Some(rx) = &self.ping_rx {
            match rx.try_recv() {
                Ok(Ok(pong)) => {
                    self.server = if pong.ok { Server::Reachable } else { Server::Unreachable(self.failures() + 1) };
                    self.ping_rx = None;
                }
                Ok(Err(_)) | Err(mpsc::TryRecvError::Disconnected) => {
                    self.server = Server::Unreachable(self.failures() + 1);
                    self.ping_rx = None;
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
        if let Some(wait) = &self.login {
            match wait.rx.try_recv() {
                Ok(Ok(cb)) => {
                    self.state.email = cb.email;
                    self.state.name = cb.name;
                    self.state.token = cb.token;
                    self.state.signed_in_at = now_secs();
                    self.state.save(self.cfg.store_dir.as_ref());
                    self.login = None;
                    self.error = None;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                }
                Ok(Err(e)) => {
                    self.error = Some(e);
                    self.login = None;
                }
                Err(mpsc::TryRecvError::Disconnected) => self.login = None,
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
    }

    fn failures(&self) -> u32 {
        match self.server {
            Server::Unreachable(n) => n,
            _ => 0,
        }
    }

    fn start_login(&mut self, ctx: &egui::Context, lang: &str) {
        if let Some(old) = self.login.take() {
            old.cancel.store(true, Ordering::Relaxed);
        }
        self.error = None;
        let nonce = random_token();
        let cancel = Arc::new(AtomicBool::new(false));
        match listen(nonce.clone(), lang.starts_with("mn"), self.cfg.app_name, cancel.clone(), ctx.clone()) {
            Ok((port, rx)) => {
                let url = format!(
                    "{}/apps/login?port={port}&state={}&product={}&install={}&v={}&lang={}",
                    self.api,
                    enc(&nonce),
                    enc(self.cfg.product),
                    enc(&self.state.install),
                    enc(self.cfg.version),
                    enc(if lang.starts_with("mn") { "mn" } else { "en" })
                );
                if std::env::var("MON_ACCOUNT_OPEN").map_or(true, |v| v != "0") {
                    ctx.open_url(egui::OpenUrl::new_tab(&url));
                }
                if std::env::var_os("MON_ACCOUNT_API").is_some() {
                    eprintln!("mon-account: sign-in page {url}");
                }
                self.login = Some(Waiting { rx, url, cancel });
            }
            Err(e) => self.error = Some(e),
        }
    }

    fn gate(&mut self, ctx: &egui::Context, lang: &str, mn: bool) {
        let t = |m: &'static str, e: &'static str| if mn { m } else { e };
        let mut start = false;
        let mut retry = false;
        let mut later = false;
        egui::Modal::new(egui::Id::NULL.with("mon_account_gate")).show(ctx, |ui| {
            ui.set_width(400.0);
            ui.vertical_centered(|ui| {
                ui.add_space(6.0);
                ui.heading(if mn { format!("{}-д тавтай морил", self.cfg.app_name) } else { format!("Welcome to {}", self.cfg.app_name) });
                ui.add_space(8.0);
                ui.label(t(
                    "Үргэлжлүүлэхийн тулд Gmail хаягаараа нэг удаа нэвтэрнэ үү. Програм үнэгүй, нэвтэрсний дараа интернэтгүй ч ажиллана.",
                    "Sign in once with your Gmail address to continue. The program is free and works offline after that.",
                ));
                ui.add_space(14.0);
                let waiting = self.login.is_some();
                let label = if waiting { t("Дахин нээх", "Open again") } else { t("Gmail-ээр нэвтрэх", "Sign in with Gmail") };
                let primary = egui::Button::new(egui::RichText::new(label).strong().color(egui::Color32::WHITE)).fill(ui.visuals().selection.bg_fill);
                if ui.add_sized([260.0, 38.0], primary).clicked() {
                    start = true;
                }
                if let Some(wait) = &self.login {
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(t("Хөтөч дээр Google хаягаа сонгоод энд буцаж ирнэ үү…", "Choose your Google account in the browser, then come back here…"));
                    });
                    if ui.small_button(t("Хөтөч нээгдээгүй бол: холбоосыг хуулах", "Browser did not open? Copy the link")).clicked() {
                        ui.ctx().copy_text(wait.url.clone());
                    }
                }
                if let Some(e) = &self.error {
                    ui.add_space(6.0);
                    ui.colored_label(ui.visuals().error_fg_color, format!("{} {e}", t("Нэвтэрч чадсангүй:", "Sign-in failed:")));
                }
                if let Server::Unreachable(n) = self.server {
                    ui.add_space(8.0);
                    ui.colored_label(ui.visuals().warn_fg_color, t(
                        "monhydro.com-д холбогдож чадсангүй. Интернэтээ шалгаад дахин оролдоно уу.",
                        "Could not reach monhydro.com. Check the internet connection and try again.",
                    ));
                    ui.horizontal(|ui| {
                        if ui.button(t("Дахин оролдох", "Try again")).clicked() {
                            retry = true;
                        }
                        if n >= 2 && ui.button(t("Дараа нэвтрэх", "Sign in later")).clicked() {
                            later = true;
                        }
                    });
                }
                ui.add_space(14.0);
                ui.separator();
                ui.small(t(
                    "Бид таны имэйл, нэр, програмын хувилбар, Windows ба компьютерийн загварыг хадгална. Таны файл, зураг хэзээ ч илгээгдэхгүй.",
                    "We keep your email, name, the program version, Windows and the computer model. Your files and drawings are never sent.",
                ));
                ui.horizontal(|ui| {
                    ui.hyperlink_to(egui::RichText::new(t("Нууцлалын бодлого", "Privacy")).small(), PRIVACY_URL);
                    ui.small(format!("· {} {}", t("Суулгалтын код:", "Install code:"), self.state.install));
                });
            });
        });
        if start {
            self.start_login(ctx, lang);
        }
        if retry {
            self.ping(ctx, lang);
        }
        if later {
            self.later = true;
        }
    }

    fn panel(&mut self, ctx: &egui::Context, mn: bool) {
        let t = |m: &'static str, e: &'static str| if mn { m } else { e };
        let mut open = self.panel_open;
        let mut sign_out = false;
        egui::Window::new(t("Бүртгэл", "Account")).id(egui::Id::NULL.with("mon_account_panel")).open(&mut open).collapsible(false).resizable(false).show(ctx, |ui| {
            egui::Grid::new("mon_account_grid").num_columns(2).spacing([16.0, 6.0]).show(ui, |ui| {
                ui.label(t("Имэйл", "Email"));
                ui.label(if self.state.email.is_empty() { t("нэвтрээгүй", "not signed in") } else { self.state.email.as_str() });
                ui.end_row();
                if !self.state.name.is_empty() {
                    ui.label(t("Нэр", "Name"));
                    ui.label(&self.state.name);
                    ui.end_row();
                }
                ui.label(t("Суулгалтын код", "Install code"));
                ui.horizontal(|ui| {
                    ui.monospace(&self.state.install);
                    if ui.small_button(t("Хуулах", "Copy")).clicked() {
                        ui.ctx().copy_text(self.state.install.clone());
                    }
                });
                ui.end_row();
            });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if self.signed_in() && ui.button(t("Гарах", "Sign out")).clicked() {
                    sign_out = true;
                }
                ui.hyperlink_to(t("Нууцлалын бодлого", "Privacy"), PRIVACY_URL);
            });
        });
        if sign_out {
            self.sign_out();
        }
        self.panel_open = open;
    }
}

/// Waits on 127.0.0.1 for the page's redirect: `GET /cb?state=…&token=…&email=…&name=…`.
fn listen(nonce: String, mn: bool, app: &'static str, cancel: Arc<AtomicBool>, ctx: egui::Context) -> Result<(u16, mpsc::Receiver<Result<Callback, String>>), String> {
    let listener = TcpListener::bind(("127.0.0.1", 0)).map_err(|e| e.to_string())?;
    let port = listener.local_addr().map_err(|e| e.to_string())?.port();
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    let (tx, rx) = mpsc::channel();
    std::thread::Builder::new()
        .name("mon-account-login".into())
        .spawn(move || {
            let deadline = Instant::now() + LOGIN_WAIT;
            while Instant::now() < deadline && !cancel.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((mut stream, _)) => match read_callback(&mut stream, &nonce) {
                        Some(Ok(cb)) => {
                            respond(&mut stream, 200, &done_page(mn, app, true));
                            let _ = tx.send(Ok(cb));
                            ctx.request_repaint();
                            return;
                        }
                        Some(Err(e)) => respond(&mut stream, 400, &done_page(mn, app, false).replace("{e}", &html_escape(&e))),
                        None => respond(&mut stream, 404, ""),
                    },
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => std::thread::sleep(Duration::from_millis(150)),
                    Err(e) => {
                        let _ = tx.send(Err(e.to_string()));
                        ctx.request_repaint();
                        return;
                    }
                }
            }
            if !cancel.load(Ordering::Relaxed) {
                let _ = tx.send(Err(if mn { "хугацаа дууссан, дахин оролдоно уу".into() } else { "timed out, please try again".into() }));
                ctx.request_repaint();
            }
        })
        .map_err(|e| e.to_string())?;
    Ok((port, rx))
}

/// `None`: not the callback path (a favicon request, say).
fn read_callback(stream: &mut TcpStream, nonce: &str) -> Option<Result<Callback, String>> {
    let _ = stream.set_nonblocking(false);
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let mut buf = Vec::with_capacity(2048);
    let mut chunk = [0u8; 1024];
    while buf.len() < 16 * 1024 && !buf.windows(4).any(|w| w == b"\r\n\r\n") {
        match stream.read(&mut chunk) {
            Ok(0) | Err(_) => break,
            Ok(n) => buf.extend_from_slice(&chunk[..n]),
        }
    }
    let head = String::from_utf8_lossy(&buf);
    let target = head.lines().next()?.strip_prefix("GET ")?.split(' ').next()?.to_string();
    parse_callback(&target, nonce)
}

fn parse_callback(target: &str, nonce: &str) -> Option<Result<Callback, String>> {
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    if path != "/cb" {
        return None;
    }
    let get = |key: &str| query.split('&').filter_map(|kv| kv.split_once('=')).find(|(k, _)| *k == key).map(|(_, v)| dec(v)).unwrap_or_default();
    if get("state") != nonce || nonce.is_empty() {
        return Some(Err("state".into()));
    }
    let email = get("email");
    let token = get("token");
    if !email.contains('@') || token.is_empty() || email.len() > 254 {
        let e: String = get("error").chars().take(200).collect();
        return Some(Err(if e.trim().is_empty() { "no email".into() } else { e.trim().to_string() }));
    }
    Some(Ok(Callback { token, email, name: get("name").chars().take(120).collect() }))
}

fn respond(stream: &mut TcpStream, status: u16, body: &str) {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        _ => "Not Found",
    };
    let _ = write!(
        stream,
        "HTTP/1.1 {status} {reason}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.flush();
}

fn done_page(mn: bool, app: &str, ok: bool) -> String {
    let (title, text) = match (ok, mn) {
        (true, true) => ("Нэвтэрлээ", format!("{app} руугаа буцна уу. Энэ хуудсыг хааж болно.")),
        (true, false) => ("Signed in", format!("Go back to {app}. You can close this page.")),
        (false, true) => ("Нэвтэрч чадсангүй", "Програм дээрээ «Gmail-ээр нэвтрэх»-ийг дахин дарна уу. ({e})".into()),
        (false, false) => ("Sign-in failed", "Press «Sign in with Gmail» in the program again. ({e})".into()),
    };
    let mark = if ok { "✓" } else { "!" };
    format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{title}</title>\
<style>body{{margin:0;min-height:100vh;display:flex;align-items:center;justify-content:center;background:#f3f0e8;font-family:'Segoe UI',system-ui,sans-serif;color:#17191e}}\
.c{{background:#faf8f3;border:1px solid rgba(23,25,30,.3);border-radius:2px;padding:36px 32px;max-width:380px;text-align:center}}\
.m{{font-size:40px;line-height:1}}h1{{font-size:22px;margin:14px 0 8px}}p{{margin:0;color:#40444c;line-height:1.5}}</style></head>\
<body><div class=\"c\"><div class=\"m\">{mark}</div><h1>{title}</h1><p>{}</p></div></body></html>",
        html_escape(&text)
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Percent-encodes everything but unreserved characters.
pub fn enc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// Decodes `%XX` and `+`.
pub fn dec(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => out.push(b' '),
            b'%' if i + 2 < bytes.len() => match (hex(bytes[i + 1]), hex(bytes[i + 2])) {
                (Some(hi), Some(lo)) => {
                    out.push(hi * 16 + lo);
                    i += 2;
                }
                _ => out.push(b'%'),
            },
            b => out.push(b),
        }
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex(b: u8) -> Option<u8> {
    (b as char).to_digit(16).map(|d| d as u8)
}

fn now_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

/// 64 random bits from the standard library's per-process random hasher keys.
fn random_u64(salt: u64) -> u64 {
    use std::hash::{BuildHasher, Hasher};
    let mut h = std::collections::hash_map::RandomState::new().build_hasher();
    h.write_u64(salt);
    h.write_u128(SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_nanos()));
    h.write_u32(std::process::id());
    h.finish()
}

fn random_token() -> String {
    format!("{:016x}{:016x}", random_u64(1), random_u64(2))
}

const CROCKFORD: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// `MC-7F3K-92QD-X1ZB`: a product prefix and 60 random bits in Crockford base32.
pub fn new_install_id(product: &str) -> String {
    let prefix = match product {
        "moncad" => "MC",
        "mon3d" => "M3",
        "monbim" => "MB",
        _ => "MX",
    };
    let mut v = random_u64(3) ^ random_u64(4).rotate_left(17);
    let mut chars = Vec::with_capacity(12);
    for _ in 0..12 {
        chars.push(CROCKFORD[(v & 31) as usize] as char);
        v >>= 5;
    }
    let s: String = chars.into_iter().collect();
    format!("{prefix}-{}-{}-{}", &s[0..4], &s[4..8], &s[8..12])
}

fn valid_install(id: &str) -> bool {
    let parts: Vec<&str> = id.split('-').collect();
    parts.len() == 4 && parts[0].len() == 2 && parts[1..].iter().all(|p| p.len() == 4 && p.bytes().all(|b| CROCKFORD.contains(&b)))
}

/// FNV-1a, 64 bits: stable across Rust versions (the machine hash must not change with them).
#[cfg_attr(not(windows), allow(dead_code))]
fn fnv64(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in data {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

#[cfg(windows)]
pub fn device() -> Device {
    use winreg::RegKey;
    use winreg::enums::HKEY_LOCAL_MACHINE;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let reg = |key: &str, value: &str| -> String {
        hklm.open_subkey(key).and_then(|k| k.get_value::<String, _>(value)).map(|v| v.trim().to_string()).unwrap_or_default()
    };
    let squash = |s: String| s.split_whitespace().collect::<Vec<_>>().join(" ");

    const NT: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion";
    let build = reg(NT, "CurrentBuild");
    let mut product = reg(NT, "ProductName");
    if build.parse::<u32>().is_ok_and(|b| b >= 22000) {
        product = product.replace("Windows 10", "Windows 11");
    }
    let os = squash(format!("{product} {} ({build})", reg(NT, "DisplayVersion")));
    const BIOS: &str = r"HARDWARE\DESCRIPTION\System\BIOS";
    let model = squash(format!("{} {}", reg(BIOS, "SystemManufacturer"), reg(BIOS, "SystemProductName")));
    let cpu = squash(reg(r"HARDWARE\DESCRIPTION\System\CentralProcessor\0", "ProcessorNameString"));
    let guid = reg(r"SOFTWARE\Microsoft\Cryptography", "MachineGuid");
    let machine = if guid.is_empty() { String::new() } else { format!("{:016x}", fnv64(format!("mon-account:{guid}").as_bytes())) };
    Device { os, model, cpu, ram_gb: 0, machine }
}

#[cfg(not(windows))]
pub fn device() -> Device {
    Device { os: format!("{} {}", std::env::consts::OS, std::env::consts::ARCH), ..Device::default() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install_ids_have_the_product_prefix_and_differ() {
        let a = new_install_id("mon3d");
        let b = new_install_id("mon3d");
        assert!(a.starts_with("M3-") && valid_install(&a), "{a}");
        assert!(new_install_id("moncad").starts_with("MC-"));
        assert!(new_install_id("monbim").starts_with("MB-"));
        assert_ne!(a, b);
        assert!(!valid_install("M3-0000-0000"));
        assert!(!valid_install("M3-OOOO-0000-0000"), "O is not Crockford");
    }

    #[test]
    fn percent_coding_round_trips() {
        let s = "Батаа Б. <a@b.mn>&x=1";
        assert_eq!(dec(&enc(s)), s);
        assert_eq!(enc("a b/c"), "a%20b%2Fc");
        assert_eq!(dec("a+b%2"), "a b%2");
    }

    #[test]
    fn the_callback_needs_the_state_and_an_email() {
        let ok = parse_callback("/cb?state=n1&token=t.k.n&email=bat%40gmail.com&name=%D0%91%D0%B0%D1%82", "n1").unwrap().ok().unwrap();
        assert_eq!((ok.email.as_str(), ok.name.as_str(), ok.token.as_str()), ("bat@gmail.com", "Бат", "t.k.n"));
        assert!(parse_callback("/cb?state=other&token=t&email=a%40b.mn", "n1").unwrap().is_err());
        assert!(parse_callback("/cb?state=n1&error=cancelled", "n1").unwrap().is_err());
        assert!(parse_callback("/favicon.ico", "n1").is_none());
    }

    #[test]
    fn state_survives_a_round_trip_and_keeps_its_install_code() {
        let dir = std::env::temp_dir().join(format!("mon-account-test-{}", random_token()));
        let first = State::load(Some(&dir), "moncad");
        assert!(first.install.starts_with("MC-"));
        let again = State::load(Some(&dir), "moncad");
        assert_eq!(first.install, again.install);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_machine_hash_is_stable() {
        assert_eq!(fnv64(b"mon-account:abc"), fnv64(b"mon-account:abc"));
        assert_eq!(format!("{:016x}", fnv64(b"")), "cbf29ce484222325");
    }
}
