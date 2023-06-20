# ReadConsumptionAccountRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**from_date** | [**String**](string.md) | The beginning of the time period, in ISO 8601 date format (for example, `2020-06-14`). The date-time format is also accepted, but only with a time set to midnight (for example, `2020-06-14T00:00:00.000Z`). | 
**overall** | Option<**bool**> | By default or if false, returns only the consumption of the specific account that sends this request. If true, returns either the overall consumption of your paying account and all linked accounts (if the account that sends this request is a paying account) or returns nothing (if the account that sends this request is a linked account). | [optional][default to false]
**to_date** | [**String**](string.md) | The end of the time period, in ISO 8601 date format (for example, `2020-06-30`). The date-time format is also accepted, but only with a time set to midnight (for example, `2020-06-30T00:00:00.000Z`). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


