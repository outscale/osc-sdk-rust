/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> Throttling: To protect against overloads, the number of identical requests allowed in a given time period is limited.<br /> Brute force: To protect against brute force attacks, the number of failed authentication attempts in a given time period is limited.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).<br /> # Authentication Schemes ### Access Key/Secret Key The main way to authenticate your requests to the OUTSCALE API is to use an access key and a secret key.<br /> The mechanism behind this is based on AWS Signature Version 4, whose technical implementation details are described in [Signature of API Requests](https://docs.outscale.com/en/userguide/Signature-of-API-Requests.html).<br /><br /> In practice, the way to specify your access key and secret key depends on the tool or SDK you want to use to interact with the API.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify your access key, secret key, and the Region of your account. > 2. You then specify the `--profile` option when executing OSC CLI commands. >  > For more information, see [Installing and Configuring OSC CLI](https://docs.outscale.com/en/userguide/Installing-and-Configuring-OSC-CLI.html).  See the code samples in each section of this documentation for specific examples in different programming languages.<br /> For more information about access keys, see [About Access Keys](https://docs.outscale.com/en/userguide/About-Access-Keys.html). ### Login/Password For certain API actions, you can also use basic authentication with the login (email address) and password of your TINA account.<br /> This is useful only in special circumstances, for example if you do not know your access key/secret key and want to retrieve them programmatically.<br /> In most cases, however, you can use the Cockpit web interface to retrieve them.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify the Region of your account, but you leave the access key value and secret key value empty (`&quot;&quot;`). > 2. You then specify the `--profile`, `--authentication-method`, `--login`, and `--password` options when executing OSC CLI commands.  See the code samples in each section of this documentation for specific examples in different programming languages. ### No Authentication A few API actions do not require any authentication. They are indicated as such in this documentation.<br /> ### Other Security Mechanisms In parallel with the authentication schemes, you can add other security mechanisms to your OUTSCALE account, for example to restrict API requests by IP or other criteria.<br /> For more information, see [Managing Your API Accesses](https://docs.outscale.com/en/userguide/Managing-Your-API-Accesses.html).
 *
 * The version of the OpenAPI document: 1.28.5
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateVmTemplateRequest {
    /// The number of vCores to use for each VM.
    #[serde(rename = "CpuCores")]
    pub cpu_cores: i32,
    /// The processor generation to use for each VM (for example, `v4`).
    #[serde(rename = "CpuGeneration")]
    pub cpu_generation: String,
    /// The performance of the VMs (`medium` \\| `high` \\|  `highest`).
    #[serde(rename = "CpuPerformance", skip_serializing_if = "Option::is_none")]
    pub cpu_performance: Option<CpuPerformance>,
    /// A description for the VM template.
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The ID of the OMI to use for each VM. You can find a list of OMIs by calling the [ReadImages](#readimages) method.
    #[serde(rename = "ImageId")]
    pub image_id: String,
    /// The name of the keypair to use for each VM.
    #[serde(rename = "KeypairName", skip_serializing_if = "Option::is_none")]
    pub keypair_name: Option<String>,
    /// The amount of RAM to use for each VM.
    #[serde(rename = "Ram")]
    pub ram: i32,
    /// One or more tags to add to the VM template.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ResourceTag>>,
    /// The name of the VM template.
    #[serde(rename = "VmTemplateName")]
    pub vm_template_name: String,
}

impl CreateVmTemplateRequest {
    pub fn new(
        cpu_cores: i32,
        cpu_generation: String,
        image_id: String,
        ram: i32,
        vm_template_name: String,
    ) -> CreateVmTemplateRequest {
        CreateVmTemplateRequest {
            cpu_cores,
            cpu_generation,
            cpu_performance: None,
            description: None,
            dry_run: None,
            image_id,
            keypair_name: None,
            ram,
            tags: None,
            vm_template_name,
        }
    }
}

/// The performance of the VMs (`medium` \\| `high` \\|  `highest`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CpuPerformance {
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "highest")]
    Highest,
}

impl Default for CpuPerformance {
    fn default() -> CpuPerformance {
        Self::Medium
    }
}
