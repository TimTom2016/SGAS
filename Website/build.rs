fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature="ssr")]
    tonic_build::compile_protos("proto/server.proto")?;
    Ok(())
}