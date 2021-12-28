# \LoadBalancerApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_load_balancer**](LoadBalancerApi.md#create_load_balancer) | **POST** /CreateLoadBalancer | 
[**create_load_balancer_tags**](LoadBalancerApi.md#create_load_balancer_tags) | **POST** /CreateLoadBalancerTags | 
[**delete_load_balancer**](LoadBalancerApi.md#delete_load_balancer) | **POST** /DeleteLoadBalancer | 
[**delete_load_balancer_tags**](LoadBalancerApi.md#delete_load_balancer_tags) | **POST** /DeleteLoadBalancerTags | 
[**deregister_vms_in_load_balancer**](LoadBalancerApi.md#deregister_vms_in_load_balancer) | **POST** /DeregisterVmsInLoadBalancer | 
[**read_load_balancer_tags**](LoadBalancerApi.md#read_load_balancer_tags) | **POST** /ReadLoadBalancerTags | 
[**read_load_balancers**](LoadBalancerApi.md#read_load_balancers) | **POST** /ReadLoadBalancers | 
[**read_vms_health**](LoadBalancerApi.md#read_vms_health) | **POST** /ReadVmsHealth | 
[**register_vms_in_load_balancer**](LoadBalancerApi.md#register_vms_in_load_balancer) | **POST** /RegisterVmsInLoadBalancer | 
[**update_load_balancer**](LoadBalancerApi.md#update_load_balancer) | **POST** /UpdateLoadBalancer | 



## create_load_balancer

> crate::models::CreateLoadBalancerResponse create_load_balancer(create_load_balancer_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_load_balancer_request** | Option<[**CreateLoadBalancerRequest**](CreateLoadBalancerRequest.md)> |  |  |

### Return type

[**crate::models::CreateLoadBalancerResponse**](CreateLoadBalancerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_load_balancer_tags

> crate::models::CreateLoadBalancerTagsResponse create_load_balancer_tags(create_load_balancer_tags_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_load_balancer_tags_request** | Option<[**CreateLoadBalancerTagsRequest**](CreateLoadBalancerTagsRequest.md)> |  |  |

### Return type

[**crate::models::CreateLoadBalancerTagsResponse**](CreateLoadBalancerTagsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_load_balancer

> crate::models::DeleteLoadBalancerResponse delete_load_balancer(delete_load_balancer_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_load_balancer_request** | Option<[**DeleteLoadBalancerRequest**](DeleteLoadBalancerRequest.md)> |  |  |

### Return type

[**crate::models::DeleteLoadBalancerResponse**](DeleteLoadBalancerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_load_balancer_tags

> crate::models::DeleteLoadBalancerTagsResponse delete_load_balancer_tags(delete_load_balancer_tags_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_load_balancer_tags_request** | Option<[**DeleteLoadBalancerTagsRequest**](DeleteLoadBalancerTagsRequest.md)> |  |  |

### Return type

[**crate::models::DeleteLoadBalancerTagsResponse**](DeleteLoadBalancerTagsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deregister_vms_in_load_balancer

> crate::models::DeregisterVmsInLoadBalancerResponse deregister_vms_in_load_balancer(deregister_vms_in_load_balancer_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deregister_vms_in_load_balancer_request** | Option<[**DeregisterVmsInLoadBalancerRequest**](DeregisterVmsInLoadBalancerRequest.md)> |  |  |

### Return type

[**crate::models::DeregisterVmsInLoadBalancerResponse**](DeregisterVmsInLoadBalancerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_load_balancer_tags

> crate::models::ReadLoadBalancerTagsResponse read_load_balancer_tags(read_load_balancer_tags_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_load_balancer_tags_request** | Option<[**ReadLoadBalancerTagsRequest**](ReadLoadBalancerTagsRequest.md)> |  |  |

### Return type

[**crate::models::ReadLoadBalancerTagsResponse**](ReadLoadBalancerTagsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_load_balancers

> crate::models::ReadLoadBalancersResponse read_load_balancers(read_load_balancers_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_load_balancers_request** | Option<[**ReadLoadBalancersRequest**](ReadLoadBalancersRequest.md)> |  |  |

### Return type

[**crate::models::ReadLoadBalancersResponse**](ReadLoadBalancersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_vms_health

> crate::models::ReadVmsHealthResponse read_vms_health(read_vms_health_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_vms_health_request** | Option<[**ReadVmsHealthRequest**](ReadVmsHealthRequest.md)> |  |  |

### Return type

[**crate::models::ReadVmsHealthResponse**](ReadVmsHealthResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_vms_in_load_balancer

> crate::models::RegisterVmsInLoadBalancerResponse register_vms_in_load_balancer(register_vms_in_load_balancer_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_vms_in_load_balancer_request** | Option<[**RegisterVmsInLoadBalancerRequest**](RegisterVmsInLoadBalancerRequest.md)> |  |  |

### Return type

[**crate::models::RegisterVmsInLoadBalancerResponse**](RegisterVmsInLoadBalancerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_load_balancer

> crate::models::UpdateLoadBalancerResponse update_load_balancer(update_load_balancer_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_load_balancer_request** | Option<[**UpdateLoadBalancerRequest**](UpdateLoadBalancerRequest.md)> |  |  |

### Return type

[**crate::models::UpdateLoadBalancerResponse**](UpdateLoadBalancerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

