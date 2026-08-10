use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = PathBuf::from("../../proto");
    let protos: Vec<PathBuf> = std::fs::read_dir(&proto_dir)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().map(|e| e == "proto").unwrap_or(false))
        .collect();

    tonic_prost_build::configure()
        .build_server(true)
        .file_descriptor_set_path(
            PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"))
                .join("robot_descriptor.bin"),
        )
        .compile_protos(&protos, &[proto_dir])?;
    Ok(())
}
