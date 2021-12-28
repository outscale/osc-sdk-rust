# FiltersFlexibleGpu

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delete_on_vm_deletion** | Option<**bool**> | Indicates whether the fGPU is deleted when terminating the VM. | [optional]
**flexible_gpu_ids** | Option<**Vec<String>**> | One or more IDs of fGPUs. | [optional]
**generations** | Option<**Vec<String>**> | The processor generations that the fGPUs are compatible with. | [optional]
**model_names** | Option<**Vec<String>**> | One or more models of fGPUs. For more information, see [About Flexible GPUs](https://wiki.outscale.net/display/EN/About+Flexible+GPUs). | [optional]
**states** | Option<**Vec<String>**> | The states of the fGPUs (`allocated` \\| `attaching` \\| `attached` \\| `detaching`). | [optional]
**subregion_names** | Option<**Vec<String>**> | The Subregions where the fGPUs are located. | [optional]
**vm_ids** | Option<**Vec<String>**> | One or more IDs of VMs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


