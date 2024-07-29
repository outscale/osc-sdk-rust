/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> Throttling: To protect against overloads, the number of identical requests allowed in a given time period is limited.<br /> Brute force: To protect against brute force attacks, the number of failed authentication attempts in a given time period is limited.<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/About-the-APIs.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).<br /> # Authentication Schemes ### Access Key/Secret Key The main way to authenticate your requests to the OUTSCALE API is to use an access key and a secret key.<br /> The mechanism behind this is based on AWS Signature Version 4, whose technical implementation details are described in [Signature of API Requests](https://docs.outscale.com/en/userguide/Signature-of-API-Requests.html).<br /><br /> In practice, the way to specify your access key and secret key depends on the tool or SDK you want to use to interact with the API.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify your access key, secret key, and the Region of your account. > 2. You then specify the `--profile` option when executing OSC CLI commands. >  > For more information, see [Installing and Configuring OSC CLI](https://docs.outscale.com/en/userguide/Installing-and-Configuring-OSC-CLI.html).  See the code samples in each section of this documentation for specific examples in different programming languages.<br /> For more information about access keys, see [About Access Keys](https://docs.outscale.com/en/userguide/About-Access-Keys.html).  > If you try to sign requests with an invalid access key four times in a row, further authentication attempts will be prevented for 1 minute. This lockout time increases 1 minute every four failed attempts, for up to 10 minutes.  ### Login/Password For certain API actions, you can also use basic authentication with the login (email address) and password of your TINA account.<br /> This is useful only in special circumstances, for example if you do not know your access key/secret key and want to retrieve them programmatically.<br /> In most cases, however, you can use the Cockpit web interface to retrieve them.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify the Region of your account, but you leave the access key value and secret key value empty (`&quot;&quot;`). > 2. You then specify the `--profile`, `--authentication-method`, `--login`, and `--password` options when executing OSC CLI commands.  See the code samples in each section of this documentation for specific examples in different programming languages.  > If you try to sign requests with an invalid password four times in a row, further authentication attempts will be prevented for 1 minute. This lockout time increases 1 minute every four failed attempts, for up to 10 minutes.  ### No Authentication A few API actions do not require any authentication. They are indicated as such in this documentation.<br /> ### Other Security Mechanisms In parallel with the authentication schemes, you can add other security mechanisms to your OUTSCALE account, for example to restrict API requests by IP or other criteria.<br /> For more information, see [Managing Your API Accesses](https://docs.outscale.com/en/userguide/Managing-Your-API-Accesses.html).<br /> # Error Codes Reference You can learn more about errors returned by the API in the dedicated [errors page](api-errors.html).
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersVm : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersVm {
    /// The architectures of the VMs (`i386` \\| `x86_64`).
    #[serde(rename = "Architectures", skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<String>>,
    /// Whether the BSU volumes are deleted when terminating the VMs.
    #[serde(
        rename = "BlockDeviceMappingDeleteOnVmDeletion",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_device_mapping_delete_on_vm_deletion: Option<bool>,
    /// The device names for the BSU volumes (in the format `/dev/sdX`, `/dev/sdXX`, `/dev/xvdX`, or `/dev/xvdXX`).
    #[serde(
        rename = "BlockDeviceMappingDeviceNames",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_device_mapping_device_names: Option<Vec<String>>,
    /// The link dates for the BSU volumes mapped to the VMs (for example, `2016-01-23T18:45:30.000Z`).
    #[serde(
        rename = "BlockDeviceMappingLinkDates",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_device_mapping_link_dates: Option<Vec<String>>,
    /// The states for the BSU volumes (`attaching` \\| `attached` \\| `detaching` \\| `detached`).
    #[serde(
        rename = "BlockDeviceMappingStates",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_device_mapping_states: Option<Vec<String>>,
    /// The volume IDs of the BSU volumes.
    #[serde(
        rename = "BlockDeviceMappingVolumeIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_device_mapping_volume_ids: Option<Vec<String>>,
    /// The idempotency tokens provided when launching the VMs.
    #[serde(rename = "ClientTokens", skip_serializing_if = "Option::is_none")]
    pub client_tokens: Option<Vec<String>>,
    /// The dates when the VMs were launched.
    #[serde(rename = "CreationDates", skip_serializing_if = "Option::is_none")]
    pub creation_dates: Option<Vec<String>>,
    /// The IDs of the OMIs used to launch the VMs.
    #[serde(rename = "ImageIds", skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<String>>,
    /// Whether the source/destination checking is enabled (true) or disabled (false).
    #[serde(
        rename = "IsSourceDestChecked",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_source_dest_checked: Option<bool>,
    /// The names of the keypairs used when launching the VMs.
    #[serde(rename = "KeypairNames", skip_serializing_if = "Option::is_none")]
    pub keypair_names: Option<Vec<String>>,
    /// The numbers for the VMs when launching a group of several VMs (for example, `0`, `1`, `2`, and so on).
    #[serde(rename = "LaunchNumbers", skip_serializing_if = "Option::is_none")]
    pub launch_numbers: Option<Vec<i32>>,
    /// Whether the VMs are Spot Instances (spot).
    #[serde(rename = "Lifecycles", skip_serializing_if = "Option::is_none")]
    pub lifecycles: Option<Vec<String>>,
    /// The IDs of the Nets in which the VMs are running.
    #[serde(rename = "NetIds", skip_serializing_if = "Option::is_none")]
    pub net_ids: Option<Vec<String>>,
    /// The IDs of the NICs.
    #[serde(rename = "NicAccountIds", skip_serializing_if = "Option::is_none")]
    pub nic_account_ids: Option<Vec<String>>,
    /// The descriptions of the NICs.
    #[serde(rename = "NicDescriptions", skip_serializing_if = "Option::is_none")]
    pub nic_descriptions: Option<Vec<String>>,
    /// Whether the source/destination checking is enabled (true) or disabled (false).
    #[serde(
        rename = "NicIsSourceDestChecked",
        skip_serializing_if = "Option::is_none"
    )]
    pub nic_is_source_dest_checked: Option<bool>,
    /// Whether the NICs are deleted when the VMs they are attached to are deleted.
    #[serde(
        rename = "NicLinkNicDeleteOnVmDeletion",
        skip_serializing_if = "Option::is_none"
    )]
    pub nic_link_nic_delete_on_vm_deletion: Option<bool>,
    /// The device numbers the NICs are attached to.
    #[serde(
        rename = "NicLinkNicDeviceNumbers",
        skip_serializing_if = "Option::is_none"
    )]
    pub nic_link_nic_device_numbers: Option<Vec<i32>>,
    /// The dates and times (UTC) when the NICs were attached to the VMs.
    #[serde(
        rename = "NicLinkNicLinkNicDates",
        skip_serializing_if = "Option::is_none"
    )]
    pub nic_link_nic_link_nic_dates: Option<Vec<String>>,
    /// The IDs of the NIC attachments.
    #[serde(
        rename = "NicLinkNicLinkNicIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub nic_link_nic_link_nic_ids: Option<Vec<String>>,
    /// The states of the attachments.
    #[serde(rename = "NicLinkNicStates", skip_serializing_if = "Option::is_none")]
    pub nic_link_nic_states: Option<Vec<String>>,
    /// The account IDs of the owners of the VMs the NICs are attached to.
    #[serde(
        rename = "NicLinkNicVmAccountIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub nic_link_nic_vm_account_ids: Option<Vec<String>>,
    /// The IDs of the VMs the NICs are attached to.
    #[serde(rename = "NicLinkNicVmIds", skip_serializing_if = "Option::is_none")]
    pub nic_link_nic_vm_ids: Option<Vec<String>>,
    /// The account IDs of the owners of the public IPs associated with the NICs.
    #[serde(
        rename = "NicLinkPublicIpAccountIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub nic_link_public_ip_account_ids: Option<Vec<String>>,
    /// The association IDs returned when the public IPs were associated with the NICs.
    #[serde(
        rename = "NicLinkPublicIpLinkPublicIpIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub nic_link_public_ip_link_public_ip_ids: Option<Vec<String>>,
    /// The allocation IDs returned when the public IPs were allocated to their accounts.
    #[serde(
        rename = "NicLinkPublicIpPublicIpIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub nic_link_public_ip_public_ip_ids: Option<Vec<String>>,
    /// The public IPs associated with the NICs.
    #[serde(
        rename = "NicLinkPublicIpPublicIps",
        skip_serializing_if = "Option::is_none"
    )]
    pub nic_link_public_ip_public_ips: Option<Vec<String>>,
    /// The Media Access Control (MAC) addresses of the NICs.
    #[serde(rename = "NicMacAddresses", skip_serializing_if = "Option::is_none")]
    pub nic_mac_addresses: Option<Vec<String>>,
    /// The IDs of the Nets where the NICs are located.
    #[serde(rename = "NicNetIds", skip_serializing_if = "Option::is_none")]
    pub nic_net_ids: Option<Vec<String>>,
    /// The IDs of the NICs.
    #[serde(rename = "NicNicIds", skip_serializing_if = "Option::is_none")]
    pub nic_nic_ids: Option<Vec<String>>,
    /// The account IDs of the owner of the public IPs associated with the private IPs.
    #[serde(
        rename = "NicPrivateIpsLinkPublicIpAccountIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub nic_private_ips_link_public_ip_account_ids: Option<Vec<String>>,
    /// The public IPs associated with the private IPs.
    #[serde(
        rename = "NicPrivateIpsLinkPublicIpIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub nic_private_ips_link_public_ip_ids: Option<Vec<String>>,
    /// Whether the private IPs are the primary IPs associated with the NICs.
    #[serde(
        rename = "NicPrivateIpsPrimaryIp",
        skip_serializing_if = "Option::is_none"
    )]
    pub nic_private_ips_primary_ip: Option<bool>,
    /// The private IPs of the NICs.
    #[serde(
        rename = "NicPrivateIpsPrivateIps",
        skip_serializing_if = "Option::is_none"
    )]
    pub nic_private_ips_private_ips: Option<Vec<String>>,
    /// The IDs of the security groups associated with the NICs.
    #[serde(
        rename = "NicSecurityGroupIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub nic_security_group_ids: Option<Vec<String>>,
    /// The names of the security groups associated with the NICs.
    #[serde(
        rename = "NicSecurityGroupNames",
        skip_serializing_if = "Option::is_none"
    )]
    pub nic_security_group_names: Option<Vec<String>>,
    /// The states of the NICs (`available` \\| `in-use`).
    #[serde(rename = "NicStates", skip_serializing_if = "Option::is_none")]
    pub nic_states: Option<Vec<String>>,
    /// The IDs of the Subnets for the NICs.
    #[serde(rename = "NicSubnetIds", skip_serializing_if = "Option::is_none")]
    pub nic_subnet_ids: Option<Vec<String>>,
    /// The Subregions where the NICs are located.
    #[serde(rename = "NicSubregionNames", skip_serializing_if = "Option::is_none")]
    pub nic_subregion_names: Option<Vec<String>>,
    /// The platforms. Use windows if you have Windows VMs. Otherwise, leave this filter blank.
    #[serde(rename = "Platforms", skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<String>>,
    /// The private IPs of the VMs.
    #[serde(rename = "PrivateIps", skip_serializing_if = "Option::is_none")]
    pub private_ips: Option<Vec<String>>,
    /// The product codes associated with the OMI used to create the VMs.
    #[serde(rename = "ProductCodes", skip_serializing_if = "Option::is_none")]
    pub product_codes: Option<Vec<String>>,
    /// The public IPs of the VMs.
    #[serde(rename = "PublicIps", skip_serializing_if = "Option::is_none")]
    pub public_ips: Option<Vec<String>>,
    /// The IDs of the reservation of the VMs, created every time you launch VMs. These reservation IDs can be associated with several VMs when you lauch a group of VMs using the same launch request.
    #[serde(rename = "ReservationIds", skip_serializing_if = "Option::is_none")]
    pub reservation_ids: Option<Vec<String>>,
    /// The names of the root devices for the VMs (for example, `/dev/sda1`)
    #[serde(rename = "RootDeviceNames", skip_serializing_if = "Option::is_none")]
    pub root_device_names: Option<Vec<String>>,
    /// The root devices types used by the VMs (always `ebs`)
    #[serde(rename = "RootDeviceTypes", skip_serializing_if = "Option::is_none")]
    pub root_device_types: Option<Vec<String>>,
    /// The IDs of the security groups for the VMs (only in the public Cloud).
    #[serde(rename = "SecurityGroupIds", skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// The names of the security groups for the VMs (only in the public Cloud).
    #[serde(rename = "SecurityGroupNames", skip_serializing_if = "Option::is_none")]
    pub security_group_names: Option<Vec<String>>,
    /// The reason codes for the state changes.
    #[serde(rename = "StateReasonCodes", skip_serializing_if = "Option::is_none")]
    pub state_reason_codes: Option<Vec<i32>>,
    /// The messages describing the state changes.
    #[serde(
        rename = "StateReasonMessages",
        skip_serializing_if = "Option::is_none"
    )]
    pub state_reason_messages: Option<Vec<String>>,
    /// The reasons explaining the current states of the VMs. This filter is like the `StateReasonCodes` one.
    #[serde(rename = "StateReasons", skip_serializing_if = "Option::is_none")]
    pub state_reasons: Option<Vec<String>>,
    /// The IDs of the Subnets for the VMs.
    #[serde(rename = "SubnetIds", skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// The names of the Subregions of the VMs.
    #[serde(rename = "SubregionNames", skip_serializing_if = "Option::is_none")]
    pub subregion_names: Option<Vec<String>>,
    /// The keys of the tags associated with the VMs.
    #[serde(rename = "TagKeys", skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// The values of the tags associated with the VMs.
    #[serde(rename = "TagValues", skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
    /// The key/value combination of the tags associated with the VMs, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The tenancies of the VMs (`dedicated` \\| `default` \\| `host`).
    #[serde(rename = "Tenancies", skip_serializing_if = "Option::is_none")]
    pub tenancies: Option<Vec<String>>,
    /// One or more IDs of VMs.
    #[serde(rename = "VmIds", skip_serializing_if = "Option::is_none")]
    pub vm_ids: Option<Vec<String>>,
    /// The IDs of the security groups for the VMs.
    #[serde(rename = "VmSecurityGroupIds", skip_serializing_if = "Option::is_none")]
    pub vm_security_group_ids: Option<Vec<String>>,
    /// The names of the security group for the VMs.
    #[serde(
        rename = "VmSecurityGroupNames",
        skip_serializing_if = "Option::is_none"
    )]
    pub vm_security_group_names: Option<Vec<String>>,
    /// The state codes of the VMs: `-1` (quarantine), `0` (pending), `16` (running), `32` (shutting-down), `48` (terminated), `64` (stopping), and `80` (stopped).
    #[serde(rename = "VmStateCodes", skip_serializing_if = "Option::is_none")]
    pub vm_state_codes: Option<Vec<i32>>,
    /// The state names of the VMs (`pending` \\| `running` \\| `stopping` \\| `stopped` \\| `shutting-down` \\| `terminated` \\| `quarantine`).
    #[serde(rename = "VmStateNames", skip_serializing_if = "Option::is_none")]
    pub vm_state_names: Option<Vec<String>>,
    /// The VM types (for example, t2.micro). For more information, see [VM Types](https://docs.outscale.com/en/userguide/VM-Types.html).
    #[serde(rename = "VmTypes", skip_serializing_if = "Option::is_none")]
    pub vm_types: Option<Vec<String>>,
}

impl FiltersVm {
    /// One or more filters.
    pub fn new() -> FiltersVm {
        FiltersVm {
            architectures: None,
            block_device_mapping_delete_on_vm_deletion: None,
            block_device_mapping_device_names: None,
            block_device_mapping_link_dates: None,
            block_device_mapping_states: None,
            block_device_mapping_volume_ids: None,
            client_tokens: None,
            creation_dates: None,
            image_ids: None,
            is_source_dest_checked: None,
            keypair_names: None,
            launch_numbers: None,
            lifecycles: None,
            net_ids: None,
            nic_account_ids: None,
            nic_descriptions: None,
            nic_is_source_dest_checked: None,
            nic_link_nic_delete_on_vm_deletion: None,
            nic_link_nic_device_numbers: None,
            nic_link_nic_link_nic_dates: None,
            nic_link_nic_link_nic_ids: None,
            nic_link_nic_states: None,
            nic_link_nic_vm_account_ids: None,
            nic_link_nic_vm_ids: None,
            nic_link_public_ip_account_ids: None,
            nic_link_public_ip_link_public_ip_ids: None,
            nic_link_public_ip_public_ip_ids: None,
            nic_link_public_ip_public_ips: None,
            nic_mac_addresses: None,
            nic_net_ids: None,
            nic_nic_ids: None,
            nic_private_ips_link_public_ip_account_ids: None,
            nic_private_ips_link_public_ip_ids: None,
            nic_private_ips_primary_ip: None,
            nic_private_ips_private_ips: None,
            nic_security_group_ids: None,
            nic_security_group_names: None,
            nic_states: None,
            nic_subnet_ids: None,
            nic_subregion_names: None,
            platforms: None,
            private_ips: None,
            product_codes: None,
            public_ips: None,
            reservation_ids: None,
            root_device_names: None,
            root_device_types: None,
            security_group_ids: None,
            security_group_names: None,
            state_reason_codes: None,
            state_reason_messages: None,
            state_reasons: None,
            subnet_ids: None,
            subregion_names: None,
            tag_keys: None,
            tag_values: None,
            tags: None,
            tenancies: None,
            vm_ids: None,
            vm_security_group_ids: None,
            vm_security_group_names: None,
            vm_state_codes: None,
            vm_state_names: None,
            vm_types: None,
        }
    }
}
