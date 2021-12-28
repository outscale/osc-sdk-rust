# \NetAccessPointApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_net_access_point**](NetAccessPointApi.md#create_net_access_point) | **POST** /CreateNetAccessPoint | 
[**delete_net_access_point**](NetAccessPointApi.md#delete_net_access_point) | **POST** /DeleteNetAccessPoint | 
[**read_net_access_point_services**](NetAccessPointApi.md#read_net_access_point_services) | **POST** /ReadNetAccessPointServices | 
[**read_net_access_points**](NetAccessPointApi.md#read_net_access_points) | **POST** /ReadNetAccessPoints | 
[**update_net_access_point**](NetAccessPointApi.md#update_net_access_point) | **POST** /UpdateNetAccessPoint | 



## create_net_access_point

> crate::models::CreateNetAccessPointResponse create_net_access_point(create_net_access_point_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_net_access_point_request** | Option<[**CreateNetAccessPointRequest**](CreateNetAccessPointRequest.md)> |  |  |

### Return type

[**crate::models::CreateNetAccessPointResponse**](CreateNetAccessPointResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_net_access_point

> crate::models::DeleteNetAccessPointResponse delete_net_access_point(delete_net_access_point_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_net_access_point_request** | Option<[**DeleteNetAccessPointRequest**](DeleteNetAccessPointRequest.md)> |  |  |

### Return type

[**crate::models::DeleteNetAccessPointResponse**](DeleteNetAccessPointResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_net_access_point_services

> crate::models::ReadNetAccessPointServicesResponse read_net_access_point_services(read_net_access_point_services_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_net_access_point_services_request** | Option<[**ReadNetAccessPointServicesRequest**](ReadNetAccessPointServicesRequest.md)> |  |  |

### Return type

[**crate::models::ReadNetAccessPointServicesResponse**](ReadNetAccessPointServicesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_net_access_points

> crate::models::ReadNetAccessPointsResponse read_net_access_points(read_net_access_points_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_net_access_points_request** | Option<[**ReadNetAccessPointsRequest**](ReadNetAccessPointsRequest.md)> |  |  |

### Return type

[**crate::models::ReadNetAccessPointsResponse**](ReadNetAccessPointsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_net_access_point

> crate::models::UpdateNetAccessPointResponse update_net_access_point(update_net_access_point_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_net_access_point_request** | Option<[**UpdateNetAccessPointRequest**](UpdateNetAccessPointRequest.md)> |  |  |

### Return type

[**crate::models::UpdateNetAccessPointResponse**](UpdateNetAccessPointResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

