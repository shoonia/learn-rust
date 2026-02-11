fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let path = protoc_bin_vendored::protoc_bin_path().unwrap();
        std::env::set_var("PROTOC", path);
    }

    tonic_build::configure().compile(&["proto/servers.proto"], &["proto"])?;

    Ok(())
}
