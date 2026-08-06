# FiltersVmsStopHistory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state_reasons** | Option<**Vec<String>**> | The reason explaining why the VM stopped. You can filter by reason code or reason prefix (for example, `Client.ApiGracefulShutdown` or `Client.*`). For the list of reason codes, see [Creating VMs > VM State Reference](https://docs.outscale.com/en/userguide/Creating-VMs). | [optional]
**stop_date_after** | Option<[**String**](string.md)> | The date and time (UTC), or the date, after which you want to retrieve VM stops, in ISO 8601 format (for example, `2026-06-14T00:00:00.000Z` or `2026-06-14`). | [optional]
**stop_date_before** | Option<[**String**](string.md)> | The date and time (UTC), or the date, before which you want to retrieve VM stops, in ISO 8601 format (for example, `2026-06-14T00:00:00.000Z` or `2026-06-14`). | [optional]
**vm_ids** | Option<**Vec<String>**> | The IDs of the stopped VM(s). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


