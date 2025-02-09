// Copyright © 2024 Denis Morel

// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU Lesser General Public License and
// a copy of the GNU General Public License along with this program. If not, see
// <https://www.gnu.org/licenses/>.

//! The gmpmee-sys crate provides Rust FFI bindings to the
//! [GMP Modular Exponentiation Extension (GMPMEE)](https://github.com/verificatum/verificatum-gmpmee),
//! which is a minor extension of [GMP](https://gmplib.org/).
//! It adds simultaneous modular exponentiation and fixed base modular exponentiation functionality to the set of
//! integer functions (the mpz-functions), as well as special purpose primality testing routines.
//!
//! The crate is strongly inspired from [gmp-mpfr-sys](https://crates.io/crates/gmp-mpfr-sys).
//! In particular the file `build.rs` is copied from this crate, and adapted to the needs of the gmpmee-sys crate.
//! No cache is implemented, since the compilation is quick.
//!
//! # Types
//!
//! Unlike in the C libraries, the types (e.g. `gmpmee_spowm_tab`, `gmpmee_fpowm_tab`) are defined directly as structs,
//! not as single-element arrays.
//!
//! # Using gmpmee-sys
//!
//!  The gmpmee-sys crate is available on crates.io.
//! To use gmpmee-sys in your crate, add it as a dependency inside [*Cargo.toml*]:
//!
//! ```toml
//! [dependencies]
//! gmpmee-sys = "0.1"
//! ```
//!
//! # Building on GNU/Linux
//! To build on GNU/Linux, simply make sure you have `diffutils`, `gcc`,
//! `make`, `autotools`, `m4` and `gmp` installed on your system. For example on Debian:
//!
//! ```sh
//! sudo dnf install diffutils gcc make m4 gmp
//! ```
//!
//! # Building on macOS
//!
//! To build on macOS, you need the command-line developer tools. To
//! install them, run the following command in a terminal:
//!
//! ```sh
//! xcode-select --install
//! ```
//!
//! # Building on Windows
//! You can build on Windows with the Rust GNU toolchain and an up-to-date
//!
//! # Licence
//!
//! The gmpee-sys crate is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. See the full text of the [LICENSE](LICENSE.md) for details.

mod gmpmee;

pub use gmpmee::*;
