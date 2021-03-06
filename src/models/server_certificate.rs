/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
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
    /// The Outscale Resource Name (ORN) of the server certificate. For more information, see [Resource Identifiers > Outscale Resource Names (ORNs)](https://docs.outscale.com/en/userguide/Resource-Identifiers.html#_outscale_resource_names_orns).
    #[serde(rename = "Orn", skip_serializing_if = "Option::is_none")]
    pub orn: Option<String>,
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
            orn: None,
            path: None,
            upload_date: None,
        }
    }
}
