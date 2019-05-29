extern crate bindgen;
extern crate cc;

fn main() {
    // Build our extracted, modified, and vendored `libloragw`.
    //
    // The origial source can be found at
    // https://github.com/Lora-net/lora_gateway
    cc::Build::new()
        .file("vendor/libloragw/loragw_aux.c")
        .file("vendor/libloragw/loragw_fpga.c")
        .file("vendor/libloragw/loragw_gps.c")
        .file("vendor/libloragw/loragw_hal.c")
        .file("vendor/libloragw/loragw_lbt.c")
        .file("vendor/libloragw/loragw_radio.c")
        .file("vendor/libloragw/loragw_reg.c")
        .file("vendor/libloragw/loragw_spi.native.c")
        .static_flag(true)
        .compile("loragw");

    // Generate rust bindings to HAL portion of `libloragw`
    bindgen::Builder::default()
        .header("vendor/libloragw/loragw_hal.h")
        .derive_default(true)
        .derive_debug(true)
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
        .expect("Unable to generate bindings")
        .write_to_file(
            ::std::path::PathBuf::from(::std::env::var("OUT_DIR").unwrap()).join("bindings.rs"),
        )
        .expect("Couldn't write bindings!");
}
