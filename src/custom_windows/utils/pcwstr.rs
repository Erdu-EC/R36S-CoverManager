use std::path::{Path, PathBuf};
use windows::core::{HSTRING, PCWSTR};

pub fn from_path(path: &Path) -> PCWSTR {
    PCWSTR(HSTRING::from(path).as_ptr())
}

