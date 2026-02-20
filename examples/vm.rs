use sdk::Profile;
use sdk::osc::{Api as _, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let profile = Profile::default().unwrap();
    let mut client = Client::new(&profile).unwrap();

    let vms = client
        .read_vms(sdk::osc::ReadVmsRequest {
            filters: Some(sdk::osc::FiltersVm {
                vm_ids: Some(vec!["i-76bcdb1d".to_string()]),
                ..Default::default()
            }),
            ..Default::default()
        })
        .await?;

    println!("response: {:?}", vms);

    Ok(())
}
