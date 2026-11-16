//! Windows only: embed the app icon and version info into `moncad.exe`. Elsewhere a no-op.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/cadcraft.ico");
    println!("cargo:rerun-if-env-changed=CADCRAFT_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=CADCRAFT_BUILD_SHA");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    if std::path::Path::new("../../assets/app-icon/cadcraft.ico").exists() {
        res.set_icon("../../assets/app-icon/cadcraft.ico");
    }
    res.set("ProductName", "MonCad 2026")
        .set("FileDescription", "MonCad computer-aided design")
        .set("CompanyName", "MonCad")
        .set("LegalCopyright", "MonCad, based on CADCraft (c) 2026 ArtCraft Team and the CADCraft contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "moncad.exe")
        .set("InternalName", "moncad");
    if let Err(e) = res.compile() {
        if std::env::var_os("CADCRAFT_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=moncad.exe built without icon/version resources: {e}");
    }
}
