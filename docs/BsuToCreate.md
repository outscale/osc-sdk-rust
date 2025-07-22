# BsuToCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delete_on_vm_deletion** | Option<**bool**> | If set to true, the volume is deleted when terminating the VM. If false, the volume is not deleted when terminating the VM. | [optional][default to true]
**iops** | Option<**i32**> | The number of I/O operations per second (IOPS). This parameter must be specified only if you create an `io1` volume. The maximum number of IOPS allowed for `io1` volumes is `13000` with a maximum performance ratio of 300 IOPS per gibibyte. | [optional]
**snapshot_id** | Option<**String**> | The ID of the snapshot used to create the volume. | [optional]
**volume_size** | Option<**i32**> | The size of the volume, in gibibytes (GiB).<br /> If you specify a snapshot ID, the volume size must be at least equal to the snapshot size.<br /> If you specify a snapshot ID but no volume size, the volume is created with a size similar to the snapshot one. | [optional]
**volume_type** | Option<**String**> | The type of the volume (`standard` \\| `io1` \\| `gp2`). If not specified in the request, a `standard` volume is created.<br /> For more information about volume types, see [About Volumes > Volume Types and IOPS](https://docs.outscale.com/en/userguide/About-Volumes.html#_volume_types_and_iops). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


