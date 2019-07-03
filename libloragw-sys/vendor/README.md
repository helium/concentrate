This directory holds external vendor code, primarily a submodule
pointing to Semtech's `sx1302_hal`. Their HAL depends on a the
existence of a `config.h` header, but that header is not checked into
their repository. Therefore, in order to allow us to vend an
unmodified version of the upstream HAL, we place the config file in
`sx1302_hal_cfg/`.

## Building ##

We are not using Semtech's `Makefile`s, as they are too opinionated
and do not follow build system best practices. Instead, we perform a
targeted build of only the sources we need via the Rust `cc`
crate. See `libloragw-sys/src/build.rs` for more info.
