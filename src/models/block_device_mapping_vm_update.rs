/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://wiki.outscale.net/display/EN/3DS+OUTSCALE+APIs+Reference).<br /><br />  You can also manage your resources using the [Cockpit](https://wiki.outscale.net/display/EN/About+Cockpit) web interface.
 *
 * The version of the OpenAPI document: 1.16
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// BlockDeviceMappingVmUpdate : Information about the block device mapping.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BlockDeviceMappingVmUpdate {
    #[serde(rename = "Bsu", skip_serializing_if = "Option::is_none")]
    pub bsu: Option<Box<crate::models::BsuToUpdateVm>>,
    /// The name of the device.
    #[serde(rename = "DeviceName", skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// Removes the device which is included in the block device mapping of the OMI.
    #[serde(rename = "NoDevice", skip_serializing_if = "Option::is_none")]
    pub no_device: Option<String>,
    /// The name of the virtual device (ephemeralN).
    #[serde(rename = "VirtualDeviceName", skip_serializing_if = "Option::is_none")]
    pub virtual_device_name: Option<String>,
}

impl BlockDeviceMappingVmUpdate {
    /// Information about the block device mapping.
    pub fn new() -> BlockDeviceMappingVmUpdate {
        BlockDeviceMappingVmUpdate {
            bsu: None,
            device_name: None,
            no_device: None,
            virtual_device_name: None,
        }
    }
}
