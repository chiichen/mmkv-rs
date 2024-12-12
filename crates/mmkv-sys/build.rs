use std::{env, path::PathBuf};

fn main() {
    let mut config = cmake::Config::new("MMKV/Core");
    let dst = config.build();
    println!("cargo:rustc-link-lib=staticlib=core");
    println!("cargo:rustc-link-search={}", dst.display());
    println!("cargo:rustc-link-search={}", dst.join("lib").display());
    let bindings = bindgen::Builder::default()
        .header("MMKV/Core/MMKV.h")
        .use_core()
        .clang_args([
            format!("-I{}/{}", dst.display(), "include"),
            #[cfg(target_os = "windows")]
            r#"-DWIN32"#.to_string(), // As Above
        ])
        .generate()
        .expect("unable to generate rfb bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings!");
}
