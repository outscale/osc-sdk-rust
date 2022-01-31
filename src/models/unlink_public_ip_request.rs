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
pub struct UnlinkPublicIpRequest {
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The ID representing the association of the public IP with the VM or the NIC. This parameter is required unless you use the `PublicIp` parameter.
    #[serde(rename = "LinkPublicIpId", skip_serializing_if = "Option::is_none")]
    pub link_public_ip_id: Option<String>,
    /// The public IP. This parameter is required unless you use the `LinkPublicIpId` parameter.
    #[serde(rename = "PublicIp", skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
}

impl UnlinkPublicIpRequest {
    pub fn new() -> UnlinkPublicIpRequest {
        UnlinkPublicIpRequest {
            dry_run: None,
            link_public_ip_id: None,
            public_ip: None,
        }
    }
}
