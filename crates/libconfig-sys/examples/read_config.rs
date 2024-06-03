use clap::Parser;
use cxx::let_cxx_string;
use std::ffi::CString;

#[derive(Parser)]
struct Args {
    #[arg(short, long = "cfg")]
    cfg_file: String,
}

fn main() {
    let args = Args::parse();
    let mut cfg = libconfig_sys::ffi::Config_ctor();
    unsafe {
        let s = CString::new(args.cfg_file).expect("CString: new failed");
        match cfg.pin_mut().readFile(s.as_ptr()) {
            Ok(_) => println!("ok"),
            Err(err) => println!("err: {}", err),
        };
    }
    unsafe {
        let s = CString::new("val_int").expect("invalid settings");
        let mut val_int: i32 = 0;
        let _ = cfg.pin_mut().lookup_i32(s.as_ptr(), &mut val_int);
        println!("val: {}", val_int);
    }
    unsafe {
        let s = CString::new("outer.inner").expect("invalid settings");
        let mut val_int: i32 = 0;
        let _ = cfg.pin_mut().lookup_i32(s.as_ptr(), &mut val_int);
        println!("val: {}", val_int);
    }
    unsafe {
        let s = CString::new("name").expect("invalid settings");
        let_cxx_string!(name = "");
        let _ = cfg.pin_mut().lookup_string(s.as_ptr(), name.as_mut());
        println!("val: {}", name);
    }
}
