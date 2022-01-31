/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.17
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateRouteRequest {
    /// The IP range used for the destination match, in CIDR notation (for example, 10.0.0.0/24).
    #[serde(rename = "DestinationIpRange")]
    pub destination_ip_range: String,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The ID of an Internet service or virtual gateway attached to your Net.
    #[serde(rename = "GatewayId", skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    /// The ID of a NAT service.
    #[serde(rename = "NatServiceId", skip_serializing_if = "Option::is_none")]
    pub nat_service_id: Option<String>,
    /// The ID of a Net peering connection.
    #[serde(rename = "NetPeeringId", skip_serializing_if = "Option::is_none")]
    pub net_peering_id: Option<String>,
    /// The ID of a NIC.
    #[serde(rename = "NicId", skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
    /// The ID of the route table for which you want to create a route.
    #[serde(rename = "RouteTableId")]
    pub route_table_id: String,
    /// The ID of a NAT VM in your Net (attached to exactly one NIC).
    #[serde(rename = "VmId", skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
}

impl CreateRouteRequest {
    pub fn new(destination_ip_range: String, route_table_id: String) -> CreateRouteRequest {
        CreateRouteRequest {
            destination_ip_range,
            dry_run: None,
            gateway_id: None,
            nat_service_id: None,
            net_peering_id: None,
            nic_id: None,
            route_table_id,
            vm_id: None,
        }
    }
}
