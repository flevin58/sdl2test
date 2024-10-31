// build.rs
//
// For explanation of println!("cargo::xxxx") commands read here:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html

// The SDL lib paths are added assuming a Mac-Os with SDL files installed with brew 🍺
// For all other cases this file is transparent (no output to the builder)
// Note: cross-compiling will NOT work.
// Warning: Currently tested on a Mac M1 mini. Will follow tests on Linux & Windows

use glob::glob;
use os_info;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let info = os_info::get();
    match info.os_type() {
        os_info::Type::Macos => homebrew_sdl(),
        _ => {}
    }
}

fn homebrew_sdl() {
    if let Ok(paths) = glob("/opt/homebrew/Cellar/sdl2*/*/lib") {
        for entry in paths {
            if let Ok(path) = entry {
                let sdl2_path = path.to_str().unwrap();
                println!("cargo::rustc-link-search={sdl2_path}");
            }
        }
        return;
    }
}
