/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.21
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// KeypairCreated : Information about the created keypair.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct KeypairCreated {
    /// The MD5 public key fingerprint as specified in section 4 of RFC 4716.
    #[serde(rename = "KeypairFingerprint", skip_serializing_if = "Option::is_none")]
    pub keypair_fingerprint: Option<String>,
    /// The name of the keypair.
    #[serde(rename = "KeypairName", skip_serializing_if = "Option::is_none")]
    pub keypair_name: Option<String>,
    /// The private key. When saving the private key in a .rsa file, replace the `\\n` escape sequences with line breaks.
    #[serde(rename = "PrivateKey", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

impl KeypairCreated {
    /// Information about the created keypair.
    pub fn new() -> KeypairCreated {
        KeypairCreated {
            keypair_fingerprint: None,
            keypair_name: None,
            private_key: None,
        }
    }
}
