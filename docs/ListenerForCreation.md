# ListenerForCreation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backend_port** | **i32** | The port on which the backend VM is listening (between `1` and `65535`, both included). | 
**backend_protocol** | Option<**String**> | The protocol for routing traffic to backend VMs (`HTTP` \\| `HTTPS` \\| `TCP` \\| `SSL`). | [optional]
**load_balancer_port** | **i32** | The port on which the load balancer is listening (between `1` and `65535`, both included). | 
**load_balancer_protocol** | **String** | The routing protocol (`HTTP` \\| `HTTPS` \\| `TCP` \\| `SSL`). | 
**server_certificate_id** | Option<**String**> | The OUTSCALE Resource Name (ORN) of the server certificate. For more information, see [Resource Identifiers > OUTSCALE Resource Names (ORNs)](https://docs.outscale.com/en/userguide/Resource-Identifiers.html#_outscale_resource_names_orns).<br/> This parameter is required for `HTTPS` and `SSL` protocols. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


