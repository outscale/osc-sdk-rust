/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.18
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateRoutePropagationRequest {
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// If true, a virtual gateway can propagate routes to a specified route table of a Net. If false, the propagation is disabled.
    #[serde(rename = "Enable")]
    pub enable: bool,
    /// The ID of the route table.
    #[serde(rename = "RouteTableId")]
    pub route_table_id: String,
    /// The ID of the virtual gateway.
    #[serde(rename = "VirtualGatewayId")]
    pub virtual_gateway_id: String,
}

impl UpdateRoutePropagationRequest {
    pub fn new(
        enable: bool,
        route_table_id: String,
        virtual_gateway_id: String,
    ) -> UpdateRoutePropagationRequest {
        UpdateRoutePropagationRequest {
            dry_run: None,
            enable,
            route_table_id,
            virtual_gateway_id,
        }
    }
}
