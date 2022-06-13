/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Route : Information about the route.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Route {
    /// The method used to create the route.
    #[serde(rename = "CreationMethod", skip_serializing_if = "Option::is_none")]
    pub creation_method: Option<String>,
    /// The IP range used for the destination match, in CIDR notation (for example, `10.0.0.0/24`).
    #[serde(rename = "DestinationIpRange", skip_serializing_if = "Option::is_none")]
    pub destination_ip_range: Option<String>,
    /// The ID of the OUTSCALE service.
    #[serde(
        rename = "DestinationServiceId",
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_service_id: Option<String>,
    /// The ID of the Internet service or virtual gateway attached to the Net.
    #[serde(rename = "GatewayId", skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    /// The ID of a NAT service attached to the Net.
    #[serde(rename = "NatServiceId", skip_serializing_if = "Option::is_none")]
    pub nat_service_id: Option<String>,
    /// The ID of the Net access point.
    #[serde(rename = "NetAccessPointId", skip_serializing_if = "Option::is_none")]
    pub net_access_point_id: Option<String>,
    /// The ID of the Net peering connection.
    #[serde(rename = "NetPeeringId", skip_serializing_if = "Option::is_none")]
    pub net_peering_id: Option<String>,
    /// The ID of the NIC.
    #[serde(rename = "NicId", skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
    /// The state of a route in the route table (`active` \\| `blackhole`). The `blackhole` state indicates that the target of the route is not available.
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The account ID of the owner of the VM.
    #[serde(rename = "VmAccountId", skip_serializing_if = "Option::is_none")]
    pub vm_account_id: Option<String>,
    /// The ID of a VM specified in a route in the table.
    #[serde(rename = "VmId", skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
}

impl Route {
    /// Information about the route.
    pub fn new() -> Route {
        Route {
            creation_method: None,
            destination_ip_range: None,
            destination_service_id: None,
            gateway_id: None,
            nat_service_id: None,
            net_access_point_id: None,
            net_peering_id: None,
            nic_id: None,
            state: None,
            vm_account_id: None,
            vm_id: None,
        }
    }
}
