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
For 10.76.100.10: 

./concentrate -p -r 10.76.100.1 -u 31335 -l 31336 serve
./target/debug/concentrate -r 10.76.100.10 -l 31335 -u 31336 -o 31342 -i 31343 longfi
./target/debug/concentrate -i 31342 -o 31343 longfi-test

For 10.76.100.11: 
./concentrate -p -r 10.76.100.1 -u 31337 -l 31338 serve
./target/debug/concentrate -r 10.76.100.11 -l 31337 -u 31338 -o 31340 -i 31341 longfi
./target/debug/concentrate -i 31340 -o 31341 longfi-test