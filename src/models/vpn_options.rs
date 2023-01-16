/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.24
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// VpnOptions : Information about the VPN options.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VpnOptions {
    #[serde(rename = "Phase1Options", skip_serializing_if = "Option::is_none")]
    pub phase1_options: Option<Box<crate::models::Phase1Options>>,
    #[serde(rename = "Phase2Options", skip_serializing_if = "Option::is_none")]
    pub phase2_options: Option<Box<crate::models::Phase2Options>>,
    /// The range of inside IPs for the tunnel. This must be a /30 CIDR block from the 169.254.254.0/24 range.
    #[serde(
        rename = "TunnelInsideIpRange",
        skip_serializing_if = "Option::is_none"
    )]
    pub tunnel_inside_ip_range: Option<String>,
}

impl VpnOptions {
    /// Information about the VPN options.
    pub fn new() -> VpnOptions {
        VpnOptions {
            phase1_options: None,
            phase2_options: None,
            tunnel_inside_ip_range: None,
        }
    }
}
