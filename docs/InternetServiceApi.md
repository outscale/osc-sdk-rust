# \InternetServiceApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_internet_service**](InternetServiceApi.md#create_internet_service) | **POST** /CreateInternetService | 
[**delete_internet_service**](InternetServiceApi.md#delete_internet_service) | **POST** /DeleteInternetService | 
[**link_internet_service**](InternetServiceApi.md#link_internet_service) | **POST** /LinkInternetService | 
[**read_internet_services**](InternetServiceApi.md#read_internet_services) | **POST** /ReadInternetServices | 
[**unlink_internet_service**](InternetServiceApi.md#unlink_internet_service) | **POST** /UnlinkInternetService | 



## create_internet_service

> crate::models::CreateInternetServiceResponse create_internet_service(create_internet_service_request)


Creates an internet service you can use with a Net.<br /> An internet service enables virtual machines (VMs) launched in a Net to connect to the Internet. It allows routing of incoming and outgoing Internet traffic and management of public IP addresses.<br /><br /> For more information, see [About Internet Services](https://docs.outscale.com/en/userguide/About-Internet-Services.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_internet_service_request** | Option<[**CreateInternetServiceRequest**](CreateInternetServiceRequest.md)> |  |  |

### Return type

[**crate::models::CreateInternetServiceResponse**](CreateInternetServiceResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_internet_service

> crate::models::DeleteInternetServiceResponse delete_internet_service(delete_internet_service_request)


Deletes an internet service.<br /> Before deleting an internet service, you must detach it from any Net it is attached to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_internet_service_request** | Option<[**DeleteInternetServiceRequest**](DeleteInternetServiceRequest.md)> |  |  |

### Return type

[**crate::models::DeleteInternetServiceResponse**](DeleteInternetServiceResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_internet_service

> crate::models::LinkInternetServiceResponse link_internet_service(link_internet_service_request)


Attaches an internet service to a Net.<br /> To enable the connection between the Internet and a Net, you must attach an internet service to this Net.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_internet_service_request** | Option<[**LinkInternetServiceRequest**](LinkInternetServiceRequest.md)> |  |  |

### Return type

[**crate::models::LinkInternetServiceResponse**](LinkInternetServiceResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_internet_services

> crate::models::ReadInternetServicesResponse read_internet_services(read_internet_services_request)


Lists one or more of your internet services.<br /> An internet service enables virtual machines (VMs) launched in a Net to connect to the Internet. It allows routing of incoming and outgoing Internet traffic and management of public IP addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_internet_services_request** | Option<[**ReadInternetServicesRequest**](ReadInternetServicesRequest.md)> |  |  |

### Return type

[**crate::models::ReadInternetServicesResponse**](ReadInternetServicesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_internet_service

> crate::models::UnlinkInternetServiceResponse unlink_internet_service(unlink_internet_service_request)


Detaches an internet service from a Net.<br /> This action disables and detaches an internet service from a Net. The Net must not contain virtual machines (VMs) using public IPs nor internet-facing load balancers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unlink_internet_service_request** | Option<[**UnlinkInternetServiceRequest**](UnlinkInternetServiceRequest.md)> |  |  |

### Return type

[**crate::models::UnlinkInternetServiceResponse**](UnlinkInternetServiceResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

