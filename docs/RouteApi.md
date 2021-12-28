# \RouteApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_route**](RouteApi.md#create_route) | **POST** /CreateRoute | 
[**delete_route**](RouteApi.md#delete_route) | **POST** /DeleteRoute | 
[**update_route**](RouteApi.md#update_route) | **POST** /UpdateRoute | 



## create_route

> crate::models::CreateRouteResponse create_route(create_route_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_route_request** | Option<[**CreateRouteRequest**](CreateRouteRequest.md)> |  |  |

### Return type

[**crate::models::CreateRouteResponse**](CreateRouteResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_route

> crate::models::DeleteRouteResponse delete_route(delete_route_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_route_request** | Option<[**DeleteRouteRequest**](DeleteRouteRequest.md)> |  |  |

### Return type

[**crate::models::DeleteRouteResponse**](DeleteRouteResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_route

> crate::models::UpdateRouteResponse update_route(update_route_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_route_request** | Option<[**UpdateRouteRequest**](UpdateRouteRequest.md)> |  |  |

### Return type

[**crate::models::UpdateRouteResponse**](UpdateRouteResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

