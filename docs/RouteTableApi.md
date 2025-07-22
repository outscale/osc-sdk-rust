# \RouteTableApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_route_table**](RouteTableApi.md#create_route_table) | **POST** /CreateRouteTable | 
[**delete_route_table**](RouteTableApi.md#delete_route_table) | **POST** /DeleteRouteTable | 
[**link_route_table**](RouteTableApi.md#link_route_table) | **POST** /LinkRouteTable | 
[**read_route_tables**](RouteTableApi.md#read_route_tables) | **POST** /ReadRouteTables | 
[**unlink_route_table**](RouteTableApi.md#unlink_route_table) | **POST** /UnlinkRouteTable | 
[**update_route_table_link**](RouteTableApi.md#update_route_table_link) | **POST** /UpdateRouteTableLink | 



## create_route_table

> crate::models::CreateRouteTableResponse create_route_table(create_route_table_request)


Creates a route table for a specified Net.<br /> You can then add routes and associate this route table with a Subnet.<br /><br /> For more information, see [About Route Tables](https://docs.outscale.com/en/userguide/About-Route-Tables.html).

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


Deletes a specified route table.<br /> Before deleting a route table, you must disassociate it from any Subnet. You cannot delete the main route table.

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


Associates a Subnet with a route table.<br /> The Subnet and the route table must be in the same Net. The traffic is routed according to the route table defined within this Net. You can associate a route table with several Subnets.

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


Lists one or more of your route tables.<br /> In your Net, each Subnet must be associated with a route table. If a Subnet is not explicitly associated with a route table, it is implicitly associated with the main route table of the Net.

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


Disassociates a Subnet from a route table.<br /> After disassociation, the Subnet can no longer use the routes in this route table, but uses the routes in the main route table of the Net instead.

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


## update_route_table_link

> crate::models::UpdateRouteTableLinkResponse update_route_table_link(update_route_table_link_request)


Replaces the route table associated with a specific Subnet in a Net with another one.<br /> After the route table is replaced, the Subnet uses the routes in the new route table it is associated with.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_route_table_link_request** | Option<[**UpdateRouteTableLinkRequest**](UpdateRouteTableLinkRequest.md)> |  |  |

### Return type

[**crate::models::UpdateRouteTableLinkResponse**](UpdateRouteTableLinkResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

