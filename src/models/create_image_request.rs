/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> Throttling: To protect against overloads, the number of identical requests allowed in a given time period is limited.<br /> Brute force: To protect against brute force attacks, the number of failed authentication attempts in a given time period is limited.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).<br /> # Authentication Schemes ### Access Key/Secret Key The main way to authenticate your requests to the OUTSCALE API is to use an access key and a secret key.<br /> The mechanism behind this is based on AWS Signature Version 4, whose technical implementation details are described in [Signature of API Requests](https://docs.outscale.com/en/userguide/Signature-of-API-Requests.html).<br /><br /> In practice, the way to specify your access key and secret key depends on the tool or SDK you want to use to interact with the API.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify your access key, secret key, and the Region of your account. > 2. You then specify the `--profile` option when executing OSC CLI commands. >  > For more information, see [Installing and Configuring OSC CLI](https://docs.outscale.com/en/userguide/Installing-and-Configuring-OSC-CLI.html).  See the code samples in each section of this documentation for specific examples in different programming languages.<br /> For more information about access keys, see [About Access Keys](https://docs.outscale.com/en/userguide/About-Access-Keys.html). ### Login/Password For certain API actions, you can also use basic authentication with the login (email address) and password of your TINA account.<br /> This is useful only in special circumstances, for example if you do not know your access key/secret key and want to retrieve them programmatically.<br /> In most cases, however, you can use the Cockpit web interface to retrieve them.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify the Region of your account, but you leave the access key value and secret key value empty (`&quot;&quot;`). > 2. You then specify the `--profile`, `--authentication-method`, `--login`, and `--password` options when executing OSC CLI commands.  See the code samples in each section of this documentation for specific examples in different programming languages. ### No Authentication A few API actions do not require any authentication. They are indicated as such in this documentation.<br /> ### Other Security Mechanisms In parallel with the authentication schemes, you can add other security mechanisms to your OUTSCALE account, for example to restrict API requests by IP or other criteria.<br /> For more information, see [Managing Your API Accesses](https://docs.outscale.com/en/userguide/Managing-Your-API-Accesses.html).
 *
 * The version of the OpenAPI document: 1.28.7
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateImageRequest {
    /// **(when registering from a snapshot, or from a bucket without using a manifest file)** The architecture of the OMI (`i386` or `x86_64`).
    #[serde(rename = "Architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// **(when registering from a snapshot, or from a bucket without using a manifest file)** One or more block device mappings.
    #[serde(
        rename = "BlockDeviceMappings",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_device_mappings: Option<Vec<crate::models::BlockDeviceMappingImage>>,
    /// A description for the new OMI.
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// **(when registering from a bucket by using a manifest file)** The pre-signed URL of the manifest file for the OMI you want to register. For more information, see [Configuring a Pre-signed URL](https://docs.outscale.com/en/userguide/Configuring-a-Pre-signed-URL.html) or [Managing Access to Your Buckets and Objects](https://docs.outscale.com/en/userguide/Managing-Access-to-Your-Buckets-and-Objects.html).<br /> You can also specify the normal URL of the OMI if you have permission on the OOS bucket, without using the manifest file, but in that case, you need to manually specify through the other parameters all the information that would otherwise be read from the manifest file.
    #[serde(rename = "FileLocation", skip_serializing_if = "Option::is_none")]
    pub file_location: Option<String>,
    /// A unique name for the new OMI.<br /> Constraints: 3-128 alphanumeric characters, underscores (`_`), spaces (` `), parentheses (`()`), slashes (`/`), periods (`.`), or dashes (`-`).
    #[serde(rename = "ImageName", skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    /// **(when creating from a VM)** If false, the VM shuts down before creating the OMI and then reboots. If true, the VM does not.
    #[serde(rename = "NoReboot", skip_serializing_if = "Option::is_none")]
    pub no_reboot: Option<bool>,
    /// The product codes associated with the OMI.
    #[serde(rename = "ProductCodes", skip_serializing_if = "Option::is_none")]
    pub product_codes: Option<Vec<String>>,
    /// **(when registering from a snapshot, or from a bucket without using a manifest file)** The name of the root device for the new OMI.
    #[serde(rename = "RootDeviceName", skip_serializing_if = "Option::is_none")]
    pub root_device_name: Option<String>,
    /// **(when copying an OMI)** The ID of the OMI you want to copy.
    #[serde(rename = "SourceImageId", skip_serializing_if = "Option::is_none")]
    pub source_image_id: Option<String>,
    /// **(when copying an OMI)** The name of the source Region (always the same as the Region of your account).
    #[serde(rename = "SourceRegionName", skip_serializing_if = "Option::is_none")]
    pub source_region_name: Option<String>,
    /// **(when creating from a VM)** The ID of the VM from which you want to create the OMI.
    #[serde(rename = "VmId", skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
}

impl CreateImageRequest {
    pub fn new() -> CreateImageRequest {
        CreateImageRequest {
            architecture: None,
            block_device_mappings: None,
            description: None,
            dry_run: None,
            file_location: None,
            image_name: None,
            no_reboot: None,
            product_codes: None,
            root_device_name: None,
            source_image_id: None,
            source_region_name: None,
            vm_id: None,
        }
    }
}
