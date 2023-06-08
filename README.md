# Learning Standford Operating System Course CS140e (Rust)

Course Home: https://cs140e.sergio.bz/

## Build
### Install Toolchain
The original course uses these build tools:
1. `nightly-2018-01-09` version of Rust
2. `Xargo` to build and customize Rust's `std` library
3. `aarch64-none-elf` GNU toolchain

But newer Rust (e.g. `nightly-2023-05-25`) provides `-build-std` feature and good support for `aarch64-unknown-none` target so `Xargo` and `aarch64-none-elf` GNU toolchain are no longer needed:

```Shell
rustup default nightly-2023-05-25
cargo install cargo-binutils # make it easy to use the LLVM tools
```

This course uses a [customized std lib](os/std), but it still depends on some built-in libs such as `core`, `alloc` etc. To build for the `aarch64-unknown-none` target, one of the following methods can be choosed:

- **Method 1**: Install pre-compiled copy of `core`, `alloc` etc libraries:
    ```Shell
    rustup target add aarch64-unknown-none # add pre-compiled copy of std library for aarch64-unknown-none
    ```
- **Method 2**: Install `rust-src` to be able to recompile `core`, `alloc` etc libraries:
    ```Shell
    rustup component add rust-src
    ```

### Build Kernel

Before talking about build, there are 2 problems about test:
1. Where to run test?
   1. On local machine: This usually means run test on host target like `x86_64-unknown-linux-gnu`. However, this project is designed for `aarch64-unknown-none` so some targe specific code will not work.
   2. On real Pi hardware (or QEMU): No previous problem but inconvinient.
2. This project used a [customized std](os/std/) while test depends on the built-in full-featured `std` lib. Two possible solutions:
   1. Make test not depends on built-in `std` using the [custom_test_frameworks](https://doc.rust-lang.org/unstable-book/language-features/custom-test-frameworks.html) feature: https://os.phil-opp.com/testing/. But it comes with certain limitations: cannot use `#[should_panic]`, multi-thread, `print!` (not supported by our [customized std](os/std/)) etc.
   2. Make the code switch to use built-in `std` when testing by using `#[cfg(test)]` conditional compilation. But it means dependencies like `os/pi`, `2-fs/fat32` needs to be adjusted as well.

Currently, run test on local machine + switch `std` is chosen:
1. For `os/kernel` and its dependencies like `os/pi`, `2-fs/fat32`, introduce a `custom-std` feature to switch between customized std and built-in std.
2. Use `#[cfg(not(test))]` for target specific code.

#### Build:
```Shell
cd os/kernel

# For "Method 1": use pre-compiled
cargo build --release --target aarch64-unknown-none --features custom-std
# For "Method 2": recompile `core`, `alloc` by ourselves
cargo build --release --target aarch64-unknown-none --features custom-std -Z build-std=core,alloc

rust-objcopy target/aarch64-unknown-none/release/kernel -O binary <overwrite kernel8.img at root directory of SD card>
```

#### Test:
```Shell
cd os/kernel
cargo test
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

```Text
arm_64bit=1
```

**NOTE**: For this course, DO NOT set `kernel_old=1` which will loads kernel to the memory address `0x0`. Instead, kernel should be loaded to `0x80000`.
