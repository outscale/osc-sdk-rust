/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.18
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// NetToVirtualGatewayLink : Information about the attachment.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetToVirtualGatewayLink {
    /// The ID of the Net to which the virtual gateway is attached.
    #[serde(rename = "NetId", skip_serializing_if = "Option::is_none")]
    pub net_id: Option<String>,
    /// The state of the attachment (`attaching` \\| `attached` \\| `detaching` \\| `detached`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl NetToVirtualGatewayLink {
    /// Information about the attachment.
    pub fn new() -> NetToVirtualGatewayLink {
        NetToVirtualGatewayLink {
            net_id: None,
            state: None,
        }
    }
}
