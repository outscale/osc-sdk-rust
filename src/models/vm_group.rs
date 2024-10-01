/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> Throttling: To protect against overloads, the number of identical requests allowed in a given time period is limited.<br /> Brute force: To protect against brute force attacks, the number of failed authentication attempts in a given time period is limited.<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/About-the-APIs.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).<br /> # Authentication Schemes ### Access Key/Secret Key The main way to authenticate your requests to the OUTSCALE API is to use an access key and a secret key.<br /> The mechanism behind this is based on AWS Signature Version 4, whose technical implementation details are described in [Signature of API Requests](https://docs.outscale.com/en/userguide/Signature-of-API-Requests.html).<br /><br /> In practice, the way to specify your access key and secret key depends on the tool or SDK you want to use to interact with the API.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify your access key, secret key, and the Region of your account. > 2. You then specify the `--profile` option when executing OSC CLI commands. >  > For more information, see [Installing and Configuring OSC CLI](https://docs.outscale.com/en/userguide/Installing-and-Configuring-OSC-CLI.html).  See the code samples in each section of this documentation for specific examples in different programming languages.<br /> For more information about access keys, see [About Access Keys](https://docs.outscale.com/en/userguide/About-Access-Keys.html).  > If you try to sign requests with an invalid access key four times in a row, further authentication attempts will be prevented for 1 minute. This lockout time increases 1 minute every four failed attempts, for up to 10 minutes.  ### Login/Password For certain API actions, you can also use basic authentication with the login (email address) and password of your TINA account.<br /> This is useful only in special circumstances, for example if you do not know your access key/secret key and want to retrieve them programmatically.<br /> In most cases, however, you can use the Cockpit web interface to retrieve them.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify the Region of your account, but you leave the access key value and secret key value empty (`&quot;&quot;`). > 2. You then specify the `--profile`, `--authentication-method`, `--login`, and `--password` options when executing OSC CLI commands.  See the code samples in each section of this documentation for specific examples in different programming languages.  > If you try to sign requests with an invalid password four times in a row, further authentication attempts will be prevented for 1 minute. This lockout time increases 1 minute every four failed attempts, for up to 10 minutes.  ### No Authentication A few API actions do not require any authentication. They are indicated as such in this documentation.<br /> ### Other Security Mechanisms In parallel with the authentication schemes, you can add other security mechanisms to your OUTSCALE account, for example to restrict API requests by IP or other criteria.<br /> For more information, see [Managing Your API Accesses](https://docs.outscale.com/en/userguide/Managing-Your-API-Accesses.html). # Pagination Tutorial You can learn more about the pagination methods for read calls in the dedicated [pagination tutorial](https://docs.outscale.com/en/userguide/Tutorial-Paginating-an-API-Request.html). # Error Codes Reference You can learn more about errors returned by the API in the dedicated [errors page](api-errors.html).
 *
 * The version of the OpenAPI document: 1.33.1
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// VmGroup : Information about the VM group.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VmGroup {
    /// The date and time (UTC) at which the VM group was created.
    #[serde(rename = "CreationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// The description of the VM group.
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The positioning strategy of the VMs on hypervisors. By default, or if set to `no-strategy`, TINA determines the most adequate position for the VMs. If set to `attract`, the VMs are deployed on the same hypervisor, which improves network performance. If set to `repulse`, the VMs are deployed on a different hypervisor, which improves fault tolerance.
    #[serde(
        rename = "PositioningStrategy",
        skip_serializing_if = "Option::is_none"
    )]
    pub positioning_strategy: Option<PositioningStrategy>,
    /// One or more IDs of security groups for the VM group.
    #[serde(rename = "SecurityGroupIds", skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// The state of the VM group (`pending` \\| `available` \\| `scaling up` \\| `scaling down` \\| `deleting` \\| `deleted`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// The ID of the Subnet for the VM group.
    #[serde(rename = "SubnetId", skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// One or more tags associated with the VM.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ResourceTag>>,
    /// The number of VMs in the VM group.
    #[serde(rename = "VmCount", skip_serializing_if = "Option::is_none")]
    pub vm_count: Option<i32>,
    /// The ID of the VM group.
    #[serde(rename = "VmGroupId", skip_serializing_if = "Option::is_none")]
    pub vm_group_id: Option<String>,
    /// The name of the VM group.
    #[serde(rename = "VmGroupName", skip_serializing_if = "Option::is_none")]
    pub vm_group_name: Option<String>,
    /// The IDs of the VMs in the VM group.
    #[serde(rename = "VmIds", skip_serializing_if = "Option::is_none")]
    pub vm_ids: Option<Vec<String>>,
    /// The ID of the VM template used by the VM group.
    #[serde(rename = "VmTemplateId", skip_serializing_if = "Option::is_none")]
    pub vm_template_id: Option<String>,
}

impl VmGroup {
    /// Information about the VM group.
    pub fn new() -> VmGroup {
        VmGroup {
            creation_date: None,
            description: None,
            positioning_strategy: None,
            security_group_ids: None,
            state: None,
            subnet_id: None,
            tags: None,
            vm_count: None,
            vm_group_id: None,
            vm_group_name: None,
            vm_ids: None,
            vm_template_id: None,
        }
    }
}

/// The positioning strategy of the VMs on hypervisors. By default, or if set to `no-strategy`, TINA determines the most adequate position for the VMs. If set to `attract`, the VMs are deployed on the same hypervisor, which improves network performance. If set to `repulse`, the VMs are deployed on a different hypervisor, which improves fault tolerance.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PositioningStrategy {
    #[serde(rename = "attract")]
    Attract,
    #[serde(rename = "no-strategy")]
    NoStrategy,
    #[serde(rename = "repulse")]
    Repulse,
}

impl Default for PositioningStrategy {
    fn default() -> PositioningStrategy {
        Self::Attract
    }
}
/// The state of the VM group (`pending` \\| `available` \\| `scaling up` \\| `scaling down` \\| `deleting` \\| `deleted`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "scaling down")]
    ScalingDown,
    #[serde(rename = "scaling up")]
    ScalingUp,
}

impl Default for State {
    fn default() -> State {
        Self::Available
    }
}
