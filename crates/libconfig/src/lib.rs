use cxx::{let_cxx_string, UniquePtr};
use libconfig_sys::ffi::lookupValueI64;
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

    pub fn lookup_i32(&mut self, path: &str) -> Option<i32> {
        let s = CString::new(path).expect("invalid settings");
        let mut tmp: i32 = 0;
        unsafe {
            match self.inner.pin_mut().lookup_i32(s.as_ptr(), &mut tmp) {
                true => Some(tmp),
                false => None,
            }
        }
    }

    pub fn lookup_i64(&mut self, path: &str) -> Option<i64> {
        let s = CString::new(path).expect("invalid settings");
        let mut tmp: i64 = 0;
        unsafe {
            match lookupValueI64(&self.inner, s.as_ptr(), &mut tmp) {
                true => Some(tmp),
                false => None,
            }
        }
    }

    pub fn lookup_f32(&mut self, path: &str) -> Option<f32> {
        let s = CString::new(path).expect("invalid settings");
        let mut tmp: f32 = 0.0;
        unsafe {
            match self.inner.pin_mut().lookup_f32(s.as_ptr(), &mut tmp) {
                true => Some(tmp),
                false => None,
            }
        }
    }

    pub fn lookup_f64(&mut self, path: &str) -> Option<f64> {
        let s = CString::new(path).expect("invalid settings");
        let mut tmp: f64 = 0.0;
        unsafe {
            match self.inner.pin_mut().lookup_f64(s.as_ptr(), &mut tmp) {
                true => Some(tmp),
                false => None,
            }
        }
    }

    pub fn lookup_string(&mut self, path: &str) -> Option<String> {
        let s = CString::new(path).expect("invalid settings");
        let_cxx_string!(tmp = "");
        unsafe {
            match self.inner.pin_mut().lookup_string(s.as_ptr(), tmp.as_mut()) {
                true => Some(tmp.to_string()),
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
        assert_eq!(
            cfg.from_file("non_existing.cfg"),
            Err(LibconfigError::Invalid)
        );
    }

    #[test]
    fn error_on_invalid_file() {
        let mut cfg = Config::new();
        assert_eq!(
            cfg.from_file("../input/invalid.cfg"),
            Err(LibconfigError::Invalid)
        );
    }

    #[test]
    fn ok_on_valid_file() {
        let mut cfg = Config::new();
        assert_eq!(cfg.from_file("../input/test.cfg"), Ok(()));
    }

    #[test]
    fn ok_on_valid_i32_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.from_file("../input/test.cfg"), Ok(()));
        assert_eq!(cfg.lookup_i32("val_int"), Some(42));
    }

    #[test]
    fn ok_on_nested_valid_i32_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.from_file("../input/test.cfg"), Ok(()));
        assert_eq!(cfg.lookup_i32("outer.inner"), Some(3));
    }

    #[test]
    fn ok_on_valid_i64_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.from_file("../input/test.cfg"), Ok(()));
        assert_eq!(cfg.lookup_i64("val_u64"), Some(0xFFFFFFFFFF));
    }

    #[test]
    fn ok_on_nested_valid_f32_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.from_file("../input/test.cfg"), Ok(()));
        assert_eq!(cfg.lookup_f32("some_f32"), Some(0.48));
    }

    #[test]
    fn ok_on_nested_valid_f64_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.from_file("../input/test.cfg"), Ok(()));
        assert_eq!(cfg.lookup_f64("some_f64"), Some(1e10));
    }

    #[test]
    fn ok_on_valid_string_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.from_file("../input/test.cfg"), Ok(()));
        assert_eq!(cfg.lookup_string("name"), Some(String::from("Some Name")));
    }
}
