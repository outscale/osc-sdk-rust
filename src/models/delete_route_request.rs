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
pub struct DeleteRouteRequest {
    /// The exact IP range for the route.
    #[serde(rename = "DestinationIpRange")]
    pub destination_ip_range: String,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The ID of the route table from which you want to delete a route.
    #[serde(rename = "RouteTableId")]
    pub route_table_id: String,
}

impl DeleteRouteRequest {
    pub fn new(destination_ip_range: String, route_table_id: String) -> DeleteRouteRequest {
        DeleteRouteRequest {
            destination_ip_range,
            dry_run: None,
            route_table_id,
        }
    }
}
