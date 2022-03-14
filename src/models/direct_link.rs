/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.18
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// DirectLink : Information about the DirectLink.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DirectLink {
    /// The account ID of the owner of the DirectLink.
    #[serde(rename = "AccountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The physical link bandwidth (either 1 Gbps or 10 Gbps).
    #[serde(rename = "Bandwidth", skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<String>,
    /// The ID of the DirectLink (for example, dxcon-xxxxxxxx).
    #[serde(rename = "DirectLinkId", skip_serializing_if = "Option::is_none")]
    pub direct_link_id: Option<String>,
    /// The name of the DirectLink.
    #[serde(rename = "DirectLinkName", skip_serializing_if = "Option::is_none")]
    pub direct_link_name: Option<String>,
    /// The datacenter where the DirectLink is located.
    #[serde(rename = "Location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// The Region in which the DirectLink has been created.
    #[serde(rename = "RegionName", skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    /// The state of the DirectLink.<br /> * `requested`: The DirectLink is requested but the request has not been validated yet.<br /> * `pending`: The DirectLink request has been validated. It remains in the `pending` state until you establish the physical link.<br /> * `available`: The physical link is established and the connection is ready to use.<br /> * `deleting`: The deletion process is in progress.<br /> * `deleted`: The DirectLink is deleted.
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl DirectLink {
    /// Information about the DirectLink.
    pub fn new() -> DirectLink {
        DirectLink {
            account_id: None,
            bandwidth: None,
            direct_link_id: None,
            direct_link_name: None,
            location: None,
            region_name: None,
            state: None,
        }
    }
}
