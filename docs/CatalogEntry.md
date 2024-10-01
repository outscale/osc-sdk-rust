# CatalogEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | Option<**String**> | The category of the catalog entry (for example, `network`). | [optional]
**flags** | Option<**String**> | When returned and equal to `PER_MONTH`, the price of the catalog entry is calculated on a monthly basis. | [optional]
**operation** | Option<**String**> | The API call associated with the catalog entry (for example, `CreateVms` or `RunInstances`). | [optional]
**service** | Option<**String**> | The service associated with the catalog entry (`TinaOS-FCU`, `TinaOS-LBU`, `TinaOS-DirectLink`, or `TinaOS-OOS`). | [optional]
**subregion_name** | Option<**String**> | The Subregion associated with the catalog entry. | [optional]
**title** | Option<**String**> | The description of the catalog entry. | [optional]
**_type** | Option<**String**> | The type of resource associated with the catalog entry. | [optional]
**unit_price** | Option<**f32**> | The unit price of the catalog entry in the currency of your account, in the ISO-4217 format (for example, `EUR`). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


