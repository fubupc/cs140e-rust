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
```Shell
cd os/kernel
cargo build --release --target aarch64-unknown-none # For "Method 1": use pre-compiled
# cargo build --release --target aarch64-unknown-none -Z build-std=core,alloc # For "Method 2": recompile `core`, `alloc` by ourselves
rust-objcopy target/aarch64-unknown-none/release/kernel -O binary <overwrite kernel8.img at root directory of SD card>
```

**NOTE**: We can save typing `--target aarch64-unknown-none`, `-Z build-std=core,alloc` manually by adding them into [cargo configuration files](os/kernel/.cargo/config.toml).

### Test

**NOTE**: Currently test has some limitations such as cannot run locally (on host env), no detailed error message, etc.

`cargo test` depends on built-in `test` crate, which in turn depends on `std` crate. This caused some troubles to `#![no_std]` project:

```Text
error[E0463]: can't find crate for `test`
  --> src/allocator/tests.rs:5:5
4  |       #[test]
   |       ------- in this procedural macro expansion
5  | /     fn test_align_down() {
...  |     ...
24 | |     }
   | |_____^ can't find crate
```

This is because there is no pre-compiled `test` crate available for target `aarch64-unknown-none`. Even if try to recompile `test` crate using `-Z build-std=test` will still report error like:

```Text
error[E0658]: use of unstable library feature 'restricted_std'
|
= help: add `#![feature(restricted_std)]` to the crate attributes to enable
```

It actually makes sense because `test` needs a real `std` implementation to work, but `aarch64-unknown-none` target can only provide a *restricted std* with all functions are implemented as dummies.

There are two possible solutions:

- **Option 1**: Use different targets for `cargo build` and `cargo test`, e.g. use `aarch64-unknown-none` for build and `x86_64-unknown-linux-gnu` for test. However, this method requires mess up the code with conditional compilation (`#cfg(...)`) to handle target specific code, e.g. the `global_asm!`/`asm!` code is designed for `aarch64` and will not compile for the `x86_64` target. Additionally, the dependencies like `os/pi` also needs to be adjusted accordingly, otherwise `os/kernel` and `os/pi` would use different `std` libs inconsistently.

- **Option 2**: Utilize the [custom_test_frameworks](https://doc.rust-lang.org/unstable-book/language-features/custom-test-frameworks.html) feature: https://os.phil-opp.com/testing/. This approach enables testing directly against `aarch64-unknown-none` target instead of *cross-test*ing. However, it comes with certain limitations, such as certain functions being unavailable in the test code, including `print!` (not supported by our [customized std](os/std/)), `#[should_panic]` etc.

Currently, **Option 2** seems to be a better solution, as chosen.

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
