#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("libconfig-sys/include/wrapper.h");

        type Config;

        #[cxx_name = "construct_unique"]
        fn Config_ctor() -> UniquePtr<Config>;

        unsafe fn readFile(self: Pin<&mut Config>, filename: *const c_char) -> Result<()>;
        fn getOptions(self: &Config) -> i32;
        fn setOptions(self: Pin<&mut Config>, options: i32);
        fn clear(self: Pin<&mut Config>);

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
        unsafe fn lookupValue(self: &Config, path: *const c_char, value: Pin<&mut CxxString>) -> bool;

        // Can't use "[unsigned] long long" directly for now
        unsafe fn lookupValueI64(config: &Config, path: *const c_char, value: &mut i64) -> bool;
        unsafe fn lookupValueU64(config: &Config, path: *const c_char, value: &mut u64) -> bool;
    }
}
