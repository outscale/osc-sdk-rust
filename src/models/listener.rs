/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Listener : Information about the listener.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Listener {
    /// The port on which the back-end VM is listening (between `1` and `65535`, both included).
    #[serde(rename = "BackendPort", skip_serializing_if = "Option::is_none")]
    pub backend_port: Option<i32>,
    /// The protocol for routing traffic to back-end VMs (`HTTP` \\| `HTTPS` \\| `TCP` \\| `SSL`).
    #[serde(rename = "BackendProtocol", skip_serializing_if = "Option::is_none")]
    pub backend_protocol: Option<String>,
    /// The port on which the load balancer is listening (between `1` and `65535`, both included).
    #[serde(rename = "LoadBalancerPort", skip_serializing_if = "Option::is_none")]
    pub load_balancer_port: Option<i32>,
    /// The routing protocol (`HTTP` \\| `HTTPS` \\| `TCP` \\| `SSL`).
    #[serde(
        rename = "LoadBalancerProtocol",
        skip_serializing_if = "Option::is_none"
    )]
    pub load_balancer_protocol: Option<String>,
    /// The names of the policies. If there are no policies enabled, the list is empty.
    #[serde(rename = "PolicyNames", skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<Vec<String>>,
    /// The OUTSCALE Resource Name (ORN) of the server certificate. For more information, see [Resource Identifiers > OUTSCALE Resource Names (ORNs)](https://docs.outscale.com/en/userguide/Resource-Identifiers.html#_outscale_resource_names_orns).
    #[serde(
        rename = "ServerCertificateId",
        skip_serializing_if = "Option::is_none"
    )]
    pub server_certificate_id: Option<String>,
}

impl Listener {
    /// Information about the listener.
    pub fn new() -> Listener {
        Listener {
            backend_port: None,
            backend_protocol: None,
            load_balancer_port: None,
            load_balancer_protocol: None,
            policy_names: None,
            server_certificate_id: None,
        }
    }
}
