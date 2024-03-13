use std::io::Result;

fn main() -> Result<()> {
    prost_build::Config::default()
        .out_dir("src")
        .include_file("_includes.rs")
        .compile_protos(
            &[
                "../../proto/cosmos/msg/v1/msg.proto",
            ],
            &["../../proto/"])
}
