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
    let libloragw_path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("lora_gateway/libloragw");
    println!(
        "cargo:rustc-link-search=native={}",
        libloragw_path.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=loragw");

    // generate bindings for `libloragw`
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_function("lgw_board_setconf")
        .whitelist_function("lgw_lbt_setconf")
        .whitelist_function("lgw_rxrf_setconf")
        .whitelist_function("lgw_rxif_setconf")
        .whitelist_function("lgw_txgain_setconf")
        .whitelist_function("lgw_start")
        .whitelist_function("lgw_stop")
        .whitelist_function("lgw_receive")
        .whitelist_function("lgw_send")
        .whitelist_function("lgw_status")
        .whitelist_function("lgw_abort_tx")
        .whitelist_function("lgw_get_trigcnt")
        .whitelist_function("lgw_version_info")
        .whitelist_function("lgw_time_on_air")
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
