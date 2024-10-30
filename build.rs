// build.rs

// Frameworks for MAC-OS can be downloaded from the SDL github official page:
// https://github.com/orgs/libsdl-org/

use glob::{glob, PatternError};

fn main() -> Result<(), PatternError> {
    for entry in glob("/opt/homebrew/Cellar/sdl2*/*/lib")? {
        match entry {
            Ok(path) => {
                let sdl2_path = path.to_str().unwrap();
                println!("cargo::rustc-link-search=static={sdl2_path}");
            }
            Err(_) => {}
        }
    }
    Ok(())
}
