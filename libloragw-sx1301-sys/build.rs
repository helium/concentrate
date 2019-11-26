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
}
