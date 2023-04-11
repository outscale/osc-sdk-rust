/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.26
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Vm : Information about the VM.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Vm {
    /// The architecture of the VM (`i386` \\| `x86_64`).
    #[serde(rename = "Architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The block device mapping of the VM.
    #[serde(
        rename = "BlockDeviceMappings",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_device_mappings: Option<Vec<crate::models::BlockDeviceMappingCreated>>,
    /// This parameter is not available. It is present in our API for the sake of historical compatibility with AWS.
    #[serde(rename = "BsuOptimized", skip_serializing_if = "Option::is_none")]
    pub bsu_optimized: Option<bool>,
    /// The idempotency token provided when launching the VM.
    #[serde(rename = "ClientToken", skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// The date and time of creation of the VM.
    #[serde(rename = "CreationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// If true, you cannot delete the VM unless you change this parameter back to false.
    #[serde(rename = "DeletionProtection", skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// The hypervisor type of the VMs (`ovm` \\| `xen`).
    #[serde(rename = "Hypervisor", skip_serializing_if = "Option::is_none")]
    pub hypervisor: Option<String>,
    /// The ID of the OMI used to create the VM.
    #[serde(rename = "ImageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// (Net only) If true, the source/destination check is enabled. If false, it is disabled. This value must be false for a NAT VM to perform network address translation (NAT) in a Net.
    #[serde(
        rename = "IsSourceDestChecked",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_source_dest_checked: Option<bool>,
    /// The name of the keypair used when launching the VM.
    #[serde(rename = "KeypairName", skip_serializing_if = "Option::is_none")]
    pub keypair_name: Option<String>,
    /// The number for the VM when launching a group of several VMs (for example, `0`, `1`, `2`, and so on).
    #[serde(rename = "LaunchNumber", skip_serializing_if = "Option::is_none")]
    pub launch_number: Option<i32>,
    /// If true, nested virtualization is enabled. If false, it is disabled.
    #[serde(
        rename = "NestedVirtualization",
        skip_serializing_if = "Option::is_none"
    )]
    pub nested_virtualization: Option<bool>,
    /// The ID of the Net in which the VM is running.
    #[serde(rename = "NetId", skip_serializing_if = "Option::is_none")]
    pub net_id: Option<String>,
    /// (Net only) The network interface cards (NICs) the VMs are attached to.
    #[serde(rename = "Nics", skip_serializing_if = "Option::is_none")]
    pub nics: Option<Vec<crate::models::NicLight>>,
    /// Indicates the operating system (OS) of the VM.
    #[serde(rename = "OsFamily", skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// The performance of the VM (`medium` \\| `high` \\|  `highest`).
    #[serde(rename = "Performance", skip_serializing_if = "Option::is_none")]
    pub performance: Option<String>,
    #[serde(rename = "Placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<Box<crate::models::Placement>>,
    /// The name of the private DNS.
    #[serde(rename = "PrivateDnsName", skip_serializing_if = "Option::is_none")]
    pub private_dns_name: Option<String>,
    /// The primary private IP of the VM.
    #[serde(rename = "PrivateIp", skip_serializing_if = "Option::is_none")]
    pub private_ip: Option<String>,
    /// The product codes associated with the OMI used to create the VM.
    #[serde(rename = "ProductCodes", skip_serializing_if = "Option::is_none")]
    pub product_codes: Option<Vec<String>>,
    /// The name of the public DNS.
    #[serde(rename = "PublicDnsName", skip_serializing_if = "Option::is_none")]
    pub public_dns_name: Option<String>,
    /// The public IP of the VM.
    #[serde(rename = "PublicIp", skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    /// The reservation ID of the VM.
    #[serde(rename = "ReservationId", skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    /// The name of the root device for the VM (for example, `/dev/vda1`).
    #[serde(rename = "RootDeviceName", skip_serializing_if = "Option::is_none")]
    pub root_device_name: Option<String>,
    /// The type of root device used by the VM (always `bsu`).
    #[serde(rename = "RootDeviceType", skip_serializing_if = "Option::is_none")]
    pub root_device_type: Option<String>,
    /// One or more security groups associated with the VM.
    #[serde(rename = "SecurityGroups", skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<crate::models::SecurityGroupLight>>,
    /// The state of the VM (`pending` \\| `running` \\| `stopping` \\| `stopped` \\| `shutting-down` \\| `terminated` \\| `quarantine`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The reason explaining the current state of the VM.
    #[serde(rename = "StateReason", skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    /// The ID of the Subnet for the VM.
    #[serde(rename = "SubnetId", skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// One or more tags associated with the VM.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ResourceTag>>,
    /// The Base64-encoded MIME user data.
    #[serde(rename = "UserData", skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// The ID of the VM.
    #[serde(rename = "VmId", skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    /// The VM behavior when you stop it. If set to `stop`, the VM stops. If set to `restart`, the VM stops then automatically restarts. If set to `terminate`, the VM stops and is deleted.
    #[serde(
        rename = "VmInitiatedShutdownBehavior",
        skip_serializing_if = "Option::is_none"
    )]
    pub vm_initiated_shutdown_behavior: Option<String>,
    /// The type of VM. For more information, see [Instance Types](https://docs.outscale.com/en/userguide/Instance-Types.html).
    #[serde(rename = "VmType", skip_serializing_if = "Option::is_none")]
    pub vm_type: Option<String>,
}

impl Vm {
    /// Information about the VM.
    pub fn new() -> Vm {
        Vm {
            architecture: None,
            block_device_mappings: None,
            bsu_optimized: None,
            client_token: None,
            creation_date: None,
            deletion_protection: None,
            hypervisor: None,
            image_id: None,
            is_source_dest_checked: None,
            keypair_name: None,
            launch_number: None,
            nested_virtualization: None,
            net_id: None,
            nics: None,
            os_family: None,
            performance: None,
            placement: None,
            private_dns_name: None,
            private_ip: None,
            product_codes: None,
            public_dns_name: None,
            public_ip: None,
            reservation_id: None,
            root_device_name: None,
            root_device_type: None,
            security_groups: None,
            state: None,
            state_reason: None,
            subnet_id: None,
            tags: None,
            user_data: None,
            vm_id: None,
            vm_initiated_shutdown_behavior: None,
            vm_type: None,
        }
    }
}
