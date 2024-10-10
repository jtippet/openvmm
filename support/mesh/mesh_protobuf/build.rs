// Copyright (C) Microsoft Corporation. All rights reserved.

fn main() {
    #[cfg(feature = "prost")]
    {
        prost_build::Config::new()
            .type_attribute(".", "#[derive(mesh_derive::Protobuf)]")
            .type_attribute(".", "#[mesh(prost)]")
            .compile_protos(&["src/prost.proto"], &["src/"])
            .unwrap();
    }
}