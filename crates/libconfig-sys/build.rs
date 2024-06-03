fn main() {
    println!("cargo:rustc-link-lib=static=config++");
    println!(
        "cargo:rustc-link-search={}",
        libconfig_cpp::libconfig_path()
            .join("lib")
            .to_str()
            .unwrap()
    );

    cxx_build::bridge("src/lib.rs")
        .std("c++14")
        .include("include")
        .include(
            libconfig_cpp::libconfig_path()
                .join("include")
                .to_str()
                .unwrap(),
        )
        .compile("wrapper");

    println!("cargo:rustc-link-lib=static=wrapper");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include/wrapper.h");
}
