use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-lib=sensors");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Can't generate bindings for libsensors");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
