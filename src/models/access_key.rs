/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// AccessKey : Information about the access key.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccessKey {
    /// The ID of the access key.
    #[serde(rename = "AccessKeyId", skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    /// The date and time (UTC) of creation of the access key.
    #[serde(rename = "CreationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// The date (UTC) at which the access key expires.
    #[serde(rename = "ExpirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// The date and time (UTC) of the last modification of the access key.
    #[serde(
        rename = "LastModificationDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modification_date: Option<String>,
    /// The state of the access key (`ACTIVE` if the key is valid for API calls, or `INACTIVE` if not).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl AccessKey {
    /// Information about the access key.
    pub fn new() -> AccessKey {
        AccessKey {
            access_key_id: None,
            creation_date: None,
            expiration_date: None,
            last_modification_date: None,
            state: None,
        }
    }
}
