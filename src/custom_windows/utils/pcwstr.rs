use std::path::{Path, PathBuf};
use windows::core::{HSTRING, PCWSTR};

pub fn from_path(path: &Path) -> PCWSTR {
    PCWSTR(HSTRING::from(path).as_ptr())
}

pub fn from_path_buf(path: PathBuf) -> PCWSTR {
    PCWSTR(HSTRING::from(path.as_path()).as_ptr())
}
