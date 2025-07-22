# \ServerCertificateApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_server_certificate**](ServerCertificateApi.md#create_server_certificate) | **POST** /CreateServerCertificate | 
[**delete_server_certificate**](ServerCertificateApi.md#delete_server_certificate) | **POST** /DeleteServerCertificate | 
[**read_server_certificates**](ServerCertificateApi.md#read_server_certificates) | **POST** /ReadServerCertificates | 
[**update_server_certificate**](ServerCertificateApi.md#update_server_certificate) | **POST** /UpdateServerCertificate | 



## create_server_certificate

> crate::models::CreateServerCertificateResponse create_server_certificate(create_server_certificate_request)


Creates a server certificate and its matching private key.<br /><br /> These elements can be used with other services (for example, to configure SSL termination on load balancers).<br /><br /> You can also specify the chain of intermediate certification authorities if your certificate is not directly signed by a root one. You can specify multiple intermediate certification authorities in the `CertificateChain` parameter. To do so, concatenate all certificates in the correct order (the first certificate must be the authority of your certificate, the second must be the authority of the first one, and so on).<br /><br /> The private key must be a RSA key in PKCS1 form. To check this, open the PEM file and ensure its header reads as follows: BEGIN RSA PRIVATE KEY.<br /><br /> [IMPORTANT]<br /><br /> This private key must not be protected by a password or a passphrase.<br /><br /> For more information, see [About Server Certificates in EIM](https://docs.outscale.com/en/userguide/About-Server-Certificates-in-EIM.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_server_certificate_request** | Option<[**CreateServerCertificateRequest**](CreateServerCertificateRequest.md)> |  |  |

### Return type

[**crate::models::CreateServerCertificateResponse**](CreateServerCertificateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_server_certificate

> crate::models::DeleteServerCertificateResponse delete_server_certificate(delete_server_certificate_request)


Deletes a specified server certificate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_server_certificate_request** | Option<[**DeleteServerCertificateRequest**](DeleteServerCertificateRequest.md)> |  |  |

### Return type

[**crate::models::DeleteServerCertificateResponse**](DeleteServerCertificateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_server_certificates

> crate::models::ReadServerCertificatesResponse read_server_certificates(read_server_certificates_request)


Lists your server certificates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_server_certificates_request** | Option<[**ReadServerCertificatesRequest**](ReadServerCertificatesRequest.md)> |  |  |

### Return type

[**crate::models::ReadServerCertificatesResponse**](ReadServerCertificatesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_server_certificate

> crate::models::UpdateServerCertificateResponse update_server_certificate(update_server_certificate_request)


Modifies the name and/or the path of a specified server certificate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_server_certificate_request** | Option<[**UpdateServerCertificateRequest**](UpdateServerCertificateRequest.md)> |  |  |

### Return type

[**crate::models::UpdateServerCertificateResponse**](UpdateServerCertificateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

