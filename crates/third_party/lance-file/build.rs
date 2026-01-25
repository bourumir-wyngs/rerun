// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Lance Authors

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
    prost_build.extern_path(".lance.encodings", "::lance_encoding::format::pb");
    prost_build.compile_protos(
        &[
            "./protos/file.proto",
            "./protos/file2.proto",
            "./protos/encodings_v2_0.proto",
            "./protos/encodings_v2_1.proto",
        ],
        &["./protos"],
    )?;

    Ok(())
}
