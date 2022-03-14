/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.18
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Phase2Options : Information about Phase 2 of the Internet Key Exchange (IKE) negotiation.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Phase2Options {
    /// The Diffie-Hellman (DH) group numbers allowed for the VPN tunnel for phase 2.
    #[serde(
        rename = "Phase2DhGroupNumbers",
        skip_serializing_if = "Option::is_none"
    )]
    pub phase2_dh_group_numbers: Option<Vec<i32>>,
    /// The encryption algorithms allowed for the VPN tunnel for phase 2.
    #[serde(
        rename = "Phase2EncryptionAlgorithms",
        skip_serializing_if = "Option::is_none"
    )]
    pub phase2_encryption_algorithms: Option<Vec<String>>,
    /// The integrity algorithms allowed for the VPN tunnel for phase 2.
    #[serde(
        rename = "Phase2IntegrityAlgorithms",
        skip_serializing_if = "Option::is_none"
    )]
    pub phase2_integrity_algorithms: Option<Vec<String>>,
    /// The lifetime for phase 2 of the Internet Key Exchange (IKE) negociation process, in seconds.
    #[serde(
        rename = "Phase2LifetimeSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub phase2_lifetime_seconds: Option<i32>,
    /// The pre-shared key to establish the initial authentication between the client gateway and the virtual gateway. This key can contain any character except line breaks and double quotes (&quot;).
    #[serde(rename = "PreSharedKey", skip_serializing_if = "Option::is_none")]
    pub pre_shared_key: Option<String>,
}

impl Phase2Options {
    /// Information about Phase 2 of the Internet Key Exchange (IKE) negotiation.
    pub fn new() -> Phase2Options {
        Phase2Options {
            phase2_dh_group_numbers: None,
            phase2_encryption_algorithms: None,
            phase2_integrity_algorithms: None,
            phase2_lifetime_seconds: None,
            pre_shared_key: None,
        }
    }
}
