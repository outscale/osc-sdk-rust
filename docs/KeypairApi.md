# \KeypairApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_keypair**](KeypairApi.md#create_keypair) | **POST** /CreateKeypair | 
[**delete_keypair**](KeypairApi.md#delete_keypair) | **POST** /DeleteKeypair | 
[**read_keypairs**](KeypairApi.md#read_keypairs) | **POST** /ReadKeypairs | 



## create_keypair

> crate::models::CreateKeypairResponse create_keypair(create_keypair_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_keypair_request** | Option<[**CreateKeypairRequest**](CreateKeypairRequest.md)> |  |  |

### Return type

[**crate::models::CreateKeypairResponse**](CreateKeypairResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_keypair

> crate::models::DeleteKeypairResponse delete_keypair(delete_keypair_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_keypair_request** | Option<[**DeleteKeypairRequest**](DeleteKeypairRequest.md)> |  |  |

### Return type

[**crate::models::DeleteKeypairResponse**](DeleteKeypairResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_keypairs

> crate::models::ReadKeypairsResponse read_keypairs(read_keypairs_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_keypairs_request** | Option<[**ReadKeypairsRequest**](ReadKeypairsRequest.md)> |  |  |

### Return type

[**crate::models::ReadKeypairsResponse**](ReadKeypairsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

