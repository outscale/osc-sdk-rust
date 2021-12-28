# UpdateVolumeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**iops** | Option<**i32**> | The new number of I/O operations per second (IOPS). This parameter can be specified only if you update an `io1` volume. The maximum number of IOPS allowed for `io1` volumes is `13000`. This modification is instantaneous on a cold volume, not on a hot one. | [optional]
**size** | Option<**i32**> | (cold volume only) The new size of the volume, in gibibytes (GiB). This value must be equal to or greater than the current size of the volume. This modification is not instantaneous. | [optional]
**volume_id** | **String** | The ID of the volume you want to update. | 
**volume_type** | Option<**String**> | (cold volume only) The new type of the volume (`standard` \\| `io1` \\| `gp2`). This modification is instantaneous. If you update to an `io1` volume, you must also specify the `Iops` parameter. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


