/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> Throttling: To protect against overloads, the number of identical requests allowed in a given time period is limited.<br /> Brute force: To protect against brute force attacks, the number of failed authentication attempts in a given time period is limited.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).<br /> # Authentication Schemes ### Access Key/Secret Key The main way to authenticate your requests to the OUTSCALE API is to use an access key and a secret key.<br /> The mechanism behind this is based on AWS Signature Version 4, whose technical implementation details are described in [Signature of API Requests](https://docs.outscale.com/en/userguide/Signature-of-API-Requests.html).<br /><br /> In practice, the way to specify your access key and secret key depends on the tool or SDK you want to use to interact with the API.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify your access key, secret key, and the Region of your account. > 2. You then specify the `--profile` option when executing OSC CLI commands. >  > For more information, see [Installing and Configuring OSC CLI](https://docs.outscale.com/en/userguide/Installing-and-Configuring-OSC-CLI.html).  See the code samples in each section of this documentation for specific examples in different programming languages.<br /> For more information about access keys, see [About Access Keys](https://docs.outscale.com/en/userguide/About-Access-Keys.html). ### Login/Password For certain API actions, you can also use basic authentication with the login (email address) and password of your TINA account.<br /> This is useful only in special circumstances, for example if you do not know your access key/secret key and want to retrieve them programmatically.<br /> In most cases, however, you can use the Cockpit web interface to retrieve them.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify the Region of your account, but you leave the access key value and secret key value empty (`&quot;&quot;`). > 2. You then specify the `--profile`, `--authentication-method`, `--login`, and `--password` options when executing OSC CLI commands.  See the code samples in each section of this documentation for specific examples in different programming languages. ### No Authentication A few API actions do not require any authentication. They are indicated as such in this documentation.<br /> ### Other Security Mechanisms In parallel with the authentication schemes, you can add other security mechanisms to your OUTSCALE account, for example to restrict API requests by IP or other criteria.<br /> For more information, see [Managing Your API Accesses](https://docs.outscale.com/en/userguide/Managing-Your-API-Accesses.html).
 *
 * The version of the OpenAPI document: 1.28.5
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// LoadBalancer : Information about the load balancer.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoadBalancer {
    #[serde(rename = "AccessLog", skip_serializing_if = "Option::is_none")]
    pub access_log: Option<Box<crate::models::AccessLog>>,
    /// The stickiness policies defined for the load balancer.
    #[serde(
        rename = "ApplicationStickyCookiePolicies",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_sticky_cookie_policies:
        Option<Vec<crate::models::ApplicationStickyCookiePolicy>>,
    /// One or more public IPs of back-end VMs.
    #[serde(rename = "BackendIps", skip_serializing_if = "Option::is_none")]
    pub backend_ips: Option<Vec<String>>,
    /// One or more IDs of back-end VMs for the load balancer.
    #[serde(rename = "BackendVmIds", skip_serializing_if = "Option::is_none")]
    pub backend_vm_ids: Option<Vec<String>>,
    /// The DNS name of the load balancer.
    #[serde(rename = "DnsName", skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[serde(rename = "HealthCheck", skip_serializing_if = "Option::is_none")]
    pub health_check: Option<Box<crate::models::HealthCheck>>,
    /// The listeners for the load balancer.
    #[serde(rename = "Listeners", skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<crate::models::Listener>>,
    /// The name of the load balancer.
    #[serde(rename = "LoadBalancerName", skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    /// The policies defined for the load balancer.
    #[serde(
        rename = "LoadBalancerStickyCookiePolicies",
        skip_serializing_if = "Option::is_none"
    )]
    pub load_balancer_sticky_cookie_policies:
        Option<Vec<crate::models::LoadBalancerStickyCookiePolicy>>,
    /// The type of load balancer. Valid only for load balancers in a Net.<br /> If `LoadBalancerType` is `internet-facing`, the load balancer has a public DNS name that resolves to a public IP.<br /> If `LoadBalancerType` is `internal`, the load balancer has a public DNS name that resolves to a private IP.
    #[serde(rename = "LoadBalancerType", skip_serializing_if = "Option::is_none")]
    pub load_balancer_type: Option<String>,
    /// The ID of the Net for the load balancer.
    #[serde(rename = "NetId", skip_serializing_if = "Option::is_none")]
    pub net_id: Option<String>,
    /// (internet-facing only) The public IP associated with the load balancer.
    #[serde(rename = "PublicIp", skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    /// Whether secure cookies are enabled for the load balancer.
    #[serde(rename = "SecuredCookies", skip_serializing_if = "Option::is_none")]
    pub secured_cookies: Option<bool>,
    /// One or more IDs of security groups for the load balancers. Valid only for load balancers in a Net.
    #[serde(rename = "SecurityGroups", skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(
        rename = "SourceSecurityGroup",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_security_group: Option<Box<crate::models::SourceSecurityGroup>>,
    /// The ID of the Subnet in which the load balancer was created.
    #[serde(rename = "Subnets", skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    /// The ID of the Subregion in which the load balancer was created.
    #[serde(rename = "SubregionNames", skip_serializing_if = "Option::is_none")]
    pub subregion_names: Option<Vec<String>>,
    /// One or more tags associated with the load balancer.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ResourceTag>>,
}

impl LoadBalancer {
    /// Information about the load balancer.
    pub fn new() -> LoadBalancer {
        LoadBalancer {
            access_log: None,
            application_sticky_cookie_policies: None,
            backend_ips: None,
            backend_vm_ids: None,
            dns_name: None,
            health_check: None,
            listeners: None,
            load_balancer_name: None,
            load_balancer_sticky_cookie_policies: None,
            load_balancer_type: None,
            net_id: None,
            public_ip: None,
            secured_cookies: None,
            security_groups: None,
            source_security_group: None,
            subnets: None,
            subregion_names: None,
            tags: None,
        }
    }
}
