fn main() {
    let lib = cmake::Config::new("libconfig")
        .define("BUILD_EXAMPLES", "false")
        .define("BUILD_TESTS", "false")
        .define("BUILD_SHARED_LIBS", "false")
        .build();
    println!(
        "cargo:rustc-env=LIBCONFIG_PATH={}",
        lib.to_str().expect("path valid")
    );
}
