use clap::Parser;
use std::ffi::CString;

#[derive(Parser)]
struct Args {
    #[arg(short, long = "cfg")]
    cfg_file: String,
}

fn main() {
    let args = Args::parse();
    let mut cfg = libconfig_sys::ffi::Config_ctor();
    let s = CString::new(args.cfg_file).expect("CString: new failed");
    unsafe {
        match cfg.pin_mut().readFile(s.as_ptr()) {
            Ok(_) => println!("ok"),
            Err(err) => println!("err: {}", err),
        };
    }
}
