# Net

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dhcp_options_set_id** | Option<**String**> | The ID of the DHCP options set (or `default` if you want to associate the default one). | [optional]
**ip_range** | Option<**String**> | The IP range for the Net, in CIDR notation (for example, `10.0.0.0/16`). | [optional]
**net_id** | Option<**String**> | The ID of the Net. | [optional]
**state** | Option<**String**> | The state of the Net (`pending` \\| `available`). | [optional]
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags associated with the Net. | [optional]
**tenancy** | Option<**String**> | The VM tenancy in a Net. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


