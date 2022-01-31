/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.17
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// PermissionsOnResourceCreation : Information about the permissions for the resource.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PermissionsOnResourceCreation {
    #[serde(rename = "Additions", skip_serializing_if = "Option::is_none")]
    pub additions: Option<Box<crate::models::PermissionsOnResource>>,
    #[serde(rename = "Removals", skip_serializing_if = "Option::is_none")]
    pub removals: Option<Box<crate::models::PermissionsOnResource>>,
}

impl PermissionsOnResourceCreation {
    /// Information about the permissions for the resource.
    pub fn new() -> PermissionsOnResourceCreation {
        PermissionsOnResourceCreation {
            additions: None,
            removals: None,
        }
    }
}
