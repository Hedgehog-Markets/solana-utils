use std::env;
use std::process::Command;

fn main() {
    let minor = match rustc_minor_version() {
        Some(minor) => minor,
        None => return,
    };

    if minor >= 77 {
        println!("cargo:rustc-check-cfg=cfg(target_os, values(\"solana\"))");
        println!("cargo:rustc-check-cfg=cfg(no_offset_of)");
    }

    if minor < 77 {
        println!("cargo:rustc-cfg=no_offset_of");
    }
}

fn rustc_minor_version() -> Option<u32> {
    let rustc = match env::var_os("RUSTC") {
        Some(rustc) => rustc,
        None => return None,
    };

    let output = match Command::new(rustc).arg("--version").output() {
        Ok(output) => output,
        Err(_) => return None,
    };

    let version = match std::str::from_utf8(&output.stdout) {
        Ok(version) => version,
        Err(_) => return None,
    };

    let mut pieces = version.split('.');
    if pieces.next() != Some("rustc 1") {
        return None;
    }

    let next = match pieces.next() {
        Some(next) => next,
        None => return None,
    };

    next.parse::<u32>().ok()
}
