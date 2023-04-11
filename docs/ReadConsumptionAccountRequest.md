# ReadConsumptionAccountRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**from_date** | [**crate::models::ReadConsumptionAccountRequestFromDate**](ReadConsumptionAccountRequest_FromDate.md) |  | 
**overall** | Option<**bool**> | By default or if false, returns only the consumption of the specific account that sends this request. If true, returns either the overall consumption of your paying account and all linked accounts (if the account that sends this request is a paying account) or returns nothing (if the account that sends this request is a linked account). | [optional][default to false]
**to_date** | [**crate::models::ReadConsumptionAccountRequestToDate**](ReadConsumptionAccountRequest_ToDate.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


