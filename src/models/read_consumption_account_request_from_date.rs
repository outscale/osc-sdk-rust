/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.26
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// ReadConsumptionAccountRequestFromDate : The beginning of the time period, in ISO 8601 date format (for example, `2020-06-14`). The date-time format is also accepted, but only with a time set to midnight (for example, `2020-06-14T00:00:00.000Z`).

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReadConsumptionAccountRequestFromDate {}

impl ReadConsumptionAccountRequestFromDate {
    /// The beginning of the time period, in ISO 8601 date format (for example, `2020-06-14`). The date-time format is also accepted, but only with a time set to midnight (for example, `2020-06-14T00:00:00.000Z`).
    pub fn new() -> ReadConsumptionAccountRequestFromDate {
        ReadConsumptionAccountRequestFromDate {}
    }
}