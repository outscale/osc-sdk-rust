# \CaApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ca**](CaApi.md#create_ca) | **POST** /CreateCa | 
[**delete_ca**](CaApi.md#delete_ca) | **POST** /DeleteCa | 
[**read_cas**](CaApi.md#read_cas) | **POST** /ReadCas | 
[**update_ca**](CaApi.md#update_ca) | **POST** /UpdateCa | 



## create_ca

> crate::models::CreateCaResponse create_ca(create_ca_request)


Creates a Client Certificate Authority (CA).<br /><br /> For more information, see [About API Access Rules](https://docs.outscale.com/en/userguide/About-API-Access-Rules.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_ca_request** | Option<[**CreateCaRequest**](CreateCaRequest.md)> |  |  |

### Return type

[**crate::models::CreateCaResponse**](CreateCaResponse.md)

### Authorization

[ApiKeyAuthSec](../README.md#ApiKeyAuthSec), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ca

> crate::models::DeleteCaResponse delete_ca(delete_ca_request)


Deletes a specified Client Certificate Authority (CA).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_ca_request** | Option<[**DeleteCaRequest**](DeleteCaRequest.md)> |  |  |

### Return type

[**crate::models::DeleteCaResponse**](DeleteCaResponse.md)

### Authorization

[ApiKeyAuthSec](../README.md#ApiKeyAuthSec), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_cas

> crate::models::ReadCasResponse read_cas(read_cas_request)


Gets information about one or more of your Client Certificate Authorities (CAs).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_cas_request** | Option<[**ReadCasRequest**](ReadCasRequest.md)> |  |  |

### Return type

[**crate::models::ReadCasResponse**](ReadCasResponse.md)

### Authorization

[ApiKeyAuthSec](../README.md#ApiKeyAuthSec), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ca

> crate::models::UpdateCaResponse update_ca(update_ca_request)


Modifies the specified attribute of a Client Certificate Authority (CA).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_ca_request** | Option<[**UpdateCaRequest**](UpdateCaRequest.md)> |  |  |

### Return type

[**crate::models::UpdateCaResponse**](UpdateCaResponse.md)

### Authorization

[ApiKeyAuthSec](../README.md#ApiKeyAuthSec), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

