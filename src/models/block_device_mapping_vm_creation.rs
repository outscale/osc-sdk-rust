/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.24
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// BlockDeviceMappingVmCreation : Information about the block device mapping.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BlockDeviceMappingVmCreation {
    #[serde(rename = "Bsu", skip_serializing_if = "Option::is_none")]
    pub bsu: Option<Box<crate::models::BsuToCreate>>,
    /// The device name for the volume. For a root device, you must use `/dev/sda1`. For other volumes, you must use `/dev/sdX`, `/dev/sdXX`, `/dev/xvdX`, or `/dev/xvdXX` (where the first `X` is a letter between `b` and `z`, and the second `X` is a letter between `a` and `z`).
    #[serde(rename = "DeviceName", skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// Removes the device which is included in the block device mapping of the OMI.
    #[serde(rename = "NoDevice", skip_serializing_if = "Option::is_none")]
    pub no_device: Option<String>,
    /// The name of the virtual device (`ephemeralN`).
    #[serde(rename = "VirtualDeviceName", skip_serializing_if = "Option::is_none")]
    pub virtual_device_name: Option<String>,
}

impl BlockDeviceMappingVmCreation {
    /// Information about the block device mapping.
    pub fn new() -> BlockDeviceMappingVmCreation {
        BlockDeviceMappingVmCreation {
            bsu: None,
            device_name: None,
            no_device: None,
            virtual_device_name: None,
        }
    }
}
