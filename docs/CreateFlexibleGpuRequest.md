# CreateFlexibleGpuRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delete_on_vm_deletion** | Option<**bool**> | If true, the fGPU is deleted when the VM is terminated. | [optional][default to false]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**generation** | Option<**String**> | The processor generation that the fGPU must be compatible with. If not specified, the oldest possible processor generation is selected (as provided by [ReadFlexibleGpuCatalog](#readflexiblegpucatalog) for the specified model of fGPU). | [optional]
**model_name** | **String** | The model of fGPU you want to allocate. For more information, see [About Flexible GPUs](https://docs.outscale.com/en/userguide/About-Flexible-GPUs.html). | 
**subregion_name** | **String** | The Subregion in which you want to create the fGPU. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


