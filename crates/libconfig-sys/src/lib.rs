#[cxx::bridge]
pub mod ffi {

    #[derive(Debug)]
    #[repr(u32)]
    enum Type {
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

    #[derive(Debug)]
    #[repr(u32)]
    enum Format {
        FormatDefault,
        FormatHex,
    }

    unsafe extern "C++" {
        include!("libconfig-sys/include/wrapper.h");

        type Type;
        type Format;

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
        unsafe fn getType(self: &Setting) -> Type;
        unsafe fn getFormat(self: &Setting) -> Format;
        unsafe fn setFormat(self: Pin<&mut Setting>, format: Format);
        unsafe fn getLength(self: &Setting) -> Result<i32>;
        unsafe fn isGroup(self: &Setting) -> bool;
        unsafe fn isArray(self: &Setting) -> bool;
        unsafe fn isList(self: &Setting) -> bool;
        unsafe fn isAggregate(self: &Setting) -> bool;
        unsafe fn isScalar(self: &Setting) -> bool;
        unsafe fn isNumber(self: &Setting) -> bool;
        unsafe fn isString(self: &Setting) -> bool;

        unsafe fn setBool<'c>(setting: Pin<&'c mut Setting>, val: bool) -> Result<()>;
        unsafe fn setI32<'c>(setting: Pin<&'c mut Setting>, val: i32) -> Result<()>;
        unsafe fn setI64<'c>(setting: Pin<&'c mut Setting>, val: i64) -> Result<()>;
        unsafe fn setF32<'c>(setting: Pin<&'c mut Setting>, val: f32) -> Result<()>;
        unsafe fn setF64<'c>(setting: Pin<&'c mut Setting>, val: f64) -> Result<()>;
        unsafe fn setString<'c>(setting: Pin<&'c mut Setting>, val: &CxxString) -> Result<()>;
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
        unsafe fn addSetting<'c>(
            setting: Pin<&'c mut Setting>,
            name: &CxxString,
            libtype: Type,
        ) -> Result<Pin<&'c mut Setting>>;
        unsafe fn removeSetting<'c>(setting: Pin<&'c mut Setting>, name: &CxxString) -> Result<()>;
        unsafe fn removeSettingByIndex<'c>(setting: Pin<&'c mut Setting>, idx: u32) -> Result<()>;
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
        unsafe fn writeFile(self: Pin<&mut Config>, filename: &CxxString) -> Result<()>;
        unsafe fn readString(self: Pin<&mut Config>, input: &CxxString) -> Result<()>;
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
