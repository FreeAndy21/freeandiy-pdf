//! Windows only: embed version info (VERSIONINFO) into `freeandiy-pdf-cli.exe`, so Explorer's
//! Details tab, `(Get-Item freeandiy-pdf-cli.exe).VersionInfo` and inventory tools can read the version
//! and publisher, as they can for `pdfcraft.exe` (`apps/pdfcraft/build.rs`).
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `PDFCRAFT_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=PDFCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set("ProductName", "FreeANDIY PDF")
        .set("FileDescription", "FreeANDIY PDF command-line tool")
        .set("LegalCopyright", "Copyright (c) the FreeANDIY PDF contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "freeandiy-pdf-cli.exe")
        .set("InternalName", "freeandiy-pdf-cli");
    if let Err(e) = res.compile() {
        if std::env::var_os("PDFCRAFT_REQUIRE_WINRES").is_some() {
            println!("cargo::error=embedding Windows resources failed: {e}");
            return;
        }
        println!("cargo:warning=freeandiy-pdf-cli.exe built without version resources: {e}");
    }
}
