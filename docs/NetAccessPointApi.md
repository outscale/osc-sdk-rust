# \NetAccessPointApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_net_access_point**](NetAccessPointApi.md#create_net_access_point) | **POST** /CreateNetAccessPoint | 
[**delete_net_access_point**](NetAccessPointApi.md#delete_net_access_point) | **POST** /DeleteNetAccessPoint | 
[**read_net_access_point_services**](NetAccessPointApi.md#read_net_access_point_services) | **POST** /ReadNetAccessPointServices | 
[**read_net_access_points**](NetAccessPointApi.md#read_net_access_points) | **POST** /ReadNetAccessPoints | 
[**update_net_access_point**](NetAccessPointApi.md#update_net_access_point) | **POST** /UpdateNetAccessPoint | 



## create_net_access_point

> crate::models::CreateNetAccessPointResponse create_net_access_point(create_net_access_point_request)


Creates a Net access point to access an OUTSCALE service from this Net without using the Internet and public IPs.<br /> You specify the service using its name. For more information about the available services, see [ReadNetAccessPointServices](#readnetaccesspointservices).<br /> <br /> To control the routing of traffic between the Net and the specified service, you can specify one or more route tables. Virtual machines placed in Subnets associated with the specified route table thus use the Net access point to access the service. When you specify a route table, a route is automatically added to it with the destination set to the prefix list ID of the service, and the target set to the ID of the access point.<br /><br /> When a Net access point is created, a public IP is automatically allocated to your account and used for the Net access point. This public IP is not connected to the Internet. It is counted in your quota, but it is not billed.<br /> <br /> For more information, see [About Net Access Points](https://docs.outscale.com/en/userguide/About-Net-Access-Points.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_net_access_point_request** | Option<[**CreateNetAccessPointRequest**](CreateNetAccessPointRequest.md)> |  |  |

### Return type

[**crate::models::CreateNetAccessPointResponse**](CreateNetAccessPointResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_net_access_point

> crate::models::DeleteNetAccessPointResponse delete_net_access_point(delete_net_access_point_request)


Deletes a specified Net access point.<br /> This action also deletes the corresponding routes added to the route tables you specified for the Net access point.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_net_access_point_request** | Option<[**DeleteNetAccessPointRequest**](DeleteNetAccessPointRequest.md)> |  |  |

### Return type

[**crate::models::DeleteNetAccessPointResponse**](DeleteNetAccessPointResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_net_access_point_services

> crate::models::ReadNetAccessPointServicesResponse read_net_access_point_services(read_net_access_point_services_request)


Lists OUTSCALE services available to create Net access points.<br /> For more information, see [CreateNetAccessPoint](#createnetaccesspoint).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_net_access_point_services_request** | Option<[**ReadNetAccessPointServicesRequest**](ReadNetAccessPointServicesRequest.md)> |  |  |

### Return type

[**crate::models::ReadNetAccessPointServicesResponse**](ReadNetAccessPointServicesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_net_access_points

> crate::models::ReadNetAccessPointsResponse read_net_access_points(read_net_access_points_request)


Lists one or more Net access points.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_net_access_points_request** | Option<[**ReadNetAccessPointsRequest**](ReadNetAccessPointsRequest.md)> |  |  |

### Return type

[**crate::models::ReadNetAccessPointsResponse**](ReadNetAccessPointsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_net_access_point

> crate::models::UpdateNetAccessPointResponse update_net_access_point(update_net_access_point_request)


Modifies the attributes of a Net access point.<br /> This action enables you to add or remove route tables associated with the specified Net access point.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_net_access_point_request** | Option<[**UpdateNetAccessPointRequest**](UpdateNetAccessPointRequest.md)> |  |  |

### Return type

[**crate::models::UpdateNetAccessPointResponse**](UpdateNetAccessPointResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

