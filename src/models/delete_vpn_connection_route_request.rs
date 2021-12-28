/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://wiki.outscale.net/display/EN/3DS+OUTSCALE+APIs+Reference).<br /><br />  You can also manage your resources using the [Cockpit](https://wiki.outscale.net/display/EN/About+Cockpit) web interface.
 *
 * The version of the OpenAPI document: 1.16
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteVpnConnectionRouteRequest {
    /// The network prefix of the route to delete, in CIDR notation (for example, 10.12.0.0/16).
    #[serde(rename = "DestinationIpRange")]
    pub destination_ip_range: String,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The ID of the target VPN connection of the static route to delete.
    #[serde(rename = "VpnConnectionId")]
    pub vpn_connection_id: String,
}

impl DeleteVpnConnectionRouteRequest {
    pub fn new(
        destination_ip_range: String,
        vpn_connection_id: String,
    ) -> DeleteVpnConnectionRouteRequest {
        DeleteVpnConnectionRouteRequest {
            destination_ip_range,
            dry_run: None,
            vpn_connection_id,
        }
    }
}
