# Rust low-level bindings for GMPMEE

The gmpmee-sys crate provides Rust FFI bindings to the [GMP Modular Exponentiation Extension (GMPMEE)](https://github.com/verificatum/verificatum-gmpmee), which is a minor extension of [GMP](https://gmplib.org/). It adds simultaneous modular exponentiation and fixed base modular exponentiation functionality to the set of integer functions (the mpz-functions), as well as special purpose primality testing routines.

The crate is strongly inspired from [gmp-mpfr-sys](https://crates.io/crates/gmp-mpfr-sys). In particular the file `build.rs` is copied from this crate, and adapted to the needs of the gmpmee-sys crate. No cache is implemented, since the compilation is quick.

## Types

Unlike in the C libraries, the types (e.g. `gmpmee_spowm_tab`, `gmpmee_fpowm_tab`) are defined directly as structs, not as single-element
arrays.

## Using gmpmee-sys

The gmpmee-sys crate is available on crates.io. To use gmpmee-sys in your crate, add it as a dependency inside [*Cargo.toml*]:

```toml
[dependencies]
gmpmee-sys = "0.1"
```

## Building on GNU/Linux

To build on GNU/Linux, simply make sure you have `diffutils`, `gcc`,
`make` and `m4` installed on your system. For example on Fedora:

```sh
sudo dnf install diffutils gcc make m4
```

## Building on macOS

To build on macOS, you need the command-line developer tools. To
install them, run the following command in a terminal:

```sh
xcode-select --install
```

## Building on Windows

You can build on Windows with the Rust GNU toolchain and an up-to-date
MSYS2 installation. Some steps for a 64-bit environment are listed
below. (32-bit: Changes for a 32-bit environment are written in
brackets like this comment.)

To install MSYS2:

 1. Install MSYS2 using the [installer][msys].

 2. Launch the MSYS2 MinGW 64-bit terminal from the start
    menu. (32-bit: Launch the MSYS2 MinGW 32-bit terminal instead.)

 3. Install the required tools.

    ```sh
    pacman -S pacman-mirrors
    pacman -S diffutils make mingw-w64-x86_64-gcc
    ```

    (32-bit: Install `mingw-w64-i686-gcc` instead of
    `mingw-w64-x86_64-gcc`.)

Then, to build a crate with a dependency on this crate:

 1. Launch the MSYS MinGW 64-bit terminal from the start menu.
    (32-bit: Launch the MSYS2 MinGW 32-bit terminal instead.)

 2. Change to the crate directory.

 3. Build the crate using `cargo`.

## Licence

The gmpee-sys crate is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. See the full text of the [LICENSE](LICENSE.md) for details.
