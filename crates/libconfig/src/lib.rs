use cxx::{let_cxx_string, UniquePtr};
use libconfig_sys::ffi::{lookupValueI64FromConfig, lookupValueI64FromSetting};
use std::ffi::CString;
use std::pin::Pin;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum LibconfigError {
    #[error("invalid file")]
    Invalid,
}

pub struct Setting<'a> {
    inner: Pin<&'a mut libconfig_sys::ffi::Setting>,
}

impl<'a> Setting<'a> {
    pub fn lookup(&'a mut self, path: &str) -> Result<Setting<'a>, LibconfigError> {
        let s = CString::new(path).expect("invalid file");
        unsafe {
            match libconfig_sys::ffi::lookupSettingFromSetting(self.inner.as_mut(), s.as_ptr()) {
                Ok(setting) => Ok(Setting { inner: setting }),
                Err(_) => Err(LibconfigError::Invalid),
            }
        }
    }

    pub fn exists(&'a self, path: &str) -> bool {
        let s = CString::new(path).expect("invalid file");
        unsafe { self.inner.as_ref().exists(s.as_ptr()) }
    }

    pub fn lookup_bool(&mut self, path: &str) -> Option<bool> {
        let s = CString::new(path).expect("invalid settings");
        let mut tmp = false;
        unsafe {
            match self.inner.as_ref().lookup_bool(s.as_ptr(), &mut tmp) {
                true => Some(tmp),
                false => None,
            }
        }
    }

    pub fn lookup_i32(&mut self, path: &str) -> Option<i32> {
        let s = CString::new(path).expect("invalid settings");
        let mut tmp: i32 = 0;
        unsafe {
            match self.inner.as_ref().lookup_i32(s.as_ptr(), &mut tmp) {
                true => Some(tmp),
                false => None,
            }
        }
    }

    pub fn lookup_i64(&mut self, path: &str) -> Option<i64> {
        let s = CString::new(path).expect("invalid settings");
        let mut tmp: i64 = 0;
        unsafe {
            match lookupValueI64FromSetting(&self.inner, s.as_ptr(), &mut tmp) {
                true => Some(tmp),
                false => None,
            }
        }
    }

    pub fn lookup_f32(&mut self, path: &str) -> Option<f32> {
        let s = CString::new(path).expect("invalid settings");
        let mut tmp: f32 = 0.0;
        unsafe {
            match self.inner.as_ref().lookup_f32(s.as_ptr(), &mut tmp) {
                true => Some(tmp),
                false => None,
            }
        }
    }

    pub fn lookup_f64(&mut self, path: &str) -> Option<f64> {
        let s = CString::new(path).expect("invalid settings");
        let mut tmp: f64 = 0.0;
        unsafe {
            match self.inner.as_ref().lookup_f64(s.as_ptr(), &mut tmp) {
                true => Some(tmp),
                false => None,
            }
        }
    }

    pub fn lookup_string(&mut self, path: &str) -> Option<String> {
        let s = CString::new(path).expect("invalid settings");
        let_cxx_string!(tmp = "");
        unsafe {
            match self.inner.as_ref().lookup_string(s.as_ptr(), tmp.as_mut()) {
                true => Some(tmp.to_string()),
                false => None,
            }
        }
    }
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

    pub fn get_root<'a>(&'a self) -> Setting<'a> {
        Setting {
            inner: unsafe { libconfig_sys::ffi::getRootFromConfig(self.inner.as_ref().unwrap()) },
        }
    }

    pub fn lookup<'a>(&'a mut self, path: &str) -> Result<Setting<'a>, LibconfigError> {
        let s = CString::new(path).expect("invalid file");
        unsafe {
            match libconfig_sys::ffi::lookupSettingFromConfig(
                self.inner.as_mut().unwrap(),
                s.as_ptr(),
            ) {
                Ok(setting) => Ok(Setting { inner: setting }),
                Err(_) => Err(LibconfigError::Invalid),
            }
        }
    }

    pub fn exists(&self, path: &str) -> bool {
        let s = CString::new(path).expect("invalid file");
        unsafe { self.inner.as_ref().unwrap().exists(s.as_ptr()) }
    }

    pub fn lookup_bool(&mut self, path: &str) -> Option<bool> {
        let s = CString::new(path).expect("invalid settings");
        let mut tmp = false;
        unsafe {
            match self
                .inner
                .as_ref()
                .unwrap()
                .lookup_bool(s.as_ptr(), &mut tmp)
            {
                true => Some(tmp),
                false => None,
            }
        }
    }

    pub fn lookup_i32(&mut self, path: &str) -> Option<i32> {
        let s = CString::new(path).expect("invalid settings");
        let mut tmp: i32 = 0;
        unsafe {
            match self
                .inner
                .as_ref()
                .unwrap()
                .lookup_i32(s.as_ptr(), &mut tmp)
            {
                true => Some(tmp),
                false => None,
            }
        }
    }

    pub fn lookup_i64(&mut self, path: &str) -> Option<i64> {
        let s = CString::new(path).expect("invalid settings");
        let mut tmp: i64 = 0;
        unsafe {
            match lookupValueI64FromConfig(&self.inner, s.as_ptr(), &mut tmp) {
                true => Some(tmp),
                false => None,
            }
        }
    }

    pub fn lookup_f32(&mut self, path: &str) -> Option<f32> {
        let s = CString::new(path).expect("invalid settings");
        let mut tmp: f32 = 0.0;
        unsafe {
            match self
                .inner
                .as_ref()
                .unwrap()
                .lookup_f32(s.as_ptr(), &mut tmp)
            {
                true => Some(tmp),
                false => None,
            }
        }
    }

    pub fn lookup_f64(&mut self, path: &str) -> Option<f64> {
        let s = CString::new(path).expect("invalid settings");
        let mut tmp: f64 = 0.0;
        unsafe {
            match self
                .inner
                .as_ref()
                .unwrap()
                .lookup_f64(s.as_ptr(), &mut tmp)
            {
                true => Some(tmp),
                false => None,
            }
        }
    }

    pub fn lookup_string(&mut self, path: &str) -> Option<String> {
        let s = CString::new(path).expect("invalid settings");
        let_cxx_string!(tmp = "");
        unsafe {
            match self
                .inner
                .as_ref()
                .unwrap()
                .lookup_string(s.as_ptr(), tmp.as_mut())
            {
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

    #[test]
    fn ok_on_setting_exists_from_config() {
        let mut cfg = Config::new();
        assert_eq!(cfg.from_file("../input/test.cfg"), Ok(()));
        assert!(cfg.exists("outer"));
    }

    #[test]
    fn ok_on_lookup_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.from_file("../input/test.cfg"), Ok(()));
        assert!(cfg.get_root().lookup("outer").is_ok());
    }

    #[test]
    fn ok_on_setting_exists_from_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.from_file("../input/test.cfg"), Ok(()));
        assert!(cfg.get_root().exists("outer"));
    }
}
