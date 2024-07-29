# UpdateLoadBalancerRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_log** | Option<[**crate::models::AccessLog**](AccessLog.md)> |  | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**health_check** | Option<[**crate::models::HealthCheck**](HealthCheck.md)> |  | [optional]
**load_balancer_name** | **String** | The name of the load balancer. | 
**load_balancer_port** | Option<**i32**> | The port on which the load balancer is listening (between `1` and `65535`, both included). This parameter is required if you want to update the server certificate. | [optional]
**policy_names** | Option<**Vec<String>**> | The name of the policy you want to enable for the listener. | [optional]
**public_ip** | Option<**String**> | (internet-facing only) The public IP you want to associate with the load balancer. The former public IP of the load balancer is then disassociated. If you specify an empty string and the former public IP belonged to you, it is disassociated and replaced by a public IP owned by 3DS OUTSCALE. | [optional]
**secured_cookies** | Option<**bool**> | If true, secure cookies are enabled for the load balancer. | [optional]
**security_groups** | Option<**Vec<String>**> | (Net only) One or more IDs of security groups you want to assign to the load balancer. You need to specify the already assigned security groups that you want to keep along with the new ones you are assigning. If the list is empty, the default security group of the Net is assigned to the load balancer. | [optional]
**server_certificate_id** | Option<**String**> | The OUTSCALE Resource Name (ORN) of the server certificate. For more information, see [Resource Identifiers > OUTSCALE Resource Names (ORNs)](https://docs.outscale.com/en/userguide/Resource-Identifiers.html#_outscale_resource_names_orns). If this parameter is specified, you must also specify the `LoadBalancerPort` parameter. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


