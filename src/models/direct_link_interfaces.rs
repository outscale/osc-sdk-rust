/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.18
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// DirectLinkInterfaces : Information about the DirectLink interfaces.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DirectLinkInterfaces {
    /// The account ID of the owner of the DirectLink interface.
    #[serde(rename = "AccountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The BGP (Border Gateway Protocol) ASN (Autonomous System Number) on the customer's side of the DirectLink interface.
    #[serde(rename = "BgpAsn", skip_serializing_if = "Option::is_none")]
    pub bgp_asn: Option<i32>,
    /// The BGP authentication key.
    #[serde(rename = "BgpKey", skip_serializing_if = "Option::is_none")]
    pub bgp_key: Option<String>,
    /// The IP on the customer's side of the DirectLink interface.
    #[serde(rename = "ClientPrivateIp", skip_serializing_if = "Option::is_none")]
    pub client_private_ip: Option<String>,
    /// The ID of the DirectLink.
    #[serde(rename = "DirectLinkId", skip_serializing_if = "Option::is_none")]
    pub direct_link_id: Option<String>,
    /// The ID of the DirectLink interface.
    #[serde(
        rename = "DirectLinkInterfaceId",
        skip_serializing_if = "Option::is_none"
    )]
    pub direct_link_interface_id: Option<String>,
    /// The name of the DirectLink interface.
    #[serde(
        rename = "DirectLinkInterfaceName",
        skip_serializing_if = "Option::is_none"
    )]
    pub direct_link_interface_name: Option<String>,
    /// The type of the DirectLink interface (always `private`).
    #[serde(rename = "InterfaceType", skip_serializing_if = "Option::is_none")]
    pub interface_type: Option<String>,
    /// The datacenter where the DirectLink interface is located.
    #[serde(rename = "Location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// The IP on the OUTSCALE side of the DirectLink interface.
    #[serde(rename = "OutscalePrivateIp", skip_serializing_if = "Option::is_none")]
    pub outscale_private_ip: Option<String>,
    /// The state of the DirectLink interface (`pending` \\| `available` \\| `deleting` \\| `deleted` \\| `confirming` \\| `rejected` \\| `expired`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The ID of the target virtual gateway.
    #[serde(rename = "VirtualGatewayId", skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    /// The VLAN number associated with the DirectLink interface.
    #[serde(rename = "Vlan", skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i32>,
}

impl DirectLinkInterfaces {
    /// Information about the DirectLink interfaces.
    pub fn new() -> DirectLinkInterfaces {
        DirectLinkInterfaces {
            account_id: None,
            bgp_asn: None,
            bgp_key: None,
            client_private_ip: None,
            direct_link_id: None,
            direct_link_interface_id: None,
            direct_link_interface_name: None,
            interface_type: None,
            location: None,
            outscale_private_ip: None,
            state: None,
            virtual_gateway_id: None,
            vlan: None,
        }
    }
}
