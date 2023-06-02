# Learning Standford Operating System Course CS140e (Rust)

Course Home: https://cs140e.sergio.bz/

## Build
### Toolchain
The original course uses these build tools:
1. `nightly-2018-01-09` version of Rust
2. `Xargo` to build and customize Rust's `std` library
3. `aarch64-none-elf` GNU toolchain

But newer Rust (e.g. `nightly-2023-05-25`) provides `-build-std` feature and good support for `aarch64-unknown-none` target so `Xargo` and `aarch64-none-elf` GNU toolchain are no longer needed:
```Shell
rustup default nightly-2023-05-25
cargo install cargo-binutils # make it easy to use the LLVM tools
```

(Optional) If you want to use official std lib instead of [the customized one](os/std):
```Shell
rustup target add aarch64-unknown-none # add pre-compiled copy of std library for aarch64-unknown-none
rustup component add rust-src # to recompile standard libraries (core, compiler_builtins etc.)
```

PS: More build settings refer to [os/kernel/rust-toolchain.tom](os/kernel/rust-toolchain.toml) and [os/kernel/.cargo/config.toml](os/kernel/.cargo/config.toml). 

### Build
```Shell
cd os/kernel
cargo build --release
rust-objcopy target/aarch64-unknown-none/release/kernel -O binary <overwrite kernel8.img at root directory of SD card>
```


## Run in `QEMU`
QEMU can be used to emulate a Raspberry Pi 3B+:
```Shell
qemu-system-aarch64 -machine raspi3b -serial null -serial stdio -kernel target/aarch64-unknown-none/release/kernel # or the bin file produced by rust-objcopy
```
PS: The first `-serial null` redirects `UART0` (`PL011`) to the *host* void device, the second `-serial stdio` to redirect `UART1` (`mini UART`) to the *host* stdio.

## `config.txt`
From [Raspberry Pi's doc](https://www.raspberrypi.com/documentation/computers/config_txt.html#what-is-config-txt):
> The Raspberry Pi uses a configuration file instead of the BIOS you would expect to find on a conventional PC. The system configuration parameters, which would traditionally be edited and stored using a BIOS, are stored instead in an optional text file named config.txt. This is read by the GPU before the ARM CPU and Linux are initialised. It must therefore be located on the first (boot) partition of your SD card, alongside bootcode.bin and start.elf. 

A sample `config.txt`:
```
arm_64bit=1
```
**NOTE**: For this course, DO NOT set `kernel_old=1` which will loads kernel to the memory address `0x0`. Instead, kernel should be loaded to `0x80000`.
