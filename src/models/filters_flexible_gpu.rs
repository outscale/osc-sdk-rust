/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> Throttling: To protect against overloads, the number of identical requests allowed in a given time period is limited.<br /> Brute force: To protect against brute force attacks, the number of failed authentication attempts in a given time period is limited.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).<br /> # Authentication Schemes ### Access Key/Secret Key The main way to authenticate your requests to the OUTSCALE API is to use an access key and a secret key.<br /> The mechanism behind this is based on AWS Signature Version 4, whose technical implementation details are described in [Signature of API Requests](https://docs.outscale.com/en/userguide/Signature-of-API-Requests.html).<br /><br /> In practice, the way to specify your access key and secret key depends on the tool or SDK you want to use to interact with the API.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify your access key, secret key, and the Region of your account. > 2. You then specify the `--profile` option when executing OSC CLI commands. >  > For more information, see [Installing and Configuring OSC CLI](https://docs.outscale.com/en/userguide/Installing-and-Configuring-OSC-CLI.html).  See the code samples in each section of this documentation for specific examples in different programming languages.<br /> For more information about access keys, see [About Access Keys](https://docs.outscale.com/en/userguide/About-Access-Keys.html). ### Login/Password For certain API actions, you can also use basic authentication with the login (email address) and password of your TINA account.<br /> This is useful only in special circumstances, for example if you do not know your access key/secret key and want to retrieve them programmatically.<br /> In most cases, however, you can use the Cockpit web interface to retrieve them.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify the Region of your account, but you leave the access key value and secret key value empty (`&quot;&quot;`). > 2. You then specify the `--profile`, `--authentication-method`, `--login`, and `--password` options when executing OSC CLI commands.  See the code samples in each section of this documentation for specific examples in different programming languages. ### No Authentication A few API actions do not require any authentication. They are indicated as such in this documentation.<br /> ### Other Security Mechanisms In parallel with the authentication schemes, you can add other security mechanisms to your OUTSCALE account, for example to restrict API requests by IP or other criteria.<br /> For more information, see [Managing Your API Accesses](https://docs.outscale.com/en/userguide/Managing-Your-API-Accesses.html).
 *
 * The version of the OpenAPI document: 1.28.5
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersFlexibleGpu : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersFlexibleGpu {
    /// Indicates whether the fGPU is deleted when terminating the VM.
    #[serde(rename = "DeleteOnVmDeletion", skip_serializing_if = "Option::is_none")]
    pub delete_on_vm_deletion: Option<bool>,
    /// One or more IDs of fGPUs.
    #[serde(rename = "FlexibleGpuIds", skip_serializing_if = "Option::is_none")]
    pub flexible_gpu_ids: Option<Vec<String>>,
    /// The processor generations that the fGPUs are compatible with.
    #[serde(rename = "Generations", skip_serializing_if = "Option::is_none")]
    pub generations: Option<Vec<String>>,
    /// One or more models of fGPUs. For more information, see [About Flexible GPUs](https://docs.outscale.com/en/userguide/About-Flexible-GPUs.html).
    #[serde(rename = "ModelNames", skip_serializing_if = "Option::is_none")]
    pub model_names: Option<Vec<String>>,
    /// The states of the fGPUs (`allocated` \\| `attaching` \\| `attached` \\| `detaching`).
    #[serde(rename = "States", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
    /// The Subregions where the fGPUs are located.
    #[serde(rename = "SubregionNames", skip_serializing_if = "Option::is_none")]
    pub subregion_names: Option<Vec<String>>,
    /// One or more IDs of VMs.
    #[serde(rename = "VmIds", skip_serializing_if = "Option::is_none")]
    pub vm_ids: Option<Vec<String>>,
}

impl FiltersFlexibleGpu {
    /// One or more filters.
    pub fn new() -> FiltersFlexibleGpu {
        FiltersFlexibleGpu {
            delete_on_vm_deletion: None,
            flexible_gpu_ids: None,
            generations: None,
            model_names: None,
            states: None,
            subregion_names: None,
            vm_ids: None,
        }
    }
}
