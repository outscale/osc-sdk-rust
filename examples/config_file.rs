use outscale_api::apis::configuration_file::{ConfigurationFile, Endpoint};
use outscale_api::apis::volume_api::read_volumes;
use outscale_api::models::ReadVolumesRequest;
use std::env;
use std::path::PathBuf;

/* Show how to configure SDK to load configuration file */
fn main() {
    // You can also use ConfigurationFile::load_default() to get configuration located in ~/.osc/config.json
    let path = PathBuf::from("examples/config_example.json");
    let mut config_file = ConfigurationFile::load(&path).unwrap();
    ignore_me(&mut config_file);

    let config = config_file.configuration("default").unwrap();
    let request = ReadVolumesRequest::new();
    if let Err(error) = read_volumes(&config, Some(request)) {
        eprintln!("Error: {:?}", error);
        std::process::exit(1);
    }
    println!("OK");
}

// Access Key and Secret Key can be put in configuration file
// but we add it here just to avoid commiting credentials in examples.
#[allow(unused_mut)]
fn ignore_me(config_file: &mut ConfigurationFile) {
    let mut profile = config_file.0.get_mut(&"default".to_string()).unwrap();
    profile.access_key = Some(env::var("OSC_ACCESS_KEY").unwrap());
    profile.secret_key = Some(env::var("OSC_SECRET_KEY").unwrap());
    profile.region = Some(env::var("OSC_REGION").unwrap());
    match env::var("OSC_PROTOCOL") {
        Ok(p) => profile.protocol = Some(p),
        _ => (),
    };
    match env::var("OSC_ENDPOINT_API_NOPROTO") {
        Ok(enpoint) => {
            profile.endpoints = Some(Endpoint {
                api: Some(enpoint),
                icu: None,
                eim: None,
                fcu: None,
                lbu: None,
                oos: None,
            })
        }
        _ => (),
    };
}
