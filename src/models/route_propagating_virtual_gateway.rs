/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.27
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// RoutePropagatingVirtualGateway : Information about the route propagating virtual gateway.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RoutePropagatingVirtualGateway {
    /// The ID of the virtual gateway.
    #[serde(rename = "VirtualGatewayId", skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
}

impl RoutePropagatingVirtualGateway {
    /// Information about the route propagating virtual gateway.
    pub fn new() -> RoutePropagatingVirtualGateway {
        RoutePropagatingVirtualGateway {
            virtual_gateway_id: None,
        }
    }
}
