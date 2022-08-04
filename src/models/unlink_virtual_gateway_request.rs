/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.21
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UnlinkVirtualGatewayRequest {
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The ID of the Net from which you want to detach the virtual gateway.
    #[serde(rename = "NetId")]
    pub net_id: String,
    /// The ID of the virtual gateway.
    #[serde(rename = "VirtualGatewayId")]
    pub virtual_gateway_id: String,
}

impl UnlinkVirtualGatewayRequest {
    pub fn new(net_id: String, virtual_gateway_id: String) -> UnlinkVirtualGatewayRequest {
        UnlinkVirtualGatewayRequest {
            dry_run: None,
            net_id,
            virtual_gateway_id,
        }
    }
}
