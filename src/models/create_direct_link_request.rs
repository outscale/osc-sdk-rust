/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateDirectLinkRequest {
    /// The bandwidth of the DirectLink (`1Gbps` \\| `10Gbps`).
    #[serde(rename = "Bandwidth")]
    pub bandwidth: String,
    /// The name of the DirectLink.
    #[serde(rename = "DirectLinkName")]
    pub direct_link_name: String,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The code of the requested location for the DirectLink, returned by the [ReadLocations](#readlocations) method.
    #[serde(rename = "Location")]
    pub location: String,
}

impl CreateDirectLinkRequest {
    pub fn new(
        bandwidth: String,
        direct_link_name: String,
        location: String,
    ) -> CreateDirectLinkRequest {
        CreateDirectLinkRequest {
            bandwidth,
            direct_link_name,
            dry_run: None,
            location,
        }
    }
}
