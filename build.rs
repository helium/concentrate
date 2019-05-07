use bindgen;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // build `libloragw`
    Command::new("make")
        .args(&["-C", "lora_gateway/libloragw"])
        .status()
        .expect("libloragw build failed");

    // statically link to `libloragw`
    println!("cargo:rustc-link-search=native={}", "lora_gateway/libloragw");
    println!("cargo:rustc-link-lib=static=loragw");

    // generate bindings for `libloragw`
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

}
