/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.24
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Placement : Information about the placement of the VM.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Placement {
    /// The name of the Subregion. If you specify this parameter, you must not specify the `Nics` parameter.
    #[serde(rename = "SubregionName", skip_serializing_if = "Option::is_none")]
    pub subregion_name: Option<String>,
    /// The tenancy of the VM (`default` \\| `dedicated`).
    #[serde(rename = "Tenancy", skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<String>,
}

impl Placement {
    /// Information about the placement of the VM.
    pub fn new() -> Placement {
        Placement {
            subregion_name: None,
            tenancy: None,
        }
    }
}
