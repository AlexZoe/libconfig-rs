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
    }
}
