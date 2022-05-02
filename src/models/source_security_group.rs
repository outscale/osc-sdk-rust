/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.19
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// SourceSecurityGroup : Information about the source security group of the load balancer, which you can use as part of your inbound rules for your registered VMs.<br /> To only allow traffic from load balancers, add a security group rule that specifies this source security group as the inbound source.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SourceSecurityGroup {
    /// The account ID of the owner of the security group.
    #[serde(
        rename = "SecurityGroupAccountId",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_group_account_id: Option<String>,
    /// The name of the security group.
    #[serde(rename = "SecurityGroupName", skip_serializing_if = "Option::is_none")]
    pub security_group_name: Option<String>,
}

impl SourceSecurityGroup {
    /// Information about the source security group of the load balancer, which you can use as part of your inbound rules for your registered VMs.<br /> To only allow traffic from load balancers, add a security group rule that specifies this source security group as the inbound source.
    pub fn new() -> SourceSecurityGroup {
        SourceSecurityGroup {
            security_group_account_id: None,
            security_group_name: None,
        }
    }
}
