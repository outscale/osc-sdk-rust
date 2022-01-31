/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.17
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReadApiLogsRequest {
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "Filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Box<crate::models::FiltersApiLog>>,
    /// The token to request the next page of results.
    #[serde(rename = "NextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// The maximum number of logs returned in a single response (between `1`and `1000`, both included). By default, `100`.
    #[serde(rename = "ResultsPerPage", skip_serializing_if = "Option::is_none")]
    pub results_per_page: Option<i32>,
    #[serde(rename = "With", skip_serializing_if = "Option::is_none")]
    pub with: Option<Box<crate::models::With>>,
}

impl ReadApiLogsRequest {
    pub fn new() -> ReadApiLogsRequest {
        ReadApiLogsRequest {
            dry_run: None,
            filters: None,
            next_page_token: None,
            results_per_page: None,
            with: None,
        }
    }
}
