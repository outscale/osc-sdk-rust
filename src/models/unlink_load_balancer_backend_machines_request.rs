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
pub struct UnlinkLoadBalancerBackendMachinesRequest {
    ///  One or more public IPs of back-end VMs.
    #[serde(rename = "BackendIps", skip_serializing_if = "Option::is_none")]
    pub backend_ips: Option<Vec<String>>,
    ///  One or more IDs of back-end VMs.
    #[serde(rename = "BackendVmIds", skip_serializing_if = "Option::is_none")]
    pub backend_vm_ids: Option<Vec<String>>,
    ///  If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    ///  The name of the load balancer.
    #[serde(rename = "LoadBalancerName")]
    pub load_balancer_name: String,
}

impl UnlinkLoadBalancerBackendMachinesRequest {
    pub fn new(load_balancer_name: String) -> UnlinkLoadBalancerBackendMachinesRequest {
        UnlinkLoadBalancerBackendMachinesRequest {
            backend_ips: None,
            backend_vm_ids: None,
            dry_run: None,
            load_balancer_name,
        }
    }
}
