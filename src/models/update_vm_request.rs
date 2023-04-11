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
pub struct UpdateVmRequest {
    /// One or more block device mappings of the VM.
    #[serde(
        rename = "BlockDeviceMappings",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_device_mappings: Option<Vec<crate::models::BlockDeviceMappingVmUpdate>>,
    /// This parameter is not available. It is present in our API for the sake of historical compatibility with AWS.
    #[serde(rename = "BsuOptimized", skip_serializing_if = "Option::is_none")]
    pub bsu_optimized: Option<bool>,
    /// If true, you cannot delete the VM unless you change this parameter back to false.
    #[serde(rename = "DeletionProtection", skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// (Net only) If true, the source/destination check is enabled. If false, it is disabled. This value must be false for a NAT VM to perform network address translation (NAT) in a Net.
    #[serde(
        rename = "IsSourceDestChecked",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_source_dest_checked: Option<bool>,
    /// The name of a keypair you want to associate with the VM.<br /> When you replace the keypair of a VM with another one, the metadata of the VM is modified to reflect the new public key, but the replacement is still not effective in the operating system of the VM. To complete the replacement and effectively apply the new keypair, you need to perform other actions inside the VM. For more information, see [Modifying the Keypair of an Instance](https://docs.outscale.com/en/userguide/Modifying-the-Keypair-of-an-Instance.html).
    #[serde(rename = "KeypairName", skip_serializing_if = "Option::is_none")]
    pub keypair_name: Option<String>,
    /// (dedicated tenancy only) If true, nested virtualization is enabled. If false, it is disabled.
    #[serde(
        rename = "NestedVirtualization",
        skip_serializing_if = "Option::is_none"
    )]
    pub nested_virtualization: Option<bool>,
    /// The performance of the VM (`medium` \\| `high` \\|  `highest`).
    #[serde(rename = "Performance", skip_serializing_if = "Option::is_none")]
    pub performance: Option<Performance>,
    /// One or more IDs of security groups for the VM.
    #[serde(rename = "SecurityGroupIds", skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// The Base64-encoded MIME user data, limited to 500 kibibytes (KiB).
    #[serde(rename = "UserData", skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// The ID of the VM.
    #[serde(rename = "VmId")]
    pub vm_id: String,
    /// The VM behavior when you stop it. If set to `stop`, the VM stops. If set to `restart`, the VM stops then automatically restarts. If set to `terminate`, the VM stops and is terminated.
    #[serde(
        rename = "VmInitiatedShutdownBehavior",
        skip_serializing_if = "Option::is_none"
    )]
    pub vm_initiated_shutdown_behavior: Option<String>,
    /// The type of VM. For more information, see [Instance Types](https://docs.outscale.com/en/userguide/Instance-Types.html).
    #[serde(rename = "VmType", skip_serializing_if = "Option::is_none")]
    pub vm_type: Option<String>,
}

impl UpdateVmRequest {
    pub fn new(vm_id: String) -> UpdateVmRequest {
        UpdateVmRequest {
            block_device_mappings: None,
            bsu_optimized: None,
            deletion_protection: None,
            dry_run: None,
            is_source_dest_checked: None,
            keypair_name: None,
            nested_virtualization: None,
            performance: None,
            security_group_ids: None,
            user_data: None,
            vm_id,
            vm_initiated_shutdown_behavior: None,
            vm_type: None,
        }
    }
}

/// The performance of the VM (`medium` \\| `high` \\|  `highest`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Performance {
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "highest")]
    Highest,
}

impl Default for Performance {
    fn default() -> Performance {
        Self::Medium
    }
}
