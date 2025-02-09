// Copyright © 2017–2019 University of Malta

// Copyright © 2024 Denis Morel (modified)

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

extern crate dirs;

use std::env;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{Result as IoResult, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const GMPMEE_DIR: &str = "verificatum-gmpmee-2.1.0-c";
//const GMP_VER: (i32, i32, i32) = (2, 1, 0);

#[derive(Clone, Copy, PartialEq)]
enum Target {
    Mingw,
    Msvc,
    Other,
}

struct Environment {
    out_dir: PathBuf,
    source_dir: PathBuf,
    lib_dir: PathBuf,
    include_dir: PathBuf,
    jobs: OsString,
    target: Target,
    make_check: bool,
    cc: String,
    cflags: String,
}

fn main() {
    let src_dir = PathBuf::from(cargo_env("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(cargo_env("OUT_DIR"));

    let target = cargo_env("TARGET");

    let target = target
        .into_string()
        .expect("cannot convert environment variable TARGET into a `String`");
    let target = if target.contains("-windows-msvc") {
        panic!("MSVC is not supported");
    } else if target.contains("-windows-gnu") {
        panic!("Windows-gnu is not working now");
    } else {
        Target::Other
    };

    let make_check = there_is_env("CARGO_FEATURE_CTEST")
        || (!there_is_env("CARGO_FEATURE_CNOTEST") && cargo_env("PROFILE") == *"release");

    // Get CC, CFLAGS and HOST from `cc` crate.
    let compiler = cc::Build::new().get_compiler();
    let cc = match &compiler.cc_env() {
        cc if cc.is_empty() => compiler
            .path()
            .to_str()
            .expect("Unprintable CC")
            .to_string(),
        cc => cc.to_str().expect("Unprintable CC").to_string(),
    };
    let cflags = compiler
        .cflags_env()
        .to_str()
        .expect("Unprintable CFLAGS")
        .to_string();
    let host = String::from_utf8(
        compiler
            .to_command()
            .arg("-dumpmachine")
            .output()
            .expect("Failed to get HOST")
            .stdout,
    )
    .expect("Unprintable HOST")
    .trim_end()
    .to_string();
    if host.is_empty() {
        panic!("Failed to get HOST")
    }

    let env = Environment {
        out_dir: out_dir.clone(),
        source_dir: out_dir.join(GMPMEE_DIR),
        lib_dir: out_dir.join("lib"),
        include_dir: out_dir.join("include"),
        jobs: cargo_env("NUM_JOBS"),
        target,
        make_check,
        cc,
        cflags,
    };

    // make sure we have target directories
    create_dir_or_panic(&env.lib_dir);
    create_dir_or_panic(&env.include_dir);

    let gmpmee_ah = (
        env.lib_dir.join("libgmpmee.a"),
        env.include_dir.join("gmpmee.h"),
    );

    check_for_msvc(&env);
    check_for_windows_gnu(&env);
    remove_dir_or_panic(&env.source_dir);
    create_dir_or_panic(&env.source_dir);

    copy_dir_or_panic(&src_dir.join(GMPMEE_DIR), &env.source_dir);

    build_gmpmee(&env, &gmpmee_ah.0, &gmpmee_ah.1);

    if !there_is_env("CARGO_FEATURE_CNODELETE") {
        remove_dir_or_panic(&env.source_dir);
    }

    write_link_info(&env);
}

fn build_gmpmee(env: &Environment, lib: &Path, header: &Path) {
    println!("$ cd {:?}", &env.source_dir);
    println!("$ export CC={:?}", &env.cc);
    println!("$ export CFLAGS={:?}", &env.cflags);
    println!("$ cd {:?}", env.source_dir);
    let test_cmd = Command::new("env");
    execute(test_cmd);
    let mut create_conf = Command::new("sh");
    create_conf
        .current_dir(&env.source_dir)
        .arg("-c")
        .arg("make -f Makefile.build")
        .env("CC", &env.cc)
        .env("CFLAGS", &env.cflags);
    execute(create_conf);
    let conf = "./configure \
         --disable-shared \
         --with-pic \
         "
    .to_string();
    let mut configure = Command::new("sh");
    configure
        .current_dir(&env.source_dir)
        .arg("-c")
        .arg(conf)
        .env("CC", &env.cc)
        .env("CFLAGS", &env.cflags);
    execute(configure);
    make_and_check(env, &env.source_dir);
    let build_lib = env.source_dir.join(".libs").join("libgmpmee.a");
    copy_file_or_panic(&build_lib, lib);
    let build_header = env.source_dir.join("gmpmee.h");
    copy_file_or_panic(&build_header, header);
}

fn write_link_info(env: &Environment) {
    let out_str = env.out_dir.to_str().unwrap_or_else(|| {
        panic!(
            "Path contains unsupported characters, can only make {}",
            env.out_dir.display()
        )
    });
    let lib_str = env.lib_dir.to_str().unwrap_or_else(|| {
        panic!(
            "Path contains unsupported characters, can only make {}",
            env.lib_dir.display()
        )
    });
    let include_str = env.include_dir.to_str().unwrap_or_else(|| {
        panic!(
            "Path contains unsupported characters, can only make {}",
            env.include_dir.display()
        )
    });
    println!("cargo:out_dir={}", out_str);
    println!("cargo:lib_dir={}", lib_str);
    println!("cargo:include_dir={}", include_str);
    println!("cargo:rustc-link-search=native={}", lib_str);
    println!("cargo:rustc-link-lib=static=gmpmee");
}

fn cargo_env(name: &str) -> OsString {
    env::var_os(name)
        .unwrap_or_else(|| panic!("environment variable not found: {}, please use cargo", name))
}

fn there_is_env(name: &str) -> bool {
    env::var_os(name).is_some()
}

fn check_for_msvc(env: &Environment) {
    if env.target == Target::Msvc {
        panic!("Windows MSVC target is not supported (linking would fail)");
    }
}

fn check_for_windows_gnu(env: &Environment) {
    if env.target == Target::Mingw {
        panic!("Windows Mingw target is not supported (linking would fail)");
    }
}

fn remove_dir(dir: &Path) -> IoResult<()> {
    if !dir.exists() {
        return Ok(());
    }
    assert!(dir.is_dir(), "Not a directory: {:?}", dir);
    println!("$ rm -r {:?}", dir);
    fs::remove_dir_all(dir)
}

fn remove_dir_or_panic(dir: &Path) {
    remove_dir(dir).unwrap_or_else(|_| panic!("Unable to remove directory: {:?}", dir));
}

fn create_dir(dir: &Path) -> IoResult<()> {
    println!("$ mkdir -p {:?}", dir);
    fs::create_dir_all(dir)
}

fn create_dir_or_panic(dir: &Path) {
    create_dir(dir).unwrap_or_else(|_| panic!("Unable to create directory: {:?}", dir));
}

#[allow(dead_code)]
fn create_file_or_panic(filename: &Path, contents: &str) {
    println!("$ printf '%s' {:?}... > {:?}", &contents[0..10], filename);
    let mut file =
        File::create(filename).unwrap_or_else(|_| panic!("Unable to create file: {:?}", filename));
    file.write_all(contents.as_bytes())
        .unwrap_or_else(|_| panic!("Unable to write to file: {:?}", filename));
}

fn copy_file(src: &Path, dst: &Path) -> IoResult<u64> {
    println!("$ cp {:?} {:?}", src, dst);
    fs::copy(src, dst)
}

fn copy_file_or_panic(src: &Path, dst: &Path) {
    copy_file(src, dst).unwrap_or_else(|_| {
        panic!("Unable to copy {:?} -> {:?}", src, dst);
    });
}

fn copy_dir(src: &Path, dst: &Path) -> fs_extra::error::Result<u64> {
    println!("$ cp -r {:?} {:?}", src, dst);
    let copy_options = fs_extra::dir::CopyOptions::new().overwrite(true);
    fs_extra::dir::copy(src, dst.parent().unwrap(), &copy_options)
}

fn copy_dir_or_panic(src: &Path, dst: &Path) {
    copy_dir(src, dst).unwrap_or_else(|e| {
        panic!("Unable to copy {:?} -> {:?}: {}", src, dst, e);
    });
}

fn make_and_check(env: &Environment, build_dir: &Path) {
    let mut make = Command::new("make");
    make.current_dir(build_dir).arg("-j").arg(&env.jobs);
    execute(make);
    if env.make_check {
        let mut make_check = Command::new("make");
        make_check
            .current_dir(build_dir)
            .arg("-j")
            .arg(&env.jobs)
            .arg("check");
        execute(make_check);
    }
}

fn execute(mut command: Command) {
    println!("$ {:?}", command);
    let status = command
        .status()
        .unwrap_or_else(|_| panic!("Unable to execute: {:?}", command));
    if !status.success() {
        if let Some(code) = status.code() {
            panic!("Program failed with code {}: {:?}", code, command);
        } else {
            panic!("Program failed: {:?}", command);
        }
    }
}
