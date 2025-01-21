# Phase2Options

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**phase2_dh_group_numbers** | Option<**Vec<i32>**> | The Diffie-Hellman (DH) group numbers allowed for the VPN tunnel for phase 2. | [optional]
**phase2_encryption_algorithms** | Option<**Vec<String>**> | The encryption algorithms allowed for the VPN tunnel for phase 2. | [optional]
**phase2_integrity_algorithms** | Option<**Vec<String>**> | The integrity algorithms allowed for the VPN tunnel for phase 2. | [optional]
**phase2_lifetime_seconds** | Option<**i32**> | The lifetime for phase 2 of the Internet Key Exchange (IKE) negotiation process, in seconds. | [optional]
**pre_shared_key** | Option<**String**> | The pre-shared key to establish the initial authentication between the client gateway and the virtual gateway. This key can contain any character except line breaks and double quotes (&quot;). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


