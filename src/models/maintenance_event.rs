/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.24
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// MaintenanceEvent : Information about the maintenance event.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MaintenanceEvent {
    /// The code of the event (`system-reboot` \\| `system-maintenance`).
    #[serde(rename = "Code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The description of the event.
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The latest scheduled end time for the event.
    #[serde(rename = "NotAfter", skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,
    /// The earliest scheduled start time for the event.
    #[serde(rename = "NotBefore", skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
}

impl MaintenanceEvent {
    /// Information about the maintenance event.
    pub fn new() -> MaintenanceEvent {
        MaintenanceEvent {
            code: None,
            description: None,
            not_after: None,
            not_before: None,
        }
    }
}
