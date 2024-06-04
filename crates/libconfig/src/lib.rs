use cxx::UniquePtr;
use std::ffi::CString;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum LibconfigError {
    #[error("invalid file")]
    Invalid,
}

pub struct Config {
    inner: UniquePtr<libconfig_sys::ffi::Config>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            inner: libconfig_sys::ffi::Config_ctor(),
        }
    }

    pub fn from_file(&mut self, path: &str) -> Result<(), LibconfigError> {
        let s = CString::new(path).expect("invalid file");
        unsafe {
            match self.inner.pin_mut().readFile(s.as_ptr()) {
                Ok(_) => Ok(()),
                Err(_) => Err(LibconfigError::Invalid),
            }
        }
    }

    pub fn lookup_bool(&mut self, path: &str) -> Option<bool> {
        let s = CString::new(path).expect("invalid settings");
        let mut tmp = false;
        unsafe {
            match self.inner.pin_mut().lookup_bool(s.as_ptr(), &mut tmp) {
                true => Some(tmp),
                false => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_on_non_existing_file() {
        let mut cfg = Config::new();
        assert_eq!(cfg.from_file("non_existing.cfg"), Err(LibconfigError::Invalid));
    }

    #[test]
    fn error_on_invalid_file() {
        let mut cfg = Config::new();
        assert_eq!(cfg.from_file("../input/invalid.cfg"), Err(LibconfigError::Invalid));
    }

    #[test]
    fn ok_on_valid_file() {
        let mut cfg = Config::new();
        assert_eq!(cfg.from_file("../input/test.cfg"), Ok(()));
    }
}
