# \AccessKeyApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_access_key**](AccessKeyApi.md#create_access_key) | **POST** /CreateAccessKey | 
[**delete_access_key**](AccessKeyApi.md#delete_access_key) | **POST** /DeleteAccessKey | 
[**read_access_keys**](AccessKeyApi.md#read_access_keys) | **POST** /ReadAccessKeys | 
[**update_access_key**](AccessKeyApi.md#update_access_key) | **POST** /UpdateAccessKey | 



## create_access_key

> crate::models::CreateAccessKeyResponse create_access_key(create_access_key_request)


Creates an access key for either your root account or an EIM user. The new key is automatically set to `ACTIVE`.<br /><br /> For more information, see [About Access Keys](https://docs.outscale.com/en/userguide/About-Access-Keys.html).

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


Deletes the specified access key of either your root account or an EIM user.<br /><br /> The access key of an EIM user must be in the `INACTIVE` state to be deleted.

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


Lists the access key IDs of either your root account or an EIM user.

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


## update_access_key

> crate::models::UpdateAccessKeyResponse update_access_key(update_access_key_request)


Modifies the attributes of the specified access key of either your root account or an EIM user.<br /><br /> The parameter `ExpirationDate` is not required when updating the state of your access key. However, if you do not specify the expiration date of an access key when updating its state, it is then set to not expire.

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

