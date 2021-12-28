# CreateLoadBalancerPolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cookie_expiration_period** | Option<**i32**> | The lifetime of the cookie, in seconds. If not specified, the default value of this parameter is 1, which means that the sticky session lasts for the duration of the browser session. | [optional]
**cookie_name** | Option<**String**> | The name of the application cookie used for stickiness. This parameter is required if you create a stickiness policy based on an application-generated cookie. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**load_balancer_name** | **String** | The name of the load balancer for which you want to create a policy. | 
**policy_name** | **String** | The name of the policy. This name must be unique and consist of alphanumeric characters and dashes (-). | 
**policy_type** | **String** | The type of stickiness policy you want to create: `app` or `load_balancer`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


