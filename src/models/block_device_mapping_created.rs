/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// BlockDeviceMappingCreated : Information about the created block device mapping.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BlockDeviceMappingCreated {
    #[serde(rename = "Bsu", skip_serializing_if = "Option::is_none")]
    pub bsu: Option<Box<crate::models::BsuCreated>>,
    /// The name of the device.
    #[serde(rename = "DeviceName", skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
}

impl BlockDeviceMappingCreated {
    /// Information about the created block device mapping.
    pub fn new() -> BlockDeviceMappingCreated {
        BlockDeviceMappingCreated {
            bsu: None,
            device_name: None,
        }
    }
}
