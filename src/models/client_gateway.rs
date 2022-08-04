/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.21
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// ClientGateway : Information about the client gateway.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ClientGateway {
    /// The Autonomous System Number (ASN) used by the Border Gateway Protocol (BGP) to find the path to your client gateway through the Internet.
    #[serde(rename = "BgpAsn", skip_serializing_if = "Option::is_none")]
    pub bgp_asn: Option<i32>,
    /// The ID of the client gateway.
    #[serde(rename = "ClientGatewayId", skip_serializing_if = "Option::is_none")]
    pub client_gateway_id: Option<String>,
    /// The type of communication tunnel used by the client gateway (only `ipsec.1` is supported).
    #[serde(rename = "ConnectionType", skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// The public IPv4 address of the client gateway (must be a fixed address into a NATed network).
    #[serde(rename = "PublicIp", skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    /// The state of the client gateway (`pending` \\| `available` \\| `deleting` \\| `deleted`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// One or more tags associated with the client gateway.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ResourceTag>>,
}

impl ClientGateway {
    /// Information about the client gateway.
    pub fn new() -> ClientGateway {
        ClientGateway {
            bgp_asn: None,
            client_gateway_id: None,
            connection_type: None,
            public_ip: None,
            state: None,
            tags: None,
        }
    }
}
