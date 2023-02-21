/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.25
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`create_nic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNicError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_nic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteNicError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`link_nic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LinkNicError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`link_private_ips`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LinkPrivateIpsError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`read_nics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadNicsError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unlink_nic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnlinkNicError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unlink_private_ips`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnlinkPrivateIpsError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_nic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateNicError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

pub fn create_nic(
    configuration: &configuration::Configuration,
    create_nic_request: Option<crate::models::CreateNicRequest>,
) -> Result<crate::models::CreateNicResponse, Error<CreateNicError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/CreateNic", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = match local_var_aws_v4_key.sign(
            &local_var_uri_str,
            "POST",
            &serde_json::to_string(&create_nic_request).expect("param should serialize to string"),
        ) {
            Ok(new_headers) => new_headers,
            Err(err) => return Err(Error::AWSV4SignatureError(err)),
        };
        for (local_var_name, local_var_value) in local_var_new_headers.iter() {
            local_var_req_builder =
                local_var_req_builder.header(local_var_name.as_str(), local_var_value.as_str());
        }
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&create_nic_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateNicError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn delete_nic(
    configuration: &configuration::Configuration,
    delete_nic_request: Option<crate::models::DeleteNicRequest>,
) -> Result<crate::models::DeleteNicResponse, Error<DeleteNicError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/DeleteNic", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = match local_var_aws_v4_key.sign(
            &local_var_uri_str,
            "POST",
            &serde_json::to_string(&delete_nic_request).expect("param should serialize to string"),
        ) {
            Ok(new_headers) => new_headers,
            Err(err) => return Err(Error::AWSV4SignatureError(err)),
        };
        for (local_var_name, local_var_value) in local_var_new_headers.iter() {
            local_var_req_builder =
                local_var_req_builder.header(local_var_name.as_str(), local_var_value.as_str());
        }
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&delete_nic_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteNicError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn link_nic(
    configuration: &configuration::Configuration,
    link_nic_request: Option<crate::models::LinkNicRequest>,
) -> Result<crate::models::LinkNicResponse, Error<LinkNicError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/LinkNic", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = match local_var_aws_v4_key.sign(
            &local_var_uri_str,
            "POST",
            &serde_json::to_string(&link_nic_request).expect("param should serialize to string"),
        ) {
            Ok(new_headers) => new_headers,
            Err(err) => return Err(Error::AWSV4SignatureError(err)),
        };
        for (local_var_name, local_var_value) in local_var_new_headers.iter() {
            local_var_req_builder =
                local_var_req_builder.header(local_var_name.as_str(), local_var_value.as_str());
        }
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&link_nic_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<LinkNicError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn link_private_ips(
    configuration: &configuration::Configuration,
    link_private_ips_request: Option<crate::models::LinkPrivateIpsRequest>,
) -> Result<crate::models::LinkPrivateIpsResponse, Error<LinkPrivateIpsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/LinkPrivateIps", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = match local_var_aws_v4_key.sign(
            &local_var_uri_str,
            "POST",
            &serde_json::to_string(&link_private_ips_request)
                .expect("param should serialize to string"),
        ) {
            Ok(new_headers) => new_headers,
            Err(err) => return Err(Error::AWSV4SignatureError(err)),
        };
        for (local_var_name, local_var_value) in local_var_new_headers.iter() {
            local_var_req_builder =
                local_var_req_builder.header(local_var_name.as_str(), local_var_value.as_str());
        }
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&link_private_ips_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<LinkPrivateIpsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn read_nics(
    configuration: &configuration::Configuration,
    read_nics_request: Option<crate::models::ReadNicsRequest>,
) -> Result<crate::models::ReadNicsResponse, Error<ReadNicsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ReadNics", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = match local_var_aws_v4_key.sign(
            &local_var_uri_str,
            "POST",
            &serde_json::to_string(&read_nics_request).expect("param should serialize to string"),
        ) {
            Ok(new_headers) => new_headers,
            Err(err) => return Err(Error::AWSV4SignatureError(err)),
        };
        for (local_var_name, local_var_value) in local_var_new_headers.iter() {
            local_var_req_builder =
                local_var_req_builder.header(local_var_name.as_str(), local_var_value.as_str());
        }
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&read_nics_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReadNicsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn unlink_nic(
    configuration: &configuration::Configuration,
    unlink_nic_request: Option<crate::models::UnlinkNicRequest>,
) -> Result<crate::models::UnlinkNicResponse, Error<UnlinkNicError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/UnlinkNic", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = match local_var_aws_v4_key.sign(
            &local_var_uri_str,
            "POST",
            &serde_json::to_string(&unlink_nic_request).expect("param should serialize to string"),
        ) {
            Ok(new_headers) => new_headers,
            Err(err) => return Err(Error::AWSV4SignatureError(err)),
        };
        for (local_var_name, local_var_value) in local_var_new_headers.iter() {
            local_var_req_builder =
                local_var_req_builder.header(local_var_name.as_str(), local_var_value.as_str());
        }
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&unlink_nic_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UnlinkNicError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn unlink_private_ips(
    configuration: &configuration::Configuration,
    unlink_private_ips_request: Option<crate::models::UnlinkPrivateIpsRequest>,
) -> Result<crate::models::UnlinkPrivateIpsResponse, Error<UnlinkPrivateIpsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/UnlinkPrivateIps", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = match local_var_aws_v4_key.sign(
            &local_var_uri_str,
            "POST",
            &serde_json::to_string(&unlink_private_ips_request)
                .expect("param should serialize to string"),
        ) {
            Ok(new_headers) => new_headers,
            Err(err) => return Err(Error::AWSV4SignatureError(err)),
        };
        for (local_var_name, local_var_value) in local_var_new_headers.iter() {
            local_var_req_builder =
                local_var_req_builder.header(local_var_name.as_str(), local_var_value.as_str());
        }
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&unlink_private_ips_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UnlinkPrivateIpsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn update_nic(
    configuration: &configuration::Configuration,
    update_nic_request: Option<crate::models::UpdateNicRequest>,
) -> Result<crate::models::UpdateNicResponse, Error<UpdateNicError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/UpdateNic", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = match local_var_aws_v4_key.sign(
            &local_var_uri_str,
            "POST",
            &serde_json::to_string(&update_nic_request).expect("param should serialize to string"),
        ) {
            Ok(new_headers) => new_headers,
            Err(err) => return Err(Error::AWSV4SignatureError(err)),
        };
        for (local_var_name, local_var_value) in local_var_new_headers.iter() {
            local_var_req_builder =
                local_var_req_builder.header(local_var_name.as_str(), local_var_value.as_str());
        }
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&update_nic_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateNicError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
