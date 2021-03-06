/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// PermissionsOnResource : Information about the users who have permissions for the resource.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PermissionsOnResource {
    /// The account ID of one or more users who have permissions for the resource.
    #[serde(rename = "AccountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// If true, the resource is public. If false, the resource is private.
    #[serde(rename = "GlobalPermission", skip_serializing_if = "Option::is_none")]
    pub global_permission: Option<bool>,
}

impl PermissionsOnResource {
    /// Information about the users who have permissions for the resource.
    pub fn new() -> PermissionsOnResource {
        PermissionsOnResource {
            account_ids: None,
            global_permission: None,
        }
    }
}
