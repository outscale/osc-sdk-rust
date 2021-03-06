/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateServerCertificateRequest {
    /// The PEM-encoded X509 certificate.
    #[serde(rename = "Body")]
    pub body: String,
    /// The PEM-encoded intermediate certification authorities.
    #[serde(rename = "Chain", skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// A unique name for the certificate. Constraints: 1-128 alphanumeric characters, pluses (+), equals (=), commas (,), periods (.), at signs (@), minuses (-), or underscores (_).
    #[serde(rename = "Name")]
    pub name: String,
    /// The path to the server certificate, set to a slash (/) if not specified.
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The PEM-encoded private key matching the certificate.
    #[serde(rename = "PrivateKey")]
    pub private_key: String,
}

impl CreateServerCertificateRequest {
    pub fn new(body: String, name: String, private_key: String) -> CreateServerCertificateRequest {
        CreateServerCertificateRequest {
            body,
            chain: None,
            dry_run: None,
            name,
            path: None,
            private_key,
        }
    }
}
