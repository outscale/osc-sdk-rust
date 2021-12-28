# \PublicIpApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_public_ip**](PublicIpApi.md#create_public_ip) | **POST** /CreatePublicIp | 
[**delete_public_ip**](PublicIpApi.md#delete_public_ip) | **POST** /DeletePublicIp | 
[**link_public_ip**](PublicIpApi.md#link_public_ip) | **POST** /LinkPublicIp | 
[**read_public_ip_ranges**](PublicIpApi.md#read_public_ip_ranges) | **POST** /ReadPublicIpRanges | 
[**read_public_ips**](PublicIpApi.md#read_public_ips) | **POST** /ReadPublicIps | 
[**unlink_public_ip**](PublicIpApi.md#unlink_public_ip) | **POST** /UnlinkPublicIp | 



## create_public_ip

> crate::models::CreatePublicIpResponse create_public_ip(create_public_ip_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_public_ip_request** | Option<[**CreatePublicIpRequest**](CreatePublicIpRequest.md)> |  |  |

### Return type

[**crate::models::CreatePublicIpResponse**](CreatePublicIpResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_public_ip

> crate::models::DeletePublicIpResponse delete_public_ip(delete_public_ip_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_public_ip_request** | Option<[**DeletePublicIpRequest**](DeletePublicIpRequest.md)> |  |  |

### Return type

[**crate::models::DeletePublicIpResponse**](DeletePublicIpResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_public_ip

> crate::models::LinkPublicIpResponse link_public_ip(link_public_ip_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_public_ip_request** | Option<[**LinkPublicIpRequest**](LinkPublicIpRequest.md)> |  |  |

### Return type

[**crate::models::LinkPublicIpResponse**](LinkPublicIpResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_public_ip_ranges

> crate::models::ReadPublicIpRangesResponse read_public_ip_ranges(read_public_ip_ranges_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_public_ip_ranges_request** | Option<[**ReadPublicIpRangesRequest**](ReadPublicIpRangesRequest.md)> |  |  |

### Return type

[**crate::models::ReadPublicIpRangesResponse**](ReadPublicIpRangesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_public_ips

> crate::models::ReadPublicIpsResponse read_public_ips(read_public_ips_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_public_ips_request** | Option<[**ReadPublicIpsRequest**](ReadPublicIpsRequest.md)> |  |  |

### Return type

[**crate::models::ReadPublicIpsResponse**](ReadPublicIpsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_public_ip

> crate::models::UnlinkPublicIpResponse unlink_public_ip(unlink_public_ip_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unlink_public_ip_request** | Option<[**UnlinkPublicIpRequest**](UnlinkPublicIpRequest.md)> |  |  |

### Return type

[**crate::models::UnlinkPublicIpResponse**](UnlinkPublicIpResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

