# Concentrate

A packet arbiter.

## Code Status

[![Build Status](https://travis-ci.com/helium/concentrate.svg?token=gSksRQHcDis4sPKF5NRm&branch=master)](https://travis-ci.com/helium/concentrate)

## Required Toolchain

You will need to download and install the ''aarch64-linux-gnu-gcc' toolchain and add the 'aarch64-unknown-linux-gnu' target to rustup in order to build this application for the current target (Rasberry Pi 3 B+).

The included .cargo/config file will specify the linker needed to complete the build. Setup instructions by system are as follows:

### Arch Linux

```zsh
> sudo pacman -S aarch64-linux-gnu-gcc
> rustup target add aarch64-unknown-linux-gnu
> cargo build --target aarch64-unknown-linux-gnu
```


