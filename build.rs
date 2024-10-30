// build.rs

use glob::{glob, PatternError};

fn main() -> Result<(), PatternError> {
    for entry in glob("/opt/homebrew/Cellar/sdl2*/*/lib")? {
        match entry {
            Ok(path) => {
                let sdl2_path = path.to_str().unwrap();
                println!("cargo::rustc-link-search=native={sdl2_path}");
            }
            Err(_) => {}
        }
    }
    Ok(())
}
