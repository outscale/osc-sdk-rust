# FiltersVmsState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**maintenance_event_codes** | Option<**Vec<String>**> | The code for the scheduled event (`system-reboot` \\| `system-maintenance`). | [optional]
**maintenance_event_descriptions** | Option<**Vec<String>**> | The description of the scheduled event. | [optional]
**maintenance_events_not_after** | Option<[**Vec<String>**](string.md)> | The latest time the event can end. | [optional]
**maintenance_events_not_before** | Option<[**Vec<String>**](string.md)> | The earliest time the event can start. | [optional]
**subregion_names** | Option<**Vec<String>**> | The names of the Subregions of the VMs. | [optional]
**vm_ids** | Option<**Vec<String>**> | One or more IDs of VMs. | [optional]
**vm_states** | Option<**Vec<String>**> | The states of the VMs (`pending` \\| `running` \\| `stopping` \\| `stopped` \\| `shutting-down` \\| `terminated` \\| `quarantine`). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


