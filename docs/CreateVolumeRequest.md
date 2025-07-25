# CreateVolumeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_token** | Option<**String**> | A unique identifier which enables you to manage the idempotency. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**iops** | Option<**i32**> | The number of I/O operations per second (IOPS). This parameter must be specified only if you create an `io1` volume. The maximum number of IOPS allowed for `io1` volumes is `13000` with a maximum performance ratio of 300 IOPS per gibibyte. | [optional]
**size** | Option<**i32**> | The size of the volume, in gibibytes (GiB). The maximum allowed size for a volume is 14901 GiB. This parameter is required if the volume is not created from a snapshot (`SnapshotId` unspecified). | [optional]
**snapshot_id** | Option<**String**> | The ID of the snapshot from which you want to create the volume. | [optional]
**subregion_name** | **String** | The Subregion in which you want to create the volume. | 
**volume_type** | Option<**String**> | The type of volume you want to create (`io1` \\| `gp2` \\| `standard`). If not specified, a `standard` volume is created.<br /> For more information about volume types, see [About Volumes > Volume Types and IOPS](https://docs.outscale.com/en/userguide/About-Volumes.html#_volume_types_and_iops). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


