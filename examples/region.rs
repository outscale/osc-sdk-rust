use outscale_api::apis::profile::ProfileBuilder;
use outscale_api::apis::volume_api::read_volumes;
use outscale_api::models::ReadVolumesRequest;

/* Show how to configure SDK for a specific region */
fn main() {
    let region = "eu-west-2";

    let config = ProfileBuilder::from_standard_configuration(None, None)
        .and_then(|pb| pb.region(region).build().try_into())
        .unwrap();

    print!("Action on specific region ({})... ", region);
    let request = ReadVolumesRequest::new();
    if let Err(error) = read_volumes(&config, Some(request)) {
        eprintln!("Error: {:?}", error);
        std::process::exit(1);
    }
    println!("OK");
}
