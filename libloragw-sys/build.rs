extern crate cc;

fn main() {
    // Build `libtinymt32` (mersenne twister) which `libloragw` depends on.
    cc::Build::new()
        .include("vendor/tinymt32")
        .file("vendor/tinymt32/tinymt32.c")
        .static_flag(true)
        .compile("tinymt32");

    // Build our extracted, modified, and vendored `libloragw`.
    cc::Build::new()
        .include("vendor/libloragw")
        .include("vendor/libloragw/inc")
        .include("vendor/tinymt32")
        .file("vendor/libloragw/src/loragw_aux.c")
        .file("vendor/libloragw/src/loragw_cal.c")
        .file("vendor/libloragw/src/loragw_debug.c")
        .file("vendor/libloragw/src/loragw_hal.c")
        .file("vendor/libloragw/src/loragw_i2c.c")
        .file("vendor/libloragw/src/loragw_reg.c")
        .file("vendor/libloragw/src/loragw_spi.c")
        .file("vendor/libloragw/src/loragw_stts751.c")
        .file("vendor/libloragw/src/loragw_sx1250.c")
        .file("vendor/libloragw/src/loragw_sx125x.c")
        .file("vendor/libloragw/src/loragw_sx1302.c")
        .file("vendor/libloragw/src/loragw_sx1302_rx.c")
        .file("vendor/libloragw/src/loragw_sx1302_timestamp.c")
        .static_flag(true)
        .compile("loragw");
}
