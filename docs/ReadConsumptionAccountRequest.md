# ReadConsumptionAccountRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**from_date** | **String** | The beginning of the time period, in ISO 8601 date format (for example, `2020-06-14`). The date-time format is also accepted, but only with a time set to midnight (for example, `2020-06-14T00:00:00.000Z`). This value is included in the time period. | 
**overall** | Option<**bool**> | If false, returns only the consumption of the specific account that sends this request. If true, returns either the overall consumption of your paying account and all linked accounts (if the account that sends this request is a paying account) or returns nothing (if the account that sends this request is a linked account). | [optional][default to false]
**show_price** | Option<**bool**> | If true, the response also includes the unit price of the consumed resource (`UnitPrice`) and the total price of the consumed resource during the specified time period (`Price`), in the currency of your account. | [optional]
**to_date** | **String** | The end of the time period, in ISO 8601 date format (for example, `2020-06-30`). The date-time format is also accepted, but only with a time set to midnight (for example, `2020-06-30T00:00:00.000Z`). This value is excluded from the time period, and must be set to a later date than `FromDate`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


