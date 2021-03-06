/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// VirtualGateway : Information about the virtual gateway.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VirtualGateway {
    /// The type of VPN connection supported by the virtual gateway (only `ipsec.1` is supported).
    #[serde(rename = "ConnectionType", skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// The Net to which the virtual gateway is attached.
    #[serde(
        rename = "NetToVirtualGatewayLinks",
        skip_serializing_if = "Option::is_none"
    )]
    pub net_to_virtual_gateway_links: Option<Vec<crate::models::NetToVirtualGatewayLink>>,
    /// The state of the virtual gateway (`pending` \\| `available` \\| `deleting` \\| `deleted`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// One or more tags associated with the virtual gateway.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ResourceTag>>,
    /// The ID of the virtual gateway.
    #[serde(rename = "VirtualGatewayId", skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
}

impl VirtualGateway {
    /// Information about the virtual gateway.
    pub fn new() -> VirtualGateway {
        VirtualGateway {
            connection_type: None,
            net_to_virtual_gateway_links: None,
            state: None,
            tags: None,
            virtual_gateway_id: None,
        }
    }
}
