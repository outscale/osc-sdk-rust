# \NatServiceApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_nat_service**](NatServiceApi.md#create_nat_service) | **POST** /CreateNatService | 
[**delete_nat_service**](NatServiceApi.md#delete_nat_service) | **POST** /DeleteNatService | 
[**read_nat_services**](NatServiceApi.md#read_nat_services) | **POST** /ReadNatServices | 



## create_nat_service

> crate::models::CreateNatServiceResponse create_nat_service(create_nat_service_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_nat_service_request** | Option<[**CreateNatServiceRequest**](CreateNatServiceRequest.md)> |  |  |

### Return type

[**crate::models::CreateNatServiceResponse**](CreateNatServiceResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_nat_service

> crate::models::DeleteNatServiceResponse delete_nat_service(delete_nat_service_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_nat_service_request** | Option<[**DeleteNatServiceRequest**](DeleteNatServiceRequest.md)> |  |  |

### Return type

[**crate::models::DeleteNatServiceResponse**](DeleteNatServiceResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_nat_services

> crate::models::ReadNatServicesResponse read_nat_services(read_nat_services_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_nat_services_request** | Option<[**ReadNatServicesRequest**](ReadNatServicesRequest.md)> |  |  |

### Return type

[**crate::models::ReadNatServicesResponse**](ReadNatServicesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

