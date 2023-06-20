/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.27
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateVmGroupResponse {
    #[serde(rename = "ResponseContext", skip_serializing_if = "Option::is_none")]
    pub response_context: Option<Box<crate::models::ResponseContext>>,
    #[serde(rename = "VmGroup", skip_serializing_if = "Option::is_none")]
    pub vm_group: Option<Box<crate::models::VmGroup>>,
}

impl UpdateVmGroupResponse {
    pub fn new() -> UpdateVmGroupResponse {
        UpdateVmGroupResponse {
            response_context: None,
            vm_group: None,
        }
    }
}
