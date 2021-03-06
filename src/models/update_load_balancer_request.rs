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
pub struct UpdateLoadBalancerRequest {
    #[serde(rename = "AccessLog", skip_serializing_if = "Option::is_none")]
    pub access_log: Option<Box<crate::models::AccessLog>>,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "HealthCheck", skip_serializing_if = "Option::is_none")]
    pub health_check: Option<Box<crate::models::HealthCheck>>,
    /// The name of the load balancer.
    #[serde(rename = "LoadBalancerName")]
    pub load_balancer_name: String,
    /// The port on which the load balancer is listening (between `1` and `65535`, both included). This parameter is required if you want to update the server certificate.
    #[serde(rename = "LoadBalancerPort", skip_serializing_if = "Option::is_none")]
    pub load_balancer_port: Option<i32>,
    /// The name of the policy you want to enable for the listener.
    #[serde(rename = "PolicyNames", skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<Vec<String>>,
    /// (internet-facing only) The public IP you want to associate with the load balancer. The former public IP of the load balancer is then disassociated. If you specify an empty string and the former public IP belonged to you, it is disassociated and replaced by a public IP owned by 3DS OUTSCALE.
    #[serde(rename = "PublicIp", skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    /// (Net only) One or more IDs of security groups you want to assign to the load balancer. You need to specify the already assigned security groups that you want to keep along with the new ones you are assigning. If the list is empty, the default security group of the Net is assigned to the load balancer.
    #[serde(rename = "SecurityGroups", skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// The Outscale Resource Name (ORN) of the server certificate. For more information, see [Resource Identifiers > Outscale Resource Names (ORNs)](https://docs.outscale.com/en/userguide/Resource-Identifiers.html#_outscale_resource_names_orns). If this parameter is specified, you must also specify the `LoadBalancerPort` parameter.
    #[serde(
        rename = "ServerCertificateId",
        skip_serializing_if = "Option::is_none"
    )]
    pub server_certificate_id: Option<String>,
}

impl UpdateLoadBalancerRequest {
    pub fn new(load_balancer_name: String) -> UpdateLoadBalancerRequest {
        UpdateLoadBalancerRequest {
            access_log: None,
            dry_run: None,
            health_check: None,
            load_balancer_name,
            load_balancer_port: None,
            policy_names: None,
            public_ip: None,
            security_groups: None,
            server_certificate_id: None,
        }
    }
}
