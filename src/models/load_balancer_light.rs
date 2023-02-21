/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.25
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// LoadBalancerLight : Information about the load balancer.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoadBalancerLight {
    /// The name of the load balancer to which the listener is attached.
    #[serde(rename = "LoadBalancerName")]
    pub load_balancer_name: String,
    /// The port of load balancer on which the load balancer is listening (between `1` and `65535` both included).
    #[serde(rename = "LoadBalancerPort")]
    pub load_balancer_port: i32,
}

impl LoadBalancerLight {
    /// Information about the load balancer.
    pub fn new(load_balancer_name: String, load_balancer_port: i32) -> LoadBalancerLight {
        LoadBalancerLight {
            load_balancer_name,
            load_balancer_port,
        }
    }
}
