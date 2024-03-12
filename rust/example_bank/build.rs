use std::io::Result;

fn main() -> Result<()> {
    let mut config = cosmos_prost_build::Config::default();
    config.prost_config.out_dir("src/types");
    config.compile_protos(
        &[
            "proto/example/bank/v1/bank.proto",
            "proto/example/escrow/v1/escrow.proto",
        ],
        &["proto/"])?;
    Ok(())
}
