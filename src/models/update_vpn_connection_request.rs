/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateVpnConnectionRequest {
    /// The ID of the client gateway.
    #[serde(rename = "ClientGatewayId", skip_serializing_if = "Option::is_none")]
    pub client_gateway_id: Option<String>,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The ID of the virtual gateway.
    #[serde(rename = "VirtualGatewayId", skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    /// The ID of the VPN connection you want to modify.
    #[serde(rename = "VpnConnectionId")]
    pub vpn_connection_id: String,
    #[serde(rename = "VpnOptions", skip_serializing_if = "Option::is_none")]
    pub vpn_options: Option<Box<crate::models::VpnOptions>>,
}

impl UpdateVpnConnectionRequest {
    pub fn new(vpn_connection_id: String) -> UpdateVpnConnectionRequest {
        UpdateVpnConnectionRequest {
            client_gateway_id: None,
            dry_run: None,
            virtual_gateway_id: None,
            vpn_connection_id,
            vpn_options: None,
        }
    }
}
