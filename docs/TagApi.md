# \TagApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tags**](TagApi.md#create_tags) | **POST** /CreateTags | 
[**delete_tags**](TagApi.md#delete_tags) | **POST** /DeleteTags | 
[**read_tags**](TagApi.md#read_tags) | **POST** /ReadTags | 



## create_tags

> crate::models::CreateTagsResponse create_tags(create_tags_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_tags_request** | Option<[**CreateTagsRequest**](CreateTagsRequest.md)> |  |  |

### Return type

[**crate::models::CreateTagsResponse**](CreateTagsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tags

> crate::models::DeleteTagsResponse delete_tags(delete_tags_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_tags_request** | Option<[**DeleteTagsRequest**](DeleteTagsRequest.md)> |  |  |

### Return type

[**crate::models::DeleteTagsResponse**](DeleteTagsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_tags

> crate::models::ReadTagsResponse read_tags(read_tags_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_tags_request** | Option<[**ReadTagsRequest**](ReadTagsRequest.md)> |  |  |

### Return type

[**crate::models::ReadTagsResponse**](ReadTagsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

