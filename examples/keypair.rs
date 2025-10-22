use outscale_api::apis::configuration::{AWSv4Key, Configuration};
use outscale_api::apis::keypair_api::{create_keypair, delete_keypair, read_keypairs};
use outscale_api::models::{
    CreateKeypairRequest, DeleteKeypairRequest, FiltersKeypair, ReadKeypairsRequest,
};
use rand::Rng;
use std::env;

fn main() {
    let mut config = Configuration::new();
    config.aws_v4_key = Some(AWSv4Key {
        access_key: env::var("OSC_ACCESS_KEY").unwrap(),
        secret_key: env::var("OSC_SECRET_KEY").unwrap().into(),
        region: "eu-west-2".to_string(),
        service: "oapi".to_string(),
    });

    match env::var("OSC_ENDPOINT_API") {
        Ok(enpoint) => config.base_path = enpoint,
        _ => (),
    };
    // Example reading all keypairs
    print!("Reading all keypairs... ");
    let request = ReadKeypairsRequest::new();
    let response = match read_keypairs(&config, Some(request)) {
        Err(error) => {
            println!("Error: {:?}", error);
            return;
        }
        Ok(resp) => resp,
    };
    if let Some(keypairs) = response.keypairs {
        println!("OK -> there are {} keypairs", keypairs.len());
    }

    // Example creating a keypair
    print!("Creating new keypair... ");
    let mut rng = rand::rng();
    let keypair_name = format!("osc-sdk-rust-test-{}", rng.random::<u64>());
    let request = CreateKeypairRequest::new(keypair_name.clone());
    match create_keypair(&config, Some(request)) {
        Err(error) => {
            println!("Error: {:?}", error);
            return;
        }
        Ok(resp) => resp,
    };
    println!("OK -> created keypair {}", keypair_name);

    // Filtering on newly created keypair
    print!("Filtering on {}... ", keypair_name);
    let mut filters = FiltersKeypair::new();
    filters.keypair_names = Some(vec![keypair_name.clone()]);
    let mut request = ReadKeypairsRequest::new();
    request.filters = Some(Box::new(filters));
    if let Err(error) = read_keypairs(&config, Some(request)) {
        eprintln!("Error: {:?}", error);
        std::process::exit(1);
    }
    println!("OK");

    // Deleting a keypair
    print!("Deleting keypair {}... ", keypair_name);
    let mut request = DeleteKeypairRequest::new();
    request.keypair_name = Some(keypair_name.clone());
    if let Err(error) = delete_keypair(&config, Some(request)) {
        eprintln!("Error: {:?}", error);
        std::process::exit(1);
    }
    println!("OK");
}
