/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Service : Information about the service.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Service {
    /// The list of network prefixes used by the service, in CIDR notation.
    #[serde(rename = "IpRanges", skip_serializing_if = "Option::is_none")]
    pub ip_ranges: Option<Vec<String>>,
    /// The ID of the service.
    #[serde(rename = "ServiceId", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    /// The name of the service.
    #[serde(rename = "ServiceName", skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

impl Service {
    /// Information about the service.
    pub fn new() -> Service {
        Service {
            ip_ranges: None,
            service_id: None,
            service_name: None,
        }
    }
}
