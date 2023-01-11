extern crate bindgen;

use std::{
    env,
    path::{PathBuf}
};

fn main() {
    let libdir_path = PathBuf::from("dlss/lib")
        .canonicalize()
        .expect("Cannot canonicalize libdir path.");

    println!("cargo:rustc-link-search=native={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=static={}", "nvsdk_ngx_s");
    println!("cargo:rerun-if-changed=dlss-wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("dlss/include/dlss-wrapper.h")
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Undable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings.");
}
