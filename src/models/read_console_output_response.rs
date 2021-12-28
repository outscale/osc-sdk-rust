/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://wiki.outscale.net/display/EN/3DS+OUTSCALE+APIs+Reference).<br /><br />  You can also manage your resources using the [Cockpit](https://wiki.outscale.net/display/EN/About+Cockpit) web interface.
 *
 * The version of the OpenAPI document: 1.16
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReadConsoleOutputResponse {
    /// The Base64-encoded output of the console. If a command line tool is used, the output is decoded by the tool.
    #[serde(rename = "ConsoleOutput", skip_serializing_if = "Option::is_none")]
    pub console_output: Option<String>,
    #[serde(rename = "ResponseContext", skip_serializing_if = "Option::is_none")]
    pub response_context: Option<Box<crate::models::ResponseContext>>,
    /// The ID of the VM.
    #[serde(rename = "VmId", skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
}

impl ReadConsoleOutputResponse {
    pub fn new() -> ReadConsoleOutputResponse {
        ReadConsoleOutputResponse {
            console_output: None,
            response_context: None,
            vm_id: None,
        }
    }
}
