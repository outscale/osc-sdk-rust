/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://wiki.outscale.net/display/EN/3DS+OUTSCALE+APIs+Reference).<br /><br />  You can also manage your resources using the [Cockpit](https://wiki.outscale.net/display/EN/About+Cockpit) web interface.
 *
 * The version of the OpenAPI document: 1.16
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
    /// The range of inside IP addresses for the tunnel. This must be a /30 CIDR block from the 169.254.254.0/24 range.
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
