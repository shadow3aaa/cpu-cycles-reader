use std::{env, path::Path};

fn main() {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        println!("cargo:rerun-if-changed=src/ffi/ffi.c");
        let ffi_c_path = Path::new("src/ffi/ffi.c");

        // generate link library
        let out_dir = env::var("OUT_DIR").unwrap();
        cc::Build::new().file(ffi_c_path).compile("ffi");

        println!("cargo:rustc-link-search=native={}", out_dir);
        println!("cargo:rustc-link-lib=static=ffi");
    }
}
