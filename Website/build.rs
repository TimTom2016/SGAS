fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature="ssr")]
    let mut config = tonic_build::configure().protoc_arg("--experimental_allow_proto3_optional");
    #[cfg(feature="ssr")]
    config.compile(&["proto/server.proto"], &["proto/"])?;
    Ok(())
}