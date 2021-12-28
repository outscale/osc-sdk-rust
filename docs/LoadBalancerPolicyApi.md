# \LoadBalancerPolicyApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_load_balancer_policy**](LoadBalancerPolicyApi.md#create_load_balancer_policy) | **POST** /CreateLoadBalancerPolicy | 
[**delete_load_balancer_policy**](LoadBalancerPolicyApi.md#delete_load_balancer_policy) | **POST** /DeleteLoadBalancerPolicy | 



## create_load_balancer_policy

> crate::models::CreateLoadBalancerPolicyResponse create_load_balancer_policy(create_load_balancer_policy_request)


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

