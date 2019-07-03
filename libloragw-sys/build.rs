extern crate cc;

fn main() {
    // Build `libtinymt32` (mersenne twister) which `libloragw` depends on.
    cc::Build::new()
        .include("vendor/sx1302_hal/libtools/inc")
        .file("vendor/sx1302_hal/libtools/src/tinymt32.c")
        .static_flag(true)
        .compile("tinymt32");

    // Build our extracted, modified, and vendored `libloragw`.
    cc::Build::new()
        .include("vendor/sx1302_hal/libloragw/inc")
        .include("vendor/sx1302_hal/libtools/inc")
        .include("vendor/sx1302_hal_cfg")
        .file("vendor/sx1302_hal/libloragw/src/loragw_aux.c")
        .file("vendor/sx1302_hal/libloragw/src/loragw_cal.c")
        .file("vendor/sx1302_hal/libloragw/src/loragw_debug.c")
        .file("vendor/sx1302_hal/libloragw/src/loragw_hal.c")
        .file("vendor/sx1302_hal/libloragw/src/loragw_i2c.c")
        .file("vendor/sx1302_hal/libloragw/src/loragw_reg.c")
        .file("vendor/sx1302_hal/libloragw/src/loragw_spi.c")
        .file("vendor/sx1302_hal/libloragw/src/loragw_stts751.c")
        .file("vendor/sx1302_hal/libloragw/src/loragw_sx1250.c")
        .file("vendor/sx1302_hal/libloragw/src/loragw_sx125x.c")
        .file("vendor/sx1302_hal/libloragw/src/loragw_sx1302.c")
        .file("vendor/sx1302_hal/libloragw/src/loragw_sx1302_rx.c")
        .file("vendor/sx1302_hal/libloragw/src/loragw_sx1302_timestamp.c")
        .static_flag(true)
        .compile("loragw");
}
