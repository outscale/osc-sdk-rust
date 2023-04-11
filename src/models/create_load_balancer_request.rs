/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.26
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateLoadBalancerRequest {
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// One or more listeners to create.
    #[serde(rename = "Listeners")]
    pub listeners: Vec<crate::models::ListenerForCreation>,
    /// The unique name of the load balancer (32 alphanumeric or hyphen characters maximum, but cannot start or end with a hyphen).
    #[serde(rename = "LoadBalancerName")]
    pub load_balancer_name: String,
    /// The type of load balancer: `internet-facing` or `internal`. Use this parameter only for load balancers in a Net.
    #[serde(rename = "LoadBalancerType", skip_serializing_if = "Option::is_none")]
    pub load_balancer_type: Option<String>,
    /// (internet-facing only) The public IP you want to associate with the load balancer. If not specified, a public IP owned by 3DS OUTSCALE is associated.
    #[serde(rename = "PublicIp", skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    /// (Net only) One or more IDs of security groups you want to assign to the load balancer. If not specified, the default security group of the Net is assigned to the load balancer.
    #[serde(rename = "SecurityGroups", skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// (Net only) The ID of the Subnet in which you want to create the load balancer. Regardless of this Subnet, the load balancer can distribute traffic to all Subnets. This parameter is required in a Net.
    #[serde(rename = "Subnets", skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    /// (public Cloud only) The Subregion in which you want to create the load balancer. Regardless of this Subregion, the load balancer can distribute traffic to all Subregions. This parameter is required in the public Cloud.
    #[serde(rename = "SubregionNames", skip_serializing_if = "Option::is_none")]
    pub subregion_names: Option<Vec<String>>,
    /// One or more tags assigned to the load balancer.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ResourceTag>>,
}

impl CreateLoadBalancerRequest {
    pub fn new(
        listeners: Vec<crate::models::ListenerForCreation>,
        load_balancer_name: String,
    ) -> CreateLoadBalancerRequest {
        CreateLoadBalancerRequest {
            dry_run: None,
            listeners,
            load_balancer_name,
            load_balancer_type: None,
            public_ip: None,
            security_groups: None,
            subnets: None,
            subregion_names: None,
            tags: None,
        }
    }
}
