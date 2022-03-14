/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.18
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersKeypair : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersKeypair {
    /// The fingerprints of the keypairs.
    #[serde(
        rename = "KeypairFingerprints",
        skip_serializing_if = "Option::is_none"
    )]
    pub keypair_fingerprints: Option<Vec<String>>,
    /// The names of the keypairs.
    #[serde(rename = "KeypairNames", skip_serializing_if = "Option::is_none")]
    pub keypair_names: Option<Vec<String>>,
}

impl FiltersKeypair {
    /// One or more filters.
    pub fn new() -> FiltersKeypair {
        FiltersKeypair {
            keypair_fingerprints: None,
            keypair_names: None,
        }
    }
}
