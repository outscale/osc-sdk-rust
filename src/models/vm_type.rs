/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> Throttling: To protect against overloads, the number of identical requests allowed in a given time period is limited.<br /> Brute force: To protect against brute force attacks, the number of failed authentication attempts in a given time period is limited.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).<br /> # Authentication Schemes ### Access Key/Secret Key The main way to authenticate your requests to the OUTSCALE API is to use an access key and a secret key.<br /> The mechanism behind this is based on AWS Signature Version 4, whose technical implementation details are described in [Signature of API Requests](https://docs.outscale.com/en/userguide/Signature-of-API-Requests.html).<br /><br /> In practice, the way to specify your access key and secret key depends on the tool or SDK you want to use to interact with the API.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify your access key, secret key, and the Region of your account. > 2. You then specify the `--profile` option when executing OSC CLI commands. >  > For more information, see [Installing and Configuring OSC CLI](https://docs.outscale.com/en/userguide/Installing-and-Configuring-OSC-CLI.html).  See the code samples in each section of this documentation for specific examples in different programming languages.<br /> For more information about access keys, see [About Access Keys](https://docs.outscale.com/en/userguide/About-Access-Keys.html). ### Login/Password For certain API actions, you can also use basic authentication with the login (email address) and password of your TINA account.<br /> This is useful only in special circumstances, for example if you do not know your access key/secret key and want to retrieve them programmatically.<br /> In most cases, however, you can use the Cockpit web interface to retrieve them.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify the Region of your account, but you leave the access key value and secret key value empty (`&quot;&quot;`). > 2. You then specify the `--profile`, `--authentication-method`, `--login`, and `--password` options when executing OSC CLI commands.  See the code samples in each section of this documentation for specific examples in different programming languages. ### No Authentication A few API actions do not require any authentication. They are indicated as such in this documentation.<br /> ### Other Security Mechanisms In parallel with the authentication schemes, you can add other security mechanisms to your OUTSCALE account, for example to restrict API requests by IP or other criteria.<br /> For more information, see [Managing Your API Accesses](https://docs.outscale.com/en/userguide/Managing-Your-API-Accesses.html).
 *
 * The version of the OpenAPI document: 1.28.5
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// VmType : Information about the VM type.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VmType {
    /// This parameter is not available. It is present in our API for the sake of historical compatibility with AWS.
    #[serde(rename = "BsuOptimized", skip_serializing_if = "Option::is_none")]
    pub bsu_optimized: Option<bool>,
    /// The type of ephemeral storage disk.
    #[serde(rename = "EphemeralsType", skip_serializing_if = "Option::is_none")]
    pub ephemerals_type: Option<String>,
    /// The number of Ethernet interface available.
    #[serde(rename = "Eth", skip_serializing_if = "Option::is_none")]
    pub eth: Option<i32>,
    /// The number of GPU available.
    #[serde(rename = "Gpu", skip_serializing_if = "Option::is_none")]
    pub gpu: Option<i32>,
    /// The maximum number of private IPs per network interface card (NIC).
    #[serde(rename = "MaxPrivateIps", skip_serializing_if = "Option::is_none")]
    pub max_private_ips: Option<i32>,
    /// The amount of memory, in gibibytes.
    #[serde(rename = "MemorySize", skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<f32>,
    /// The number of vCores.
    #[serde(rename = "VcoreCount", skip_serializing_if = "Option::is_none")]
    pub vcore_count: Option<i32>,
    /// The name of the VM type.
    #[serde(rename = "VmTypeName", skip_serializing_if = "Option::is_none")]
    pub vm_type_name: Option<String>,
    /// The maximum number of ephemeral storage disks.
    #[serde(rename = "VolumeCount", skip_serializing_if = "Option::is_none")]
    pub volume_count: Option<i32>,
    /// The size of one ephemeral storage disk, in gibibytes (GiB).
    #[serde(rename = "VolumeSize", skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i32>,
}

impl VmType {
    /// Information about the VM type.
    pub fn new() -> VmType {
        VmType {
            bsu_optimized: None,
            ephemerals_type: None,
            eth: None,
            gpu: None,
            max_private_ips: None,
            memory_size: None,
            vcore_count: None,
            vm_type_name: None,
            volume_count: None,
            volume_size: None,
        }
    }
}
