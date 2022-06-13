# ReadConsumptionAccountRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**from_date** | **String** | The beginning of the time period, in ISO 8601 date-time format (for example, `2017-06-14` or `2017-06-14T00:00:00Z`). | 
**overall** | Option<**bool**> | By default or if true, returns either the overall consumption of your paying account and all linked accounts (if the account that sends this request is a paying account) or returns nothing (if the account that sends this request is a linked account). If false, returns only the consumption of the specific account that sends this request. | [optional][default to true]
**to_date** | **String** | The end of the time period, in ISO 8601 date-time format (for example, `2017-06-30` or `2017-06-30T00:00:00Z`). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


