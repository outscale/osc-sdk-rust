/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.18
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Phase1Options : Information about Phase 1 of the Internet Key Exchange (IKE) negotiation. When Phase 1 finishes successfully, peers proceed to Phase 2 negotiations.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Phase1Options {
    /// The action to carry out after a Dead Peer Detection (DPD) timeout occurs.
    #[serde(rename = "DpdTimeoutAction", skip_serializing_if = "Option::is_none")]
    pub dpd_timeout_action: Option<String>,
    /// The maximum waiting time for a Dead Peer Detection (DPD) response before considering the peer as dead, in seconds.
    #[serde(rename = "DpdTimeoutSeconds", skip_serializing_if = "Option::is_none")]
    pub dpd_timeout_seconds: Option<i32>,
    /// The Internet Key Exchange (IKE) versions allowed for the VPN tunnel.
    #[serde(rename = "IkeVersions", skip_serializing_if = "Option::is_none")]
    pub ike_versions: Option<Vec<String>>,
    /// The Diffie-Hellman (DH) group numbers allowed for the VPN tunnel for phase 1.
    #[serde(
        rename = "Phase1DhGroupNumbers",
        skip_serializing_if = "Option::is_none"
    )]
    pub phase1_dh_group_numbers: Option<Vec<i32>>,
    /// The encryption algorithms allowed for the VPN tunnel for phase 1.
    #[serde(
        rename = "Phase1EncryptionAlgorithms",
        skip_serializing_if = "Option::is_none"
    )]
    pub phase1_encryption_algorithms: Option<Vec<String>>,
    /// The integrity algorithms allowed for the VPN tunnel for phase 1.
    #[serde(
        rename = "Phase1IntegrityAlgorithms",
        skip_serializing_if = "Option::is_none"
    )]
    pub phase1_integrity_algorithms: Option<Vec<String>>,
    /// The lifetime for phase 1 of the IKE negotiation process, in seconds.
    #[serde(
        rename = "Phase1LifetimeSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub phase1_lifetime_seconds: Option<i32>,
    /// The number of packets in an IKE replay window.
    #[serde(rename = "ReplayWindowSize", skip_serializing_if = "Option::is_none")]
    pub replay_window_size: Option<i32>,
    /// The action to carry out when establishing tunnels for a VPN connection.
    #[serde(rename = "StartupAction", skip_serializing_if = "Option::is_none")]
    pub startup_action: Option<String>,
}

impl Phase1Options {
    /// Information about Phase 1 of the Internet Key Exchange (IKE) negotiation. When Phase 1 finishes successfully, peers proceed to Phase 2 negotiations.
    pub fn new() -> Phase1Options {
        Phase1Options {
            dpd_timeout_action: None,
            dpd_timeout_seconds: None,
            ike_versions: None,
            phase1_dh_group_numbers: None,
            phase1_encryption_algorithms: None,
            phase1_integrity_algorithms: None,
            phase1_lifetime_seconds: None,
            replay_window_size: None,
            startup_action: None,
        }
    }
}
