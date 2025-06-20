// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=include/yyjson/*");
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=CFLAGS");
    println!("cargo:rerun-if-env-changed=LDFLAGS");
    println!("cargo:rerun-if-env-changed=ORJSON_DISABLE_YYJSON");
    println!("cargo:rerun-if-env-changed=RUSTFLAGS");
    println!("cargo:rustc-check-cfg=cfg(intrinsics)");
    println!("cargo:rustc-check-cfg=cfg(optimize)");
    println!("cargo:rustc-check-cfg=cfg(Py_3_10)");
    println!("cargo:rustc-check-cfg=cfg(Py_3_11)");
    println!("cargo:rustc-check-cfg=cfg(Py_3_12)");
    println!("cargo:rustc-check-cfg=cfg(Py_3_13)");
    println!("cargo:rustc-check-cfg=cfg(Py_3_14)");
    println!("cargo:rustc-check-cfg=cfg(Py_3_15)");
    println!("cargo:rustc-check-cfg=cfg(Py_3_9)");
    println!("cargo:rustc-check-cfg=cfg(Py_GIL_DISABLED)");

    let python_config = pyo3_build_config::get();
    for cfg in python_config.build_script_outputs() {
        println!("{cfg}");
    }

    #[allow(unused_variables)]
    let is_64_bit_python = matches!(python_config.pointer_width, Some(64));

    if version_check::supports_feature("core_intrinsics").unwrap_or(false) {
        println!("cargo:rustc-cfg=feature=\"intrinsics\"");
    }

    if version_check::supports_feature("optimize_attribute").unwrap_or(false) {
        println!("cargo:rustc-cfg=feature=\"optimize\"");
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    if is_64_bit_python {
        println!("cargo:rustc-cfg=feature=\"inline_int\"");
    }

    if env::var("ORJSON_DISABLE_YYJSON").is_ok() {
        if env::var("CARGO_FEATURE_YYJSON").is_ok() {
            panic!("ORJSON_DISABLE_YYJSON and --features=yyjson both enabled.")
        }
    } else {
        match cc::Build::new()
            .file("include/yyjson/yyjson.c")
            .include("include/yyjson")
            .define("YYJSON_DISABLE_NON_STANDARD", "1")
            .define("YYJSON_DISABLE_UTF8_VALIDATION", "1")
            .define("YYJSON_DISABLE_UTILS", "1")
            .define("YYJSON_DISABLE_WRITER", "1")
            .try_compile("yyjson")
        {
            Ok(_) => {
                println!("cargo:rustc-cfg=feature=\"yyjson\"");
            }
            Err(_) => {
                if env::var("CARGO_FEATURE_YYJSON").is_ok() {
                    panic!(
                        "yyjson was enabled but the build failed. To build with a different backend do not specify the feature."
                    )
                }
            }
        }
    }
}
