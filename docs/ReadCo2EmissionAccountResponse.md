# ReadCo2EmissionAccountResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**co2_emission_entries** | Option<[**Vec<crate::models::Co2EmissionEntry>**](CO2EmissionEntry.md)> | The CO2 emission by month and account, for the specified request. | [optional]
**response_context** | Option<[**crate::models::ResponseContext**](ResponseContext.md)> |  | [optional]
**unit** | Option<**String**> | The unit of all the `Value` fields of the response, expressed in kgCO₂e. | [optional]
**value** | Option<**f64**> | The total CO2 emission for the specified request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


