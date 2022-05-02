/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.19
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// ServerCertificate : Information about the server certificate.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerCertificate {
    /// The date at which the server certificate expires.
    #[serde(rename = "ExpirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// The ID of the server certificate.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the server certificate.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The path to the server certificate.
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The date at which the server certificate has been uploaded.
    #[serde(rename = "UploadDate", skip_serializing_if = "Option::is_none")]
    pub upload_date: Option<String>,
}

impl ServerCertificate {
    /// Information about the server certificate.
    pub fn new() -> ServerCertificate {
        ServerCertificate {
            expiration_date: None,
            id: None,
            name: None,
            path: None,
            upload_date: None,
        }
    }
}
