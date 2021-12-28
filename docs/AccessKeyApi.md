# \AccessKeyApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_access_key**](AccessKeyApi.md#create_access_key) | **POST** /CreateAccessKey | 
[**delete_access_key**](AccessKeyApi.md#delete_access_key) | **POST** /DeleteAccessKey | 
[**read_access_keys**](AccessKeyApi.md#read_access_keys) | **POST** /ReadAccessKeys | 
[**read_secret_access_key**](AccessKeyApi.md#read_secret_access_key) | **POST** /ReadSecretAccessKey | 
[**update_access_key**](AccessKeyApi.md#update_access_key) | **POST** /UpdateAccessKey | 



## create_access_key

> crate::models::CreateAccessKeyResponse create_access_key(create_access_key_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_access_key_request** | Option<[**CreateAccessKeyRequest**](CreateAccessKeyRequest.md)> |  |  |

### Return type

[**crate::models::CreateAccessKeyResponse**](CreateAccessKeyResponse.md)

### Authorization

[ApiKeyAuthSec](../README.md#ApiKeyAuthSec), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_access_key

> crate::models::DeleteAccessKeyResponse delete_access_key(delete_access_key_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_access_key_request** | Option<[**DeleteAccessKeyRequest**](DeleteAccessKeyRequest.md)> |  |  |

### Return type

[**crate::models::DeleteAccessKeyResponse**](DeleteAccessKeyResponse.md)

### Authorization

[ApiKeyAuthSec](../README.md#ApiKeyAuthSec), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_access_keys

> crate::models::ReadAccessKeysResponse read_access_keys(read_access_keys_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_access_keys_request** | Option<[**ReadAccessKeysRequest**](ReadAccessKeysRequest.md)> |  |  |

### Return type

[**crate::models::ReadAccessKeysResponse**](ReadAccessKeysResponse.md)

### Authorization

[ApiKeyAuthSec](../README.md#ApiKeyAuthSec), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_secret_access_key

> crate::models::ReadSecretAccessKeyResponse read_secret_access_key(read_secret_access_key_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_secret_access_key_request** | Option<[**ReadSecretAccessKeyRequest**](ReadSecretAccessKeyRequest.md)> |  |  |

### Return type

[**crate::models::ReadSecretAccessKeyResponse**](ReadSecretAccessKeyResponse.md)

### Authorization

[ApiKeyAuthSec](../README.md#ApiKeyAuthSec), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_access_key

> crate::models::UpdateAccessKeyResponse update_access_key(update_access_key_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_access_key_request** | Option<[**UpdateAccessKeyRequest**](UpdateAccessKeyRequest.md)> |  |  |

### Return type

[**crate::models::UpdateAccessKeyResponse**](UpdateAccessKeyResponse.md)

### Authorization

[ApiKeyAuthSec](../README.md#ApiKeyAuthSec), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

