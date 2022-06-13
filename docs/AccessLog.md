# AccessLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_enabled** | Option<**bool**> | If true, access logs are enabled for your load balancer. If false, they are not. If you set this to true in your request, the `OsuBucketName` parameter is required. | [optional]
**osu_bucket_name** | Option<**String**> | The name of the OOS bucket for the access logs. | [optional]
**osu_bucket_prefix** | Option<**String**> | The path to the folder of the access logs in your OOS bucket (by default, the `root` level of your bucket). | [optional]
**publication_interval** | Option<**i32**> | The time interval for the publication of access logs in the OOS bucket, in minutes. This value can be either `5` or `60` (by default, `60`). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


