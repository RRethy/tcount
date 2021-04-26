use crate::error::Error;
use std::path::Path;

extern "C" {
    fn tree_sitter_rust() -> tree_sitter::Language;
}

pub fn language(path: impl AsRef<Path>) -> Result<tree_sitter::Language, Error> {
    match path.as_ref().extension() {
        Some(osstr) => {
            let ext = osstr.to_string_lossy();
            if ext == "rs" {
                Ok(unsafe { tree_sitter_rust() })
            } else {
                Err(Error::Unsupported(String::new()))
            }
        }
        None => Err(Error::Unsupported(String::new())),
    }
}
