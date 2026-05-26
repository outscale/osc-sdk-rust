use env_logger::Env;
use osc_sdk_rust::Profile;
#[cfg(feature = "osc")]
use osc_sdk_rust::osc::{Api as _, Client};

#[cfg(feature = "osc")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");

    env_logger::init_from_env(env);

    let profile = Profile::new().unwrap();
    let mut client = Client::new(&profile).unwrap();

    let vms = client
        .read_vms(osc_sdk_rust::osc::ReadVmsRequest {
            filters: Some(osc_sdk_rust::osc::FiltersVm {
                vm_ids: Some(vec!["i-76bcdb1d".to_string()]),
                ..Default::default()
            }),
            ..Default::default()
        })
        .await?;

    println!("response: {:?}", vms);

    Ok(())
}

#[cfg(not(feature = "osc"))]
fn main() {
    compile_error!("This example require osc feature");
}
