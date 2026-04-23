use env_logger::Env;
use osc_sdk_rust::Profile;
#[cfg(feature = "oks")]
use osc_sdk_rust::oks::{Api as _, Client};

#[cfg(feature = "oks")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");

    env_logger::init_from_env(env);

    let profile = Profile::new().unwrap();
    let mut client = Client::new(&profile).unwrap();

    let list_projects_resp = client
        .list_projects(osc_sdk_rust::oks::ListProjectsParam {
            name: Some("test".to_string()),
            ..Default::default()
        })
        .await?;

    let my_test_project_id = if list_projects_resp.projects.is_empty() {
        log::info!("No projects found");

        let mut project_template = client.get_project_template().await?;
        project_template.template.name = "test".to_string();

        let create_project_resp = client.create_project(project_template.template).await?;

        let wait_job = tokio::spawn({
            let mut client = client.clone();
            let project_id = create_project_resp.project.id.clone();
            log::info!("entring waiting loop");
            async move {
                loop {
                    match client.get_project(project_id.clone()).await {
                        Err(e) => {
                            log::error!("API error: {}", e.to_string());
                            break;
                        }
                        Ok(project_resp)
                            if project_resp.project.status == "ready"
                                || project_resp.project.status == "failed" =>
                        {
                            break;
                        }
                        _ => {
                            log::info!("project still creating");
                            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        }
                    }
                }
            }
        });

        let timeout = tokio::time::sleep(tokio::time::Duration::from_mins(10));

        tokio::select! {
            _ = wait_job => {
                log::info!("Project creation job finished");
            }
            _ = timeout => {
                log::error!("Project creation timed out");
            }
        };

        create_project_resp.project.id
    } else {
        list_projects_resp.projects[0].id.clone()
    };

    client.delete_project(my_test_project_id).await?;

    Ok(())
}

#[cfg(not(feature = "oks"))]
fn main() {
    compile_error!("This example require oks feature");
}
