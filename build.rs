// Copyright 2022 Sebastian Ramacher
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::env;
use std::path::PathBuf;

fn main() {
    system_deps::Config::new().probe().unwrap();

    // Invalidate the built crate whenever the wrapper and the build script changes.
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .use_core()
        .blocklist_function("dpkg_set_report_buffer")
        .blocklist_type("(_IO_)?FILE")
        .blocklist_type("_IO_.*")
        .allowlist_function("dpkg_.*")
        .rustified_enum("dpkg_relation")
        .rustified_enum("dpkg_arch_type")
        .rustified_enum("dpkg_msg_type")
        // Finish the builder and generate the bindings.
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
