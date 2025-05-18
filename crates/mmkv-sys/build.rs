use std::{env, path::PathBuf};

fn main() {
    // Path to MMKV Core source
    let mmkv_core = PathBuf::from("MMKV/Core");

    // Configure and build MMKV Core using CMake
    let dst = cmake::Config::new(&mmkv_core)
        .define("CMAKE_BUILD_TYPE", "Release")
        .build_target("core")
        .build();

    // Tell cargo where to find the built library
    println!("cargo:rustc-link-search=native={}", dst.display());
    let mmkv_core_include = mmkv_core.join("include/MMKV");
    // Generate bindings
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for
        .header(mmkv_core_include.join("MMKV.h").to_str().unwrap())
        // .header(mmkv_core_include.join("MMKVPredef.h").to_str().unwrap())
        // .header(mmkv_core_include.join("MMBuffer.h").to_str().unwrap())
        // .header(mmkv_core_include.join("MiniPBCoder.h").to_str().unwrap())
        .use_core()
        .clang_args(["-x", "c++"])
        .clang_arg("-std=c++17")
        .layout_tests(false) //Layout tests are failed. Skip it for now
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // https://github.com/rust-lang/rust-bindgen/issues/2157
        // .clang_arg("-stdlib=libc++")
        // .no_copy("std\\_vector\\_\\_Temporary_value\\_\\_Storage")
        // .manually_drop_union("std\\_vector\\_\\_Temporary_value\\_\\_Storage")
        // .default_non_copy_union_style(bindgen::NonCopyUnionStyle::ManuallyDrop)
        .allowlist_type("MMKV")
        .opaque_type("MMKV")
        .opaque_type("^(std.*)$")
        .formatter(bindgen::Formatter::Prettyplease)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
