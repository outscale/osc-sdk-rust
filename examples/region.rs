use outscale_api::apis::configuration::{AWSv4Key, Configuration};
use outscale_api::apis::volume_api::read_volumes;
use outscale_api::models::ReadVolumesRequest;
use secrecy::SecretString;
use std::env;

/* Show how to configure SDK for a specific region */
fn main() {
    let region = "eu-west-2";

    let mut config = Configuration::new();
    config.base_path = format!("https://api.{}.outscale.com/api/v1", region);
    config.aws_v4_key = Some(AWSv4Key {
        access_key: env::var("OSC_ACCESS_KEY").unwrap(),
        secret_key: SecretString::new(env::var("OSC_SECRET_KEY").unwrap()),
        region: region.to_string(),
        service: "oapi".to_string(),
    });

    match env::var("OSC_ENDPOINT_API") {
        Ok(enpoint) => config.base_path = enpoint,
        _ => (),
    };

    print!("Action on specific region ({})... ", region);
    let request = ReadVolumesRequest::new();
    if let Err(error) = read_volumes(&config, Some(request)) {
        eprintln!("Error: {:?}", error);
        std::process::exit(1);
    }
    println!("OK");
}
