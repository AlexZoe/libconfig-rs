#[cxx::bridge]
pub mod ffi {

    #[derive(Debug)]
    #[repr(u32)]
    enum LibType {
        TypeNone,
        TypeInt,
        TypeInt64,
        TypeFloat,
        TypeString,
        TypeBoolean,
        TypeGroup,
        TypeArray,
        TypeList,
    }

    unsafe extern "C++" {
        include!("libconfig-sys/include/wrapper.h");

        type LibType;

        type Setting;
        type SettingIterator;

        unsafe fn exists(self: &Setting, path: *const c_char) -> bool;
        #[rust_name = "lookup_bool"]
        unsafe fn lookupValue(self: &Setting, path: *const c_char, value: &mut bool) -> bool;
        #[rust_name = "lookup_i32"]
        unsafe fn lookupValue(self: &Setting, path: *const c_char, value: &mut i32) -> bool;
        #[rust_name = "lookup_u32"]
        unsafe fn lookupValue(self: &Setting, path: *const c_char, value: &mut u32) -> bool;
        #[rust_name = "lookup_f32"]
        unsafe fn lookupValue(self: &Setting, path: *const c_char, value: &mut f32) -> bool;
        #[rust_name = "lookup_f64"]
        unsafe fn lookupValue(self: &Setting, path: *const c_char, value: &mut f64) -> bool;
        #[rust_name = "lookup_string"]
        unsafe fn lookupValue(
            self: &Setting,
            path: *const c_char,
            value: Pin<&mut CxxString>,
        ) -> bool;
        unsafe fn getName(self: &Setting) -> *const c_char;
        unsafe fn isRoot(self: &Setting) -> bool;
        unsafe fn getIndex(self: &Setting) -> i32;
        unsafe fn getType(self: &Setting) -> LibType;
        unsafe fn getLength(self: &Setting) -> Result<i32>;
        unsafe fn isGroup(self: &Setting) -> bool;
        unsafe fn isArray(self: &Setting) -> bool;
        unsafe fn isList(self: &Setting) -> bool;
        unsafe fn isAggregate(self: &Setting) -> bool;
        unsafe fn isScalar(self: &Setting) -> bool;
        unsafe fn isNumber(self: &Setting) -> bool;
        unsafe fn isString(self: &Setting) -> bool;

        // Cannot use "[unsigned] long long" directly for now
        unsafe fn lookupValueI64FromSetting(
            setting: &Setting,
            path: *const c_char,
            value: &mut i64,
        ) -> bool;
        unsafe fn lookupValueU64FromSetting(
            setting: &Setting,
            path: *const c_char,
            value: &mut u64,
        ) -> bool;
        unsafe fn tryBoolFromSetting(setting: &Setting) -> Result<bool>;
        unsafe fn tryI32FromSetting(setting: &Setting) -> Result<i32>;
        unsafe fn tryI64FromSetting(setting: &Setting) -> Result<i64>;
        unsafe fn tryF32FromSetting(setting: &Setting) -> Result<f32>;
        unsafe fn tryF64FromSetting(setting: &Setting) -> Result<f64>;
        unsafe fn tryStringFromSetting(setting: &Setting) -> Result<UniquePtr<CxxString>>;
        // Cannot use as member function due to lifetime
        unsafe fn lookupSettingFromSetting<'c>(
            setting: Pin<&'c mut Setting>,
            path: *const c_char,
        ) -> Result<Pin<&'c mut Setting>>;
        unsafe fn getPathFromSetting(setting: &Setting, path: Pin<&mut CxxString>);
        unsafe fn getParentFromSetting<'c>(
            setting: Pin<&'c mut Setting>,
        ) -> Result<Pin<&'c mut Setting>>;
        unsafe fn getSettingIter<'c>(
            setting: Pin<&'c mut Setting>,
        ) -> Result<UniquePtr<SettingIterator>>;
        unsafe fn getNextFromIter<'c>(
            iter: &mut UniquePtr<SettingIterator>,
        ) -> Pin<&'c mut Setting>;

        type Config;

        #[cxx_name = "construct_unique"]
        fn Config_ctor() -> UniquePtr<Config>;

        unsafe fn readFile(self: Pin<&mut Config>, filename: *const c_char) -> Result<()>;
        unsafe fn setIncludeDir(self: Pin<&mut Config>, path: *const c_char);
        unsafe fn exists(self: &Config, path: *const c_char) -> bool;
        #[rust_name = "lookup_bool"]
        unsafe fn lookupValue(self: &Config, path: *const c_char, value: &mut bool) -> bool;
        #[rust_name = "lookup_i32"]
        unsafe fn lookupValue(self: &Config, path: *const c_char, value: &mut i32) -> bool;
        #[rust_name = "lookup_u32"]
        unsafe fn lookupValue(self: &Config, path: *const c_char, value: &mut u32) -> bool;
        #[rust_name = "lookup_f32"]
        unsafe fn lookupValue(self: &Config, path: *const c_char, value: &mut f32) -> bool;
        #[rust_name = "lookup_f64"]
        unsafe fn lookupValue(self: &Config, path: *const c_char, value: &mut f64) -> bool;
        #[rust_name = "lookup_string"]
        unsafe fn lookupValue(
            self: &Config,
            path: *const c_char,
            value: Pin<&mut CxxString>,
        ) -> bool;

        // Cannot use "[unsigned] long long" directly for now
        unsafe fn lookupValueI64FromConfig(
            config: &Config,
            path: *const c_char,
            value: &mut i64,
        ) -> bool;
        unsafe fn lookupValueU64FromConfig(
            config: &Config,
            path: *const c_char,
            value: &mut u64,
        ) -> bool;
        // Cannot use as member function due to lifetime
        unsafe fn getRootFromConfig<'c>(config: &'c Config) -> Pin<&'c mut Setting>;
        unsafe fn lookupSettingFromConfig<'c>(
            cfg: Pin<&'c mut Config>,
            path: *const c_char,
        ) -> Result<Pin<&'c mut Setting>>;
    }
}
