# CreateVmTemplateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu_cores** | **i32** | The number of vCores to use for each VM. | 
**cpu_generation** | **String** | The processor generation to use for each VM (for example, `v4`). | 
**cpu_performance** | Option<**String**> | The performance of the VMs (`medium` \\| `high` \\|  `highest`).  | [optional][default to CpuPerformance_High]
**description** | Option<**String**> | A description for the VM template. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**image_id** | **String** | The ID of the OMI to use for each VM. You can find a list of OMIs by calling the [ReadImages](#readimages) method. | 
**keypair_name** | Option<**String**> | The name of the keypair to use for each VM. | [optional]
**ram** | **i32** | The amount of RAM to use for each VM. | 
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags to add to the VM template. | [optional]
**vm_template_name** | **String** | The name of the VM template. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


