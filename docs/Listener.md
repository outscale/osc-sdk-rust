# Listener

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backend_port** | Option<**i32**> | The port on which the back-end VM is listening (between `1` and `65535`, both included). | [optional]
**backend_protocol** | Option<**String**> | The protocol for routing traffic to back-end VMs (`HTTP` \\| `HTTPS` \\| `TCP` \\| `SSL`). | [optional]
**load_balancer_port** | Option<**i32**> | The port on which the load balancer is listening (between 1 and `65535`, both included). | [optional]
**load_balancer_protocol** | Option<**String**> | The routing protocol (`HTTP` \\| `HTTPS` \\| `TCP` \\| `SSL`). | [optional]
**policy_names** | Option<**Vec<String>**> | The names of the policies. If there are no policies enabled, the list is empty. | [optional]
**server_certificate_id** | Option<**String**> | The OUTSCALE Resource Name (ORN) of the server certificate. For more information, see [Resource Identifiers > OUTSCALE Resource Names (ORNs)](https://wiki.outscale.net/display/EN/Resource+Identifiers#ResourceIdentifiers-ORNFormat). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


