// build.rs
//
// This build script compiles the Protocol Buffers for the gRPC service.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        // Using the default OUT_DIR for generated code.
        .build_server(true)
        .compile(&["proto/rediodb.proto"], &["proto"])?;
    Ok(())
}
