use outscale_api::apis::profile::ProfileBuilder;
use outscale_api::apis::volume_api::read_volumes;
use outscale_api::models::ReadVolumesRequest;
use std::path::PathBuf;

/* Show how to configure SDK to load configuration file */
fn main() {
    let path = PathBuf::from("examples/config_example.json");

    let config = ProfileBuilder::from_standard_configuration(path, None)
        .and_then(|pb| pb.build().try_into())
        .unwrap();

    let request = ReadVolumesRequest::new();
    if let Err(error) = read_volumes(&config, Some(request)) {
        eprintln!("Error: {:?}", error);
        std::process::exit(1);
    }
    println!("OK");
}
