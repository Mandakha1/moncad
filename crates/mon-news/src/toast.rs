//! Windows notifications (toasts) for an unpackaged desktop app. The installer registers the app's
//! notification id under `HKCU\Software\Classes\AppUserModelId\<id>` (name and icon), so Windows
//! shows the toast under the app's name. A toast with a link opens it when clicked (protocol
//! activation, which needs no COM server). A copy that was not installed (the portable zip) has no
//! registered id; Windows then refuses the toast and the card in the window is all there is.

use windows::Data::Xml::Dom::XmlDocument;
use windows::UI::Notifications::{ToastNotification, ToastNotificationManager};
use windows::core::HSTRING;

pub fn show(aumid: &str, app: &str, title: &str, body: &str, link: Option<&str>) {
    let (aumid, app, title, body, link) = (aumid.to_owned(), app.to_owned(), title.to_owned(), body.to_owned(), link.map(str::to_owned));
    // WinRT calls stay off the window's thread.
    let _ = std::thread::Builder::new().name("news-toast".into()).spawn(move || {
        if let Err(e) = send(&aumid, &app, &title, &body, link.as_deref()) {
            eprintln!("news: the Windows notification was not shown: {e}");
        }
    });
}

fn send(aumid: &str, app: &str, title: &str, body: &str, link: Option<&str>) -> windows::core::Result<()> {
    let launch = link.map(|l| format!(r#" activationType="protocol" launch="{}""#, escape(l))).unwrap_or_default();
    let xml = format!(
        r#"<toast{launch}><visual><binding template="ToastGeneric"><text>{}</text><text>{}</text><text placement="attribution">{}</text></binding></visual></toast>"#,
        escape(title),
        escape(body),
        escape(app)
    );
    let doc = XmlDocument::new()?;
    doc.LoadXml(&HSTRING::from(xml))?;
    let toast = ToastNotification::CreateToastNotification(&doc)?;
    ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(aumid))?.Show(&toast)
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;").replace('\'', "&apos;")
}
