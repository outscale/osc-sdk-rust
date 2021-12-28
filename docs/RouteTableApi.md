# \RouteTableApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_route_table**](RouteTableApi.md#create_route_table) | **POST** /CreateRouteTable | 
[**delete_route_table**](RouteTableApi.md#delete_route_table) | **POST** /DeleteRouteTable | 
[**link_route_table**](RouteTableApi.md#link_route_table) | **POST** /LinkRouteTable | 
[**read_route_tables**](RouteTableApi.md#read_route_tables) | **POST** /ReadRouteTables | 
[**unlink_route_table**](RouteTableApi.md#unlink_route_table) | **POST** /UnlinkRouteTable | 



## create_route_table

> crate::models::CreateRouteTableResponse create_route_table(create_route_table_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_route_table_request** | Option<[**CreateRouteTableRequest**](CreateRouteTableRequest.md)> |  |  |

### Return type

[**crate::models::CreateRouteTableResponse**](CreateRouteTableResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_route_table

> crate::models::DeleteRouteTableResponse delete_route_table(delete_route_table_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_route_table_request** | Option<[**DeleteRouteTableRequest**](DeleteRouteTableRequest.md)> |  |  |

### Return type

[**crate::models::DeleteRouteTableResponse**](DeleteRouteTableResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_route_table

> crate::models::LinkRouteTableResponse link_route_table(link_route_table_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_route_table_request** | Option<[**LinkRouteTableRequest**](LinkRouteTableRequest.md)> |  |  |

### Return type

[**crate::models::LinkRouteTableResponse**](LinkRouteTableResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_route_tables

> crate::models::ReadRouteTablesResponse read_route_tables(read_route_tables_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_route_tables_request** | Option<[**ReadRouteTablesRequest**](ReadRouteTablesRequest.md)> |  |  |

### Return type

[**crate::models::ReadRouteTablesResponse**](ReadRouteTablesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_route_table

> crate::models::UnlinkRouteTableResponse unlink_route_table(unlink_route_table_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unlink_route_table_request** | Option<[**UnlinkRouteTableRequest**](UnlinkRouteTableRequest.md)> |  |  |

### Return type

[**crate::models::UnlinkRouteTableResponse**](UnlinkRouteTableResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

