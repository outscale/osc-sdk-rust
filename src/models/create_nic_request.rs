/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.27
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateNicRequest {
    /// A description for the NIC.
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The primary private IP for the NIC.<br /> This IP must be within the IP range of the Subnet that you specify with the `SubnetId` attribute.<br /> If you do not specify this attribute, a random private IP is selected within the IP range of the Subnet.
    #[serde(rename = "PrivateIps", skip_serializing_if = "Option::is_none")]
    pub private_ips: Option<Vec<crate::models::PrivateIpLight>>,
    /// One or more IDs of security groups for the NIC.
    #[serde(rename = "SecurityGroupIds", skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// The ID of the Subnet in which you want to create the NIC.
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
}

impl CreateNicRequest {
    pub fn new(subnet_id: String) -> CreateNicRequest {
        CreateNicRequest {
            description: None,
            dry_run: None,
            private_ips: None,
            security_group_ids: None,
            subnet_id,
        }
    }
}
