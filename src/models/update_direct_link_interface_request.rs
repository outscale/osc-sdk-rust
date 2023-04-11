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
pub struct UpdateDirectLinkInterfaceRequest {
    /// The ID of the DirectLink interface you want to update.
    #[serde(rename = "DirectLinkInterfaceId")]
    pub direct_link_interface_id: String,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The maximum transmission unit (MTU) of the DirectLink interface, in bytes (always `1500`).
    #[serde(rename = "Mtu")]
    pub mtu: Mtu,
}

impl UpdateDirectLinkInterfaceRequest {
    pub fn new(direct_link_interface_id: String, mtu: Mtu) -> UpdateDirectLinkInterfaceRequest {
        UpdateDirectLinkInterfaceRequest {
            direct_link_interface_id,
            dry_run: None,
            mtu,
        }
    }
}

/// The maximum transmission unit (MTU) of the DirectLink interface, in bytes (always `1500`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mtu {
    #[serde(rename = "1500")]
    _1500,
}

impl Default for Mtu {
    fn default() -> Mtu {
        Self::_1500
    }
}
