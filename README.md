# Concentrate

A packet arbiter.

## Code Status

[![Build Status](https://travis-ci.com/helium/concentrate.svg?token=gSksRQHcDis4sPKF5NRm&branch=master)](https://travis-ci.com/helium/concentrate)

## Required Toolchain

You will need to download and install the ''aarch64-linux-gnu-gcc' toolchain and add the 'aarch64-unknown-linux-gnu' target to rustup in order to build this application for the current target (Rasberry Pi 3 B+).

The included .cargo/config file will specify the linker needed to complete the build. Setup instructions by system are as follows:

### Linux
Install the toolchain to your system:
#### Arch
Install the toolchain:
```sh
> sudo pacman -S aarch64-linux-gnu-gcc
```
#### Ubuntu
```sh
> sudo apt-get install gcc-aarch64-linux-gnu
```
#### Common
Add it to Rust:
```sh
> rustup target add aarch64-unknown-linux-gnu
```
Now building is easy:
```sh
> cargo build --target aarch64-unknown-linux-gnu [--release]
```
## Testing
If you are putting the binary on an otherwise working Raspberry Pi image, you will want to kill the pre-existing concentrate service:
```sh
sudo sv d /etc/sv/concentrate
```
If you would like to run a test session with a _remote_ client, you can start a concentrate server with the following options:
```sh
sudo concentrate -p -r 192.168.1.xxx serve
```
On the remote machine, such a local development machine, you can run the client:
```sh
concentrate -p listen
```

### Notes
To build with `log_env`:
```
~/concentrate/concentrate$ cargo build --no-default-features --features log_env
```

For 10.76.100.10: 
```
./concentrate serve -p --publish 10.76.100.1:31337 --listen 0.0.0.0:31338

CONCENTRATE_LOG="debug" ./target/debug/concentrate longfi --radio-listen 10.76.100.10:31338 --radio-publish 0.0.0.0:31337 --longfi-listen 127.0.0.1:31341 --longfi-publish 127.0.0.1:31340

./target/debug/concentrate longfi-test --publish 127.0.0.1:31340 --listen 127.0.0.1:31341
```

For 10.76.100.11:
```
./concentrate serve -p --publish 10.76.100.1:31330 --listen 0.0.0.0:31331

CONCENTRATE_LOG="debug" ./target/debug/concentrate longfi --radio-listen 10.76.100.11:31331 --radio-publish 0.0.0.0:31330 --longfi-listen 127.0.0.1:31332 --longfi-publish 127.0.0.1:31333

./target/debug/concentrate longfi-test --publish 127.0.0.1:31333 --listen 127.0.0.1:31332
```
