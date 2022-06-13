/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// StateComment : Information about the change of state.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StateComment {
    /// The code of the change of state.
    #[serde(rename = "StateCode", skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
    /// A message explaining the change of state.
    #[serde(rename = "StateMessage", skip_serializing_if = "Option::is_none")]
    pub state_message: Option<String>,
}

impl StateComment {
    /// Information about the change of state.
    pub fn new() -> StateComment {
        StateComment {
            state_code: None,
            state_message: None,
        }
    }
}
