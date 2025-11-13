# ReadCo2EmissionAccountRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_month** | [**String**](string.md) | The beginning of the time period, in ISO 8601 date format (for example, `2020-06-01`). This value must correspond to the first day of the month and is included in the time period. | 
**overall** | Option<**bool**> | If false, returns only the CO2 emission of the specific account that sends the request. If true, returns either the overall CO2 emission of your paying account and all linked accounts (if the account that sends this request is a paying account) or returns nothing (if the account that sends this request is a linked account). | [optional][default to false]
**to_month** | [**String**](string.md) | The end of the time period, in ISO 8601 date format (for example, `2020-06-14`). This value must correspond to the first day of the month and is excluded from the time period. It must be set to a later date than `FromMonth`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


