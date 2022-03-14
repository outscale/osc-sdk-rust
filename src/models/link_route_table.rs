/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.18
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// LinkRouteTable : One or more associations between the route table and the Subnets.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkRouteTable {
    /// The ID of the association between the route table and the Subnet.
    #[serde(rename = "LinkRouteTableId", skip_serializing_if = "Option::is_none")]
    pub link_route_table_id: Option<String>,
    /// If true, the route table is the main one.
    #[serde(rename = "Main", skip_serializing_if = "Option::is_none")]
    pub main: Option<bool>,
    /// The ID of the route table.
    #[serde(rename = "RouteTableId", skip_serializing_if = "Option::is_none")]
    pub route_table_id: Option<String>,
    /// The ID of the Subnet.
    #[serde(rename = "SubnetId", skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

impl LinkRouteTable {
    /// One or more associations between the route table and the Subnets.
    pub fn new() -> LinkRouteTable {
        LinkRouteTable {
            link_route_table_id: None,
            main: None,
            route_table_id: None,
            subnet_id: None,
        }
    }
}
