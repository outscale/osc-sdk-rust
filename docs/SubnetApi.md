# \SubnetApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_subnet**](SubnetApi.md#create_subnet) | **POST** /CreateSubnet | 
[**delete_subnet**](SubnetApi.md#delete_subnet) | **POST** /DeleteSubnet | 
[**read_subnets**](SubnetApi.md#read_subnets) | **POST** /ReadSubnets | 
[**update_subnet**](SubnetApi.md#update_subnet) | **POST** /UpdateSubnet | 



## create_subnet

> crate::models::CreateSubnetResponse create_subnet(create_subnet_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_subnet_request** | Option<[**CreateSubnetRequest**](CreateSubnetRequest.md)> |  |  |

### Return type

[**crate::models::CreateSubnetResponse**](CreateSubnetResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subnet

> crate::models::DeleteSubnetResponse delete_subnet(delete_subnet_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_subnet_request** | Option<[**DeleteSubnetRequest**](DeleteSubnetRequest.md)> |  |  |

### Return type

[**crate::models::DeleteSubnetResponse**](DeleteSubnetResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_subnets

> crate::models::ReadSubnetsResponse read_subnets(read_subnets_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_subnets_request** | Option<[**ReadSubnetsRequest**](ReadSubnetsRequest.md)> |  |  |

### Return type

[**crate::models::ReadSubnetsResponse**](ReadSubnetsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subnet

> crate::models::UpdateSubnetResponse update_subnet(update_subnet_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_subnet_request** | Option<[**UpdateSubnetRequest**](UpdateSubnetRequest.md)> |  |  |

### Return type

[**crate::models::UpdateSubnetResponse**](UpdateSubnetResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

