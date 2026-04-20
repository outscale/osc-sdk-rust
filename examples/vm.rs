use env_logger::Env;
use sdk::Profile;
use sdk::osc::{Api as _, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");

    env_logger::init_from_env(env);

    let profile = Profile::new().unwrap();
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
