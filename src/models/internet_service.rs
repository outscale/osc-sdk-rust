/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://wiki.outscale.net/display/EN/3DS+OUTSCALE+APIs+Reference).<br /><br />  You can also manage your resources using the [Cockpit](https://wiki.outscale.net/display/EN/About+Cockpit) web interface.
 *
 * The version of the OpenAPI document: 1.16
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// InternetService : Information about the Internet service.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InternetService {
    /// The ID of the Internet service.
    #[serde(rename = "InternetServiceId", skip_serializing_if = "Option::is_none")]
    pub internet_service_id: Option<String>,
    /// The ID of the Net attached to the Internet service.
    #[serde(rename = "NetId", skip_serializing_if = "Option::is_none")]
    pub net_id: Option<String>,
    /// The state of the attachment of the Internet service to the Net (always `available`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// One or more tags associated with the Internet service.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ResourceTag>>,
}

impl InternetService {
    /// Information about the Internet service.
    pub fn new() -> InternetService {
        InternetService {
            internet_service_id: None,
            net_id: None,
            state: None,
            tags: None,
        }
    }
}
