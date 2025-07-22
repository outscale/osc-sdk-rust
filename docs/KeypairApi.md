# \KeypairApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_keypair**](KeypairApi.md#create_keypair) | **POST** /CreateKeypair | 
[**delete_keypair**](KeypairApi.md#delete_keypair) | **POST** /DeleteKeypair | 
[**read_keypairs**](KeypairApi.md#read_keypairs) | **POST** /ReadKeypairs | 



## create_keypair

> crate::models::CreateKeypairResponse create_keypair(create_keypair_request)


Creates a keypair to use with your virtual machines (VMs).<br /> You can use this method in two different ways: * **Creating a keypair**: In that case, 3DS OUTSCALE creates a 2048-bit RSA keypair, stores its public key in your account, and returns its private key in the response of the call so that you can save it in a file.<br /> When you save the returned private key, make sure you replace the `\\n` escape sequences with real line breaks. * **Importing a keypair created locally**: If you already have a keypair that you have created locally with a third-party tool, you can import its public key in your account. The following types of key can be imported: RSA (2048 bits or preferably 4096 bits), Ed25519, and ECDSA (256 bits, 384 bits, or 521 bits). The following formats can be used: PEM, PKCS8, RFC4716, and OpenSSH.  For more information, see [About Keypairs](https://docs.outscale.com/en/userguide/About-Keypairs.html).

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


Deletes the specified keypair.<br /> This action deletes the public key stored by 3DS OUTSCALE, thus deleting the keypair.

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


Lists one or more of your keypairs.

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

