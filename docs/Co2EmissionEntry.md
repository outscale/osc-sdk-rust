# Co2EmissionEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The ID of the account associated with the consumption. | [optional]
**category_distribution** | Option<[**Vec<crate::models::Co2CategoryDistribution>**](CO2CategoryDistribution.md)> | The allocation of the `Value` among categories. | [optional]
**factor_distribution** | Option<[**Vec<crate::models::Co2FactorDistribution>**](CO2FactorDistribution.md)> | The allocation of the `Value` among factors. | [optional]
**month** | Option<[**String**](string.md)> | The month associated with the CO2 emission entry. | [optional]
**paying_account_id** | Option<**String**> | The ID of the paying account related to the `AccountId` parameter. | [optional]
**value** | Option<**f64**> | The total CO2 emissions for the `Month` and `AccountId` specified. This value corresponds to the sum of all entries in `CategoryDistribution` and `FactorDistributionEntry`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


