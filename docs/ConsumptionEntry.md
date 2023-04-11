# ConsumptionEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The ID of your TINA account. | [optional]
**category** | Option<**String**> | The category of the resource (for example, `network`). | [optional]
**from_date** | Option<**String**> | The beginning of the time period, in ISO 8601 date-time format. | [optional]
**operation** | Option<**String**> | The API call that triggered the resource consumption (for example, `RunInstances` or `CreateVolume`). | [optional]
**paying_account_id** | Option<**String**> | The ID of the TINA account which is billed for your consumption. It can be different from your account in the `AccountId` parameter. | [optional]
**service** | Option<**String**> | The service of the API call (`TinaOS-FCU`, `TinaOS-LBU`, `TinaOS-DirectLink`, `TinaOS-OOS`, or `TinaOS-OSU`). | [optional]
**subregion_name** | Option<**String**> | The name of the Subregion. | [optional]
**title** | Option<**String**> | A description of the consumed resource. | [optional]
**to_date** | Option<**String**> | The end of the time period, in ISO 8601 date-time format. | [optional]
**_type** | Option<**String**> | The type of resource, depending on the API call. | [optional]
**value** | Option<**f64**> | The consumed amount for the resource. The unit depends on the resource type. For more information, see the `Title` element. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


