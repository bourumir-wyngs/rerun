// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Lance Authors

use std::env;
use std::io::Result;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=protos");

    // `lance-*` build scripts require a sufficiently new `protoc` for
    // `--experimental_allow_proto3_optional`.
    // Use a prebuilt `protoc` by default to avoid relying on the system toolchain.
    if std::env::var_os("PROTOC").is_none() {
        let (protoc_bin, _include_dir) = protoc_prebuilt::init("22.0")
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;
        std::env::set_var("PROTOC", protoc_bin);
    }

    let mut prost_build = prost_build::Config::new();
    prost_build.protoc_arg("--experimental_allow_proto3_optional");
    prost_build.enable_type_names();
    prost_build.compile_protos(
        &["./protos/index.proto", "./protos/index_old.proto"],
        &["./protos"],
    )?;

    let rust_toolchain = env::var("RUSTUP_TOOLCHAIN")
        .or_else(|e| match e {
            env::VarError::NotPresent => Ok("stable".into()),
            e => Err(e),
        })
        .unwrap();
    if rust_toolchain.starts_with("nightly") {
        // enable the 'nightly' feature flag
        println!("cargo:rustc-cfg=feature=\"nightly\"");
    }

    Ok(())
}
