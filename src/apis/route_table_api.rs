/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.18
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`create_route_table`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRouteTableError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_route_table`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRouteTableError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`link_route_table`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LinkRouteTableError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`read_route_tables`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadRouteTablesError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unlink_route_table`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnlinkRouteTableError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status500(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

pub fn create_route_table(
    configuration: &configuration::Configuration,
    create_route_table_request: Option<crate::models::CreateRouteTableRequest>,
) -> Result<crate::models::CreateRouteTableResponse, Error<CreateRouteTableError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/CreateRouteTable", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = local_var_aws_v4_key.sign(
            &local_var_uri_str,
            "POST",
            &serde_json::to_string(&create_route_table_request).unwrap(),
        );
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
    local_var_req_builder = local_var_req_builder.json(&create_route_table_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateRouteTableError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn delete_route_table(
    configuration: &configuration::Configuration,
    delete_route_table_request: Option<crate::models::DeleteRouteTableRequest>,
) -> Result<crate::models::DeleteRouteTableResponse, Error<DeleteRouteTableError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/DeleteRouteTable", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = local_var_aws_v4_key.sign(
            &local_var_uri_str,
            "POST",
            &serde_json::to_string(&delete_route_table_request).unwrap(),
        );
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
    local_var_req_builder = local_var_req_builder.json(&delete_route_table_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteRouteTableError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn link_route_table(
    configuration: &configuration::Configuration,
    link_route_table_request: Option<crate::models::LinkRouteTableRequest>,
) -> Result<crate::models::LinkRouteTableResponse, Error<LinkRouteTableError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/LinkRouteTable", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = local_var_aws_v4_key.sign(
            &local_var_uri_str,
            "POST",
            &serde_json::to_string(&link_route_table_request).unwrap(),
        );
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
    local_var_req_builder = local_var_req_builder.json(&link_route_table_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<LinkRouteTableError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn read_route_tables(
    configuration: &configuration::Configuration,
    read_route_tables_request: Option<crate::models::ReadRouteTablesRequest>,
) -> Result<crate::models::ReadRouteTablesResponse, Error<ReadRouteTablesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ReadRouteTables", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = local_var_aws_v4_key.sign(
            &local_var_uri_str,
            "POST",
            &serde_json::to_string(&read_route_tables_request).unwrap(),
        );
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
    local_var_req_builder = local_var_req_builder.json(&read_route_tables_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReadRouteTablesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub fn unlink_route_table(
    configuration: &configuration::Configuration,
    unlink_route_table_request: Option<crate::models::UnlinkRouteTableRequest>,
) -> Result<crate::models::UnlinkRouteTableResponse, Error<UnlinkRouteTableError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/UnlinkRouteTable", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_aws_v4_key) = local_var_configuration.aws_v4_key {
        let local_var_new_headers = local_var_aws_v4_key.sign(
            &local_var_uri_str,
            "POST",
            &serde_json::to_string(&unlink_route_table_request).unwrap(),
        );
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
    local_var_req_builder = local_var_req_builder.json(&unlink_route_table_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UnlinkRouteTableError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
