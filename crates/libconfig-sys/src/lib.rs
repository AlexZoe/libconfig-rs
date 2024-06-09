#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("libconfig-sys/include/wrapper.h");

        type Setting;

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
        // Cannot use as member function due to lifetime
        unsafe fn lookupSettingFromSetting<'c>(
            cfg: Pin<&'c mut Setting>,
            path: *const c_char,
        ) -> Result<Pin<&'c mut Setting>>;
        unsafe fn getPathFromSetting(setting: &Setting, path: Pin<&mut CxxString>);

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
