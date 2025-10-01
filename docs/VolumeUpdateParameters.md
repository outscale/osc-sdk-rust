# VolumeUpdateParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**iops** | Option<**i32**> | The new number of I/O operations per second (IOPS):<br /> - For `io1` volumes, the number of provisioned IOPS.<br /> - For `gp2` volumes, the baseline performance of the volume. | 
**size** | **i32** | The new size of the volume, in gibibytes (GiB). | 
**volume_type** | **String** | The type of the volume (`standard` \\| `io1` \\| `gp2`). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


