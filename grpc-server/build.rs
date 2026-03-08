use protoc_bin_vendored::protoc_bin_path;
use std::{env, error::Error};
use tonic_build::configure;

fn main() -> Result<(), Box<dyn Error>> {
    let path = protoc_bin_path()?;

    unsafe {
        env::set_var("PROTOC", path);
    }

    configure().compile(&["proto/servers.proto"], &["proto"])?;

    Ok(())
}
