# \LoadBalancerPolicyApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_load_balancer_policy**](LoadBalancerPolicyApi.md#create_load_balancer_policy) | **POST** /CreateLoadBalancerPolicy | 
[**delete_load_balancer_policy**](LoadBalancerPolicyApi.md#delete_load_balancer_policy) | **POST** /DeleteLoadBalancerPolicy | 



## create_load_balancer_policy

> crate::models::CreateLoadBalancerPolicyResponse create_load_balancer_policy(create_load_balancer_policy_request)


Creates a stickiness policy with sticky session lifetimes defined by the browser lifetime.<br /> The created policy can be used with HTTP or HTTPS listeners only.<br /> If this policy is implemented by a load balancer, this load balancer uses this cookie in all incoming requests to direct them to the specified backend server virtual machine (VM). If this cookie is not present, the load balancer sends the request to any other server according to its load-balancing algorithm.<br /><br />  You can also create a stickiness policy with sticky session lifetimes following the lifetime of an application-generated cookie.<br /> Unlike the other type of stickiness policy, the lifetime of the special Load Balancer Unit (LBU) cookie follows the lifetime of the application-generated cookie specified in the policy configuration. The load balancer inserts a new stickiness cookie only when the application response includes a new application cookie.<br /> The session stops being sticky if the application cookie is removed or expires, until a new application cookie is issued.<br /><br /> For more information, see [About Load Balancers](https://docs.outscale.com/en/userguide/About-Load-Balancers.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_load_balancer_policy_request** | Option<[**CreateLoadBalancerPolicyRequest**](CreateLoadBalancerPolicyRequest.md)> |  |  |

### Return type

[**crate::models::CreateLoadBalancerPolicyResponse**](CreateLoadBalancerPolicyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_load_balancer_policy

> crate::models::DeleteLoadBalancerPolicyResponse delete_load_balancer_policy(delete_load_balancer_policy_request)


Deletes a specified policy from a load balancer.<br /> In order to be deleted, the policy must not be enabled for any listener.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_load_balancer_policy_request** | Option<[**DeleteLoadBalancerPolicyRequest**](DeleteLoadBalancerPolicyRequest.md)> |  |  |

### Return type

[**crate::models::DeleteLoadBalancerPolicyResponse**](DeleteLoadBalancerPolicyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

