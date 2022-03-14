/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.18
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Location : Information about the DirectLink location.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Location {
    /// The location code, to be set as the `Location` parameter of the *CreateDirectLink* method when creating a DirectLink.
    #[serde(rename = "Code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The name and description of the location, corresponding to a datacenter.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Location {
    /// Information about the DirectLink location.
    pub fn new() -> Location {
        Location {
            code: None,
            name: None,
        }
    }
}
