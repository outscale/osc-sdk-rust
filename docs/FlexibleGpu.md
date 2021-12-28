# FlexibleGpu

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delete_on_vm_deletion** | Option<**bool**> | If true, the fGPU is deleted when the VM is terminated. | [optional]
**flexible_gpu_id** | Option<**String**> | The ID of the fGPU. | [optional]
**generation** | Option<**String**> | The compatible processor generation. | [optional]
**model_name** | Option<**String**> | The model of fGPU. For more information, see [About Flexible GPUs](https://wiki.outscale.net/display/EN/About+Flexible+GPUs). | [optional]
**state** | Option<**String**> | The state of the fGPU (`allocated` \\| `attaching` \\| `attached` \\| `detaching`). | [optional]
**subregion_name** | Option<**String**> | The Subregion where the fGPU is located. | [optional]
**vm_id** | Option<**String**> | The ID of the VM the fGPU is attached to, if any. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


