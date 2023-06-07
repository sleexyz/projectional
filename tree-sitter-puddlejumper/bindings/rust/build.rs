fn main() {
    let src_dir = std::path::Path::new("src");
    let parser_path = src_dir.join("parser.c");
    let mut compiler = cc::Build::new();

    // set minimal C sysroot if wasm32-unknown-unknown
    if std::env::var("TARGET").unwrap() == "wasm32-unknown-unknown" {
        let sysroot_dir = std::path::Path::new("bindings/rust/wasm-sysroot");
        compiler
            .compiler("clang-wasi32")
            .archiver("llvm-ar")
            .include(&sysroot_dir);
    }

    compiler.include(&src_dir);
    compiler
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs")
        .file(&parser_path)
        .compile("parser");

    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());

    #[cfg(feature = "native")]
    {
        let mut cpp_config = cc::Build::new();
        cpp_config.cpp(true);
        cpp_config.include(&src_dir);
        cpp_config
            .flag_if_supported("-Wno-unused-parameter")
            .flag_if_supported("-Wno-unused-but-set-variable");
        let scanner_path = src_dir.join("scanner.cc");
        cpp_config.file(&scanner_path);
        cpp_config.compile("scanner");
        println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
    }
}
