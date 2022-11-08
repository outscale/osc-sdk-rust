/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.23
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// ConsumptionEntry : Information about the resources consumed during the specified time period.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConsumptionEntry {
    /// The ID of your TINA account.
    #[serde(rename = "AccountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The category of the resource (for example, `network`).
    #[serde(rename = "Category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The beginning of the time period.
    #[serde(rename = "FromDate", skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
    /// The API call that triggered the resource consumption (for example, `RunInstances` or `CreateVolume`).
    #[serde(rename = "Operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    /// The ID of the TINA account which is billed for your consumption. It can be different from your account in the `AccountId` parameter.
    #[serde(rename = "PayingAccountId", skip_serializing_if = "Option::is_none")]
    pub paying_account_id: Option<String>,
    /// The service of the API call (`TinaOS-FCU`, `TinaOS-LBU`, `TinaOS-DirectLink`, `TinaOS-OOS`, or `TinaOS-OSU`).
    #[serde(rename = "Service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// The name of the Subregion.
    #[serde(rename = "SubregionName", skip_serializing_if = "Option::is_none")]
    pub subregion_name: Option<String>,
    /// A description of the consumed resource.
    #[serde(rename = "Title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The end of the time period.
    #[serde(rename = "ToDate", skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    /// The type of resource, depending on the API call.
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The consumed amount for the resource. The unit depends on the resource type. For more information, see the `Title` element.
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

impl ConsumptionEntry {
    /// Information about the resources consumed during the specified time period.
    pub fn new() -> ConsumptionEntry {
        ConsumptionEntry {
            account_id: None,
            category: None,
            from_date: None,
            operation: None,
            paying_account_id: None,
            service: None,
            subregion_name: None,
            title: None,
            to_date: None,
            _type: None,
            value: None,
        }
    }
}
