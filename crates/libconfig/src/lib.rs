use cxx::{let_cxx_string, UniquePtr};
use libconfig_sys::ffi::{
    getParentFromSetting, getPathFromSetting, getRootFromConfig, lookupSettingFromConfig,
    lookupSettingFromSetting, lookupValueI64FromConfig, lookupValueI64FromSetting, Config_ctor,
};
use std::borrow::BorrowMut;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::pin::Pin;
use thiserror::Error;

pub use libconfig_sys::ffi::LibType;

#[derive(Error, Debug, PartialEq)]
pub enum LibconfigError {
    #[error("invalid file")]
    Invalid,
}

pub struct Setting<'a> {
    inner: Pin<&'a mut libconfig_sys::ffi::Setting>,
}

pub struct SettingIter<'a> {
    inner: Option<UniquePtr<libconfig_sys::ffi::SettingIterator>>,
    items: usize,
    count: usize,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> Setting<'a> {
    pub fn lookup(&'a mut self, path: &str) -> Result<Setting<'a>, LibconfigError> {
        let s = CString::new(path).expect("invalid file");
        unsafe {
            match lookupSettingFromSetting(self.inner.as_mut(), s.as_ptr()) {
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

    pub fn set_bool(&mut self, val: bool) -> Result<(), LibconfigError> {
        unsafe {
            match libconfig_sys::ffi::setBool(self.inner.as_mut(), val) {
                Ok(_) => Ok(()),
                Err(_) => Err(LibconfigError::Invalid),
            }
        }
    }

    pub fn set_i32(&mut self, val: i32) -> Result<(), LibconfigError> {
        unsafe {
            match libconfig_sys::ffi::setI32(self.inner.as_mut(), val) {
                Ok(_) => Ok(()),
                Err(_) => Err(LibconfigError::Invalid),
            }
        }
    }

    pub fn set_i64(&mut self, val: i64) -> Result<(), LibconfigError> {
        unsafe {
            match libconfig_sys::ffi::setI64(self.inner.as_mut(), val) {
                Ok(_) => Ok(()),
                Err(_) => Err(LibconfigError::Invalid),
            }
        }
    }

    pub fn set_f32(&mut self, val: f32) -> Result<(), LibconfigError> {
        unsafe {
            match libconfig_sys::ffi::setF32(self.inner.as_mut(), val) {
                Ok(_) => Ok(()),
                Err(_) => Err(LibconfigError::Invalid),
            }
        }
    }

    pub fn set_f64(&mut self, val: f64) -> Result<(), LibconfigError> {
        unsafe {
            match libconfig_sys::ffi::setF64(self.inner.as_mut(), val) {
                Ok(_) => Ok(()),
                Err(_) => Err(LibconfigError::Invalid),
            }
        }
    }

    pub fn set_str(&mut self, val: &str) -> Result<(), LibconfigError> {
        unsafe {
            let_cxx_string!(s = val);
            match libconfig_sys::ffi::setString(self.inner.as_mut(), &s) {
                Ok(_) => Ok(()),
                Err(_) => Err(LibconfigError::Invalid),
            }
        }
    }

    pub fn remove(&'a mut self, path: &str) -> Result<(), LibconfigError> {
        unsafe {
            let_cxx_string!(s = path);
            match libconfig_sys::ffi::removeSetting(self.inner.as_mut(), &s) {
                Ok(_) => Ok(()),
                Err(_) => Err(LibconfigError::Invalid),
            }
        }
    }

    pub fn remove_idx(&'a mut self, idx: usize) -> Result<(), LibconfigError> {
        unsafe {
            match libconfig_sys::ffi::removeSettingByIndex(self.inner.as_mut(), idx as u32) {
                Ok(_) => Ok(()),
                Err(_) => Err(LibconfigError::Invalid),
            }
        }
    }

    pub fn add(
        &'a mut self,
        path: &str,
        setting_type: LibType,
    ) -> Result<Setting<'a>, LibconfigError> {
        unsafe {
            let_cxx_string!(s = path);
            match libconfig_sys::ffi::addSetting(self.inner.as_mut(), &s, setting_type) {
                Ok(setting) => Ok(Setting { inner: setting }),
                Err(_) => Err(LibconfigError::Invalid),
            }
        }
    }

    pub fn get_name(&'a self) -> Option<&'a str> {
        unsafe {
            match self.inner.as_ref().getName() {
                s if !s.is_null() => Some(CStr::from_ptr(s).to_str().unwrap()),
                _ => None,
            }
        }
    }

    pub fn get_path(&self) -> String {
        let_cxx_string!(tmp = "");
        unsafe {
            getPathFromSetting(&self.inner, tmp.as_mut());
            tmp.to_string()
        }
    }

    pub fn get_parent(&'a mut self) -> Result<Setting<'a>, LibconfigError> {
        unsafe {
            match getParentFromSetting(self.inner.as_mut()) {
                Ok(setting) => Ok(Setting { inner: setting }),
                Err(_) => Err(LibconfigError::Invalid),
            }
        }
    }

    pub fn is_root(&self) -> bool {
        unsafe { self.inner.as_ref().isRoot() }
    }

    pub fn get_index(&self) -> Option<usize> {
        unsafe {
            match self.inner.getIndex() {
                -1 => None,
                i => Some(i as usize),
            }
        }
    }

    pub fn get_type(&self) -> LibType {
        unsafe { self.inner.getType() }
    }

    pub fn get_length(&self) -> Result<i32, LibconfigError> {
        unsafe {
            match self.inner.getLength() {
                Ok(length) => Ok(length),
                _ => Err(LibconfigError::Invalid),
            }
        }
    }

    pub fn is_group(&self) -> bool {
        unsafe { self.inner.isGroup() }
    }

    pub fn is_array(&self) -> bool {
        unsafe { self.inner.isArray() }
    }

    pub fn is_list(&self) -> bool {
        unsafe { self.inner.isList() }
    }

    pub fn is_aggregate(&self) -> bool {
        unsafe { self.inner.isAggregate() }
    }

    pub fn is_scalar(&self) -> bool {
        unsafe { self.inner.isScalar() }
    }

    pub fn is_number(&self) -> bool {
        unsafe { self.inner.isNumber() }
    }

    pub fn is_string(&self) -> bool {
        unsafe { self.inner.isString() }
    }
}

impl<'a> TryInto<bool> for Setting<'a> {
    type Error = LibconfigError;
    fn try_into(self) -> Result<bool, Self::Error> {
        unsafe {
            match libconfig_sys::ffi::tryBoolFromSetting(&self.inner) {
                Err(_) => Err(LibconfigError::Invalid),
                Ok(val) => Ok(val),
            }
        }
    }
}

impl<'a> TryInto<i32> for Setting<'a> {
    type Error = LibconfigError;
    fn try_into(self) -> Result<i32, Self::Error> {
        unsafe {
            match libconfig_sys::ffi::tryI32FromSetting(&self.inner) {
                Err(_) => Err(LibconfigError::Invalid),
                Ok(val) => Ok(val),
            }
        }
    }
}

impl<'a> TryInto<i64> for Setting<'a> {
    type Error = LibconfigError;
    fn try_into(self) -> Result<i64, Self::Error> {
        unsafe {
            match libconfig_sys::ffi::tryI64FromSetting(&self.inner) {
                Err(_) => Err(LibconfigError::Invalid),
                Ok(val) => Ok(val),
            }
        }
    }
}

impl<'a> TryInto<f32> for Setting<'a> {
    type Error = LibconfigError;
    fn try_into(self) -> Result<f32, Self::Error> {
        unsafe {
            match libconfig_sys::ffi::tryF32FromSetting(&self.inner) {
                Err(_) => Err(LibconfigError::Invalid),
                Ok(val) => Ok(val),
            }
        }
    }
}

impl<'a> TryInto<f64> for Setting<'a> {
    type Error = LibconfigError;
    fn try_into(self) -> Result<f64, Self::Error> {
        unsafe {
            match libconfig_sys::ffi::tryF64FromSetting(&self.inner) {
                Err(_) => Err(LibconfigError::Invalid),
                Ok(val) => Ok(val),
            }
        }
    }
}

impl<'a> TryInto<String> for Setting<'a> {
    type Error = LibconfigError;
    fn try_into(self) -> Result<String, Self::Error> {
        unsafe {
            match libconfig_sys::ffi::tryStringFromSetting(&self.inner) {
                Err(_) => Err(LibconfigError::Invalid),
                Ok(val) => Ok(String::from(&*val.to_string())),
            }
        }
    }
}

impl<'a> Iterator for SettingIter<'a> {
    type Item = Setting<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.items {
            None
        } else {
            self.count += 1;
            unsafe {
                Some(Setting {
                    inner: libconfig_sys::ffi::getNextFromIter(
                        self.inner.as_mut().unwrap().borrow_mut(),
                    ),
                })
            }
        }
    }
}

impl<'a> IntoIterator for Setting<'a> {
    type Item = Setting<'a>;
    type IntoIter = SettingIter<'a>;

    fn into_iter(mut self) -> Self::IntoIter {
        unsafe {
            if let Ok(iter) = libconfig_sys::ffi::getSettingIter(self.inner.as_mut()) {
                Self::IntoIter {
                    inner: Some(iter),
                    count: 0,
                    items: self.inner.as_mut().getLength().unwrap_or(0) as usize,
                    _lifetime: PhantomData,
                }
            } else {
                Self::IntoIter {
                    inner: None,
                    count: 0,
                    items: 0,
                    _lifetime: PhantomData,
                }
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
            inner: Config_ctor(),
        }
    }

    pub fn read_file(&mut self, path: &str) -> Result<(), LibconfigError> {
        let s = CString::new(path).expect("invalid file");
        unsafe {
            match self.inner.pin_mut().readFile(s.as_ptr()) {
                Ok(_) => Ok(()),
                Err(_) => Err(LibconfigError::Invalid),
            }
        }
    }

    pub fn write_file(&mut self, path: &str) -> Result<(), LibconfigError> {
        let_cxx_string!(s = path);
        unsafe {
            match self.inner.pin_mut().writeFile(&s) {
                Ok(_) => Ok(()),
                Err(_) => Err(LibconfigError::Invalid),
            }
        }
    }

    pub fn read_str(&mut self, path: &str) -> Result<(), LibconfigError> {
        let_cxx_string!(s = path);
        unsafe {
            match self.inner.pin_mut().readString(&s) {
                Ok(_) => Ok(()),
                Err(_) => Err(LibconfigError::Invalid),
            }
        }
    }

    pub fn set_include_path(&mut self, path: &str) {
        let s = CString::new(path).expect("invalid file");
        unsafe {
            self.inner.pin_mut().setIncludeDir(s.as_ptr());
        }
    }

    pub fn get_root<'a>(&'a self) -> Setting<'a> {
        Setting {
            inner: unsafe { getRootFromConfig(self.inner.as_ref().unwrap()) },
        }
    }

    pub fn lookup<'a>(&'a mut self, path: &str) -> Result<Setting<'a>, LibconfigError> {
        let s = CString::new(path).expect("invalid file");
        unsafe {
            match lookupSettingFromConfig(self.inner.as_mut().unwrap(), s.as_ptr()) {
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
            cfg.read_file("non_existing.cfg"),
            Err(LibconfigError::Invalid)
        );
    }

    #[test]
    fn error_on_invalid_file() {
        let mut cfg = Config::new();
        assert_eq!(
            cfg.read_file("../input/invalid.cfg"),
            Err(LibconfigError::Invalid)
        );
    }

    #[test]
    fn ok_on_valid_include_dir() {
        let mut cfg = Config::new();
        cfg.set_include_path("../input");
        assert_eq!(cfg.read_file("../input/test_with_include.cfg"), Ok(()));
    }

    #[test]
    fn error_on_valid_include_dir() {
        let mut cfg = Config::new();
        cfg.set_include_path("../");
        assert_eq!(
            cfg.read_file("../input/test_with_include.cfg"),
            Err(LibconfigError::Invalid)
        );
    }

    #[test]
    fn ok_on_valid_file() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
    }

    #[test]
    fn ok_on_valid_i32_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        assert_eq!(cfg.lookup_i32("val_int"), Some(42));
    }

    #[test]
    fn ok_on_nested_valid_i32_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        assert_eq!(cfg.lookup_i32("outer.inner"), Some(3));
    }

    #[test]
    fn ok_on_valid_i64_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        assert_eq!(cfg.lookup_i64("val_u64"), Some(0xFFFFFFFFFF));
    }

    #[test]
    fn ok_on_nested_valid_f32_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        assert_eq!(cfg.lookup_f32("some_f32"), Some(0.48));
    }

    #[test]
    fn ok_on_nested_valid_f64_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        assert_eq!(cfg.lookup_f64("some_f64"), Some(1e10));
    }

    #[test]
    fn ok_on_valid_string_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        assert_eq!(cfg.lookup_string("name"), Some(String::from("Some Name")));
    }

    #[test]
    fn ok_on_setting_exists_from_config() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        assert!(cfg.exists("outer"));
    }

    #[test]
    fn ok_on_lookup_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        assert!(cfg.get_root().lookup("outer").is_ok());
    }

    #[test]
    fn ok_on_setting_exists_from_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        assert!(cfg.get_root().exists("outer"));
    }

    #[test]
    fn ok_on_underlying_setting_exists_from_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        if let Ok(setting) = cfg.get_root().lookup("outer") {
            assert!(setting.exists("inner"));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn ok_on_setting_name() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        if let Ok(setting) = cfg.get_root().lookup("outer") {
            assert_eq!(setting.get_name(), Some("outer"));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn err_on_setting_without_name() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        let setting = cfg.get_root();
        assert!(setting.get_name().is_none());
    }

    #[test]
    fn ok_on_setting_path() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        if let Ok(setting) = cfg.get_root().lookup("outer").unwrap().lookup("inner") {
            assert_eq!(setting.get_path(), "outer.inner");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn ok_on_setting_get_parent() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        if let Ok(mut setting) = cfg.get_root().lookup("outer").unwrap().lookup("inner") {
            assert_eq!(setting.get_parent().unwrap().get_name(), Some("outer"));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn ok_on_setting_is_root() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        let setting = cfg.get_root();
        assert!(setting.is_root());
    }

    #[test]
    fn err_on_setting_is_not_root() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        if let Ok(setting) = cfg.get_root().lookup("outer").unwrap().lookup("inner") {
            assert!(!setting.is_root());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn ok_on_setting_get_type_int() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        if let Ok(setting) = cfg.get_root().lookup("outer").unwrap().lookup("inner") {
            assert_eq!(setting.get_type(), LibType::TypeInt);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn ok_on_setting_get_type_x() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        if let Ok(setting) = cfg.get_root().lookup("outer") {
            assert_eq!(setting.get_type(), LibType::TypeGroup);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn none_for_non_aggregate() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        if let Ok(setting) = cfg.get_root().lookup("val_int") {
            assert_eq!(setting.get_type(), LibType::TypeInt);
            let mut iter = setting.into_iter();
            assert!(iter.next().is_none());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn some_for_aggregate() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        if let Ok(setting) = cfg.get_root().lookup("arr") {
            assert_eq!(setting.get_type(), LibType::TypeArray);
            let mut iter = setting.into_iter();
            assert_eq!(iter.next().unwrap().try_into(), Ok(3));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn iter_returns_none_after_last_item() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        if let Ok(setting) = cfg.get_root().lookup("arr") {
            assert_eq!(setting.get_type(), LibType::TypeArray);
            let mut iter = setting.into_iter();
            assert_eq!(iter.next().unwrap().try_into(), Ok(3));
            assert_eq!(iter.next().unwrap().try_into(), Ok(5));
            assert_eq!(iter.next().unwrap().try_into(), Ok(8));
            assert!(iter.next().is_none());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn write_setting() {
        let mut cfg = Config::new();
        assert_eq!(cfg.read_file("../input/test.cfg"), Ok(()));
        let mut setting = cfg.get_root();
        let setting = setting.add("new_val", LibType::TypeInt);
        _ = setting.unwrap().set_i32(5);
        _ = cfg.write_file("../input/test.cfg");
    }
}
