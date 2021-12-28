use outscale_api::apis::configuration::{AWSv4Key, Configuration};
use outscale_api::apis::volume_api::{create_volume, delete_volume, read_volumes};
use outscale_api::models::{
    CreateVolumeRequest, DeleteVolumeRequest, FiltersVolume, ReadVolumesRequest,
};
use std::env;

fn main() {
    let mut config = Configuration::new();
    config.aws_v4_key = Some(AWSv4Key {
        access_key: env::var("OSC_ACCESS_KEY").unwrap(),
        secret_key: env::var("OSC_SECRET_KEY").unwrap(),
        region: "eu-west-2".to_string(),
        service: "oapi".to_string(),
    });

    // Example reading all volumes
    print!("Reading all volumes... ");
    let request = ReadVolumesRequest::new();
    let response = match read_volumes(&config, Some(request)) {
        Err(error) => {
            println!("Error: {:?}", error);
            return;
        }
        Ok(resp) => resp,
    };
    if let Some(volumes) = response.volumes {
        println!("OK -> there are {} volumes", volumes.len());
    }

    // Example creating a volume
    print!("Creating new volume... ");
    let mut request = CreateVolumeRequest::new("eu-west-2a".to_string());
    request.volume_type = Some("gp2".to_string());
    request.size = Some(10);
    let response = match create_volume(&config, Some(request)) {
        Err(error) => {
            println!("Error: {:?}", error);
            return;
        }
        Ok(resp) => resp,
    };
    let volume_id = response.volume.unwrap().volume_id.unwrap();
    println!("OK -> created volume id {}", volume_id);

    // Filtering on newly created volume
    print!("Filtering on {}... ", volume_id);
    let mut filters = FiltersVolume::new();
    filters.volume_ids = Some(vec![volume_id.clone()]);
    let mut request = ReadVolumesRequest::new();
    request.filters = Some(Box::new(filters));
    if let Err(error) = read_volumes(&config, Some(request)) {
        println!("Error: {:?}", error);
        return;
    }
    println!("OK");

    // Deleting a volume
    print!("Deleting volume {}... ", volume_id);
    let request = DeleteVolumeRequest::new(volume_id.clone());
    if let Err(error) = delete_volume(&config, Some(request)) {
        println!("Error: {:?}", error);
        return;
    }
    println!("OK");
}
