/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.22
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// SecurityGroupsMember : Information about the member of a security group.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityGroupsMember {
    /// The account ID of a user.
    #[serde(rename = "AccountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The ID of the security group.
    #[serde(rename = "SecurityGroupId", skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// The name of the security group.
    #[serde(rename = "SecurityGroupName", skip_serializing_if = "Option::is_none")]
    pub security_group_name: Option<String>,
}

impl SecurityGroupsMember {
    /// Information about the member of a security group.
    pub fn new() -> SecurityGroupsMember {
        SecurityGroupsMember {
            account_id: None,
            security_group_id: None,
            security_group_name: None,
        }
    }
}
