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
pub struct ReadConsumptionAccountRequest {
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The beginning of the time period, in ISO 8601 date-time format (for example, `2017-06-14` or `2017-06-14T00:00:00Z`).
    #[serde(rename = "FromDate")]
    pub from_date: String,
    /// The end of the time period, in ISO 8601 date-time format (for example, `2017-06-30` or `2017-06-30T00:00:00Z`).
    #[serde(rename = "ToDate")]
    pub to_date: String,
}

impl ReadConsumptionAccountRequest {
    pub fn new(from_date: String, to_date: String) -> ReadConsumptionAccountRequest {
        ReadConsumptionAccountRequest {
            dry_run: None,
            from_date,
            to_date,
        }
    }
}
