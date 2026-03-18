# PrivateIpLight

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_primary** | Option<**bool**> | If true, the IP is the primary private IP of the NIC. | [optional]
**private_ip** | Option<**String**> | A private IP for the NIC. This IP must be within the IP range of the Subnet that you specify with the `SubnetId` parameter. However, it cannot be one of the first four IPs (ending in `.0`, `.1`, `.2`, `.3`) or the last IP (ending in `.255`) of the Subnet, as these are reserved by 3DS OUTSCALE. For more information, see [About Nets](https://docs.outscale.com/en/userguide/About-Nets.html). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


