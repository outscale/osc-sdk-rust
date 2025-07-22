# \NatServiceApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_nat_service**](NatServiceApi.md#create_nat_service) | **POST** /CreateNatService | 
[**delete_nat_service**](NatServiceApi.md#delete_nat_service) | **POST** /DeleteNatService | 
[**read_nat_services**](NatServiceApi.md#read_nat_services) | **POST** /ReadNatServices | 



## create_nat_service

> crate::models::CreateNatServiceResponse create_nat_service(create_nat_service_request)


Creates a network address translation (NAT) service in the specified public Subnet of a Net.<br /> A NAT service enables virtual machines (VMs) placed in the private Subnet of this Net to connect to the Internet, without being accessible from the Internet.<br /> When creating a NAT service, you specify the allocation ID of the public IP you want to use as public IP for the NAT service. Once the NAT service is created, you need to create a route in the route table of the private Subnet, with 0.0.0.0/0 as destination and the ID of the NAT service as target. For more information, see [LinkPublicIP](#linkpublicip) and [CreateRoute](#createroute).<br /> This action also enables you to create multiple NAT services in the same Net (one per public Subnet).<br /><br />  **[IMPORTANT]**<br /> You cannot modify the public IP associated with a NAT service after its creation. To do so, you need to delete the NAT service and create a new one with another public IP.<br /><br /> For more information, see [About NAT Services](https://docs.outscale.com/en/userguide/About-NAT-Services.html).

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


Deletes a specified network address translation (NAT) service.<br /> This action disassociates the public IP from the NAT service, but does not release this public IP from your account. However, it does not delete any NAT service routes in your route tables.

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


Lists one or more network address translation (NAT) services.

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

