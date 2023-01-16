/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.24
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Log : Information about the log.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Log {
    /// The account ID of the logged call.
    #[serde(rename = "AccountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The duration of the logged call, in microseconds.
    #[serde(rename = "CallDuration", skip_serializing_if = "Option::is_none")]
    pub call_duration: Option<i32>,
    /// The access key used for the logged call.
    #[serde(rename = "QueryAccessKey", skip_serializing_if = "Option::is_none")]
    pub query_access_key: Option<String>,
    /// The name of the API used by the logged call (always `oapi` for the OUTSCALE API).
    #[serde(rename = "QueryApiName", skip_serializing_if = "Option::is_none")]
    pub query_api_name: Option<String>,
    /// The version of the API used by the logged call.
    #[serde(rename = "QueryApiVersion", skip_serializing_if = "Option::is_none")]
    pub query_api_version: Option<String>,
    /// The name of the logged call.
    #[serde(rename = "QueryCallName", skip_serializing_if = "Option::is_none")]
    pub query_call_name: Option<String>,
    /// The date of the logged call, in ISO 8601 format.
    #[serde(rename = "QueryDate", skip_serializing_if = "Option::is_none")]
    pub query_date: Option<String>,
    /// The raw header of the HTTP request of the logged call.
    #[serde(rename = "QueryHeaderRaw", skip_serializing_if = "Option::is_none")]
    pub query_header_raw: Option<String>,
    /// The size of the raw header of the HTTP request of the logged call, in bytes.
    #[serde(rename = "QueryHeaderSize", skip_serializing_if = "Option::is_none")]
    pub query_header_size: Option<i32>,
    /// The IP used for the logged call.
    #[serde(rename = "QueryIpAddress", skip_serializing_if = "Option::is_none")]
    pub query_ip_address: Option<String>,
    /// The raw payload of the HTTP request of the logged call.
    #[serde(rename = "QueryPayloadRaw", skip_serializing_if = "Option::is_none")]
    pub query_payload_raw: Option<String>,
    /// The size of the raw payload of the HTTP request of the logged call, in bytes.
    #[serde(rename = "QueryPayloadSize", skip_serializing_if = "Option::is_none")]
    pub query_payload_size: Option<i32>,
    /// The user agent of the HTTP request of the logged call.
    #[serde(rename = "QueryUserAgent", skip_serializing_if = "Option::is_none")]
    pub query_user_agent: Option<String>,
    /// The request ID provided in the response of the logged call.
    #[serde(rename = "RequestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The size of the response of the logged call, in bytes.
    #[serde(rename = "ResponseSize", skip_serializing_if = "Option::is_none")]
    pub response_size: Option<i32>,
    /// The HTTP status code of the response of the logged call.
    #[serde(rename = "ResponseStatusCode", skip_serializing_if = "Option::is_none")]
    pub response_status_code: Option<i32>,
}

impl Log {
    /// Information about the log.
    pub fn new() -> Log {
        Log {
            account_id: None,
            call_duration: None,
            query_access_key: None,
            query_api_name: None,
            query_api_version: None,
            query_call_name: None,
            query_date: None,
            query_header_raw: None,
            query_header_size: None,
            query_ip_address: None,
            query_payload_raw: None,
            query_payload_size: None,
            query_user_agent: None,
            request_id: None,
            response_size: None,
            response_status_code: None,
        }
    }
}
