# Phase1Options

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dpd_timeout_action** | Option<**String**> | The action to carry out after a Dead Peer Detection (DPD) timeout occurs. | [optional]
**dpd_timeout_seconds** | Option<**i32**> | The maximum waiting time for a Dead Peer Detection (DPD) response before considering the peer as dead, in seconds. | [optional]
**ike_versions** | Option<**Vec<String>**> | The Internet Key Exchange (IKE) versions allowed for the VPN tunnel. | [optional]
**phase1_dh_group_numbers** | Option<**Vec<i32>**> | The Diffie-Hellman (DH) group numbers allowed for the VPN tunnel for phase 1. | [optional]
**phase1_encryption_algorithms** | Option<**Vec<String>**> | The encryption algorithms allowed for the VPN tunnel for phase 1. | [optional]
**phase1_integrity_algorithms** | Option<**Vec<String>**> | The integrity algorithms allowed for the VPN tunnel for phase 1. | [optional]
**phase1_lifetime_seconds** | Option<**i32**> | The lifetime for phase 1 of the IKE negotiation process, in seconds. | [optional]
**replay_window_size** | Option<**i32**> | The number of packets in an IKE replay window. | [optional]
**startup_action** | Option<**String**> | The action to carry out when establishing tunnels for a VPN connection. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


