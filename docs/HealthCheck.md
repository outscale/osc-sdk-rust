# HealthCheck

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**check_interval** | **i32** | The number of seconds between two pings (between `5` and `600` both included). | 
**healthy_threshold** | **i32** | The number of consecutive successful pings before considering the VM as healthy (between `2` and `10` both included). | 
**path** | Option<**String**> | If you use the HTTP or HTTPS protocols, the ping path. | [optional]
**port** | **i32** | The port number (between `1` and `65535`, both included). | 
**protocol** | **String** | The protocol for the URL of the VM (`HTTP` \\| `HTTPS` \\| `TCP` \\| `SSL`). | 
**timeout** | **i32** | The maximum waiting time for a response before considering the VM as unhealthy, in seconds (between `2` and `60` both included). | 
**unhealthy_threshold** | **i32** | The number of consecutive failed pings before considering the VM as unhealthy (between `2` and `10` both included). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


