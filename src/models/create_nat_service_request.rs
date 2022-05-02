/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.19
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateNatServiceRequest {
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The allocation ID of the public IP to associate with the NAT service.<br /> If the public IP is already associated with another resource, you must first disassociate it.
    #[serde(rename = "PublicIpId")]
    pub public_ip_id: String,
    /// The ID of the Subnet in which you want to create the NAT service.
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
}

impl CreateNatServiceRequest {
    pub fn new(public_ip_id: String, subnet_id: String) -> CreateNatServiceRequest {
        CreateNatServiceRequest {
            dry_run: None,
            public_ip_id,
            subnet_id,
        }
    }
}
