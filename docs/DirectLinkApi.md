# \DirectLinkApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_direct_link**](DirectLinkApi.md#create_direct_link) | **POST** /CreateDirectLink | 
[**delete_direct_link**](DirectLinkApi.md#delete_direct_link) | **POST** /DeleteDirectLink | 
[**read_direct_links**](DirectLinkApi.md#read_direct_links) | **POST** /ReadDirectLinks | 



## create_direct_link

> crate::models::CreateDirectLinkResponse create_direct_link(create_direct_link_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_direct_link_request** | Option<[**CreateDirectLinkRequest**](CreateDirectLinkRequest.md)> |  |  |

### Return type

[**crate::models::CreateDirectLinkResponse**](CreateDirectLinkResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_direct_link

> crate::models::DeleteDirectLinkResponse delete_direct_link(delete_direct_link_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_direct_link_request** | Option<[**DeleteDirectLinkRequest**](DeleteDirectLinkRequest.md)> |  |  |

### Return type

[**crate::models::DeleteDirectLinkResponse**](DeleteDirectLinkResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_direct_links

> crate::models::ReadDirectLinksResponse read_direct_links(read_direct_links_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_direct_links_request** | Option<[**ReadDirectLinksRequest**](ReadDirectLinksRequest.md)> |  |  |

### Return type

[**crate::models::ReadDirectLinksResponse**](ReadDirectLinksResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

