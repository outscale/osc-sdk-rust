/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.21
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// NetPeeringState : Information about the state of the Net peering connection.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetPeeringState {
    /// Additional information about the state of the Net peering connection.
    #[serde(rename = "Message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The state of the Net peering connection (`pending-acceptance` \\| `active` \\| `rejected` \\| `failed` \\| `expired` \\| `deleted`).
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl NetPeeringState {
    /// Information about the state of the Net peering connection.
    pub fn new() -> NetPeeringState {
        NetPeeringState {
            message: None,
            name: None,
        }
    }
}
