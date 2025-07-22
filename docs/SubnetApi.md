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


Creates a Subnet in an existing Net.<br /> To create a Subnet in a Net, you have to provide the ID of the Net and the IP range for the Subnet (its network range). Once the Subnet is created, you cannot modify its IP range.<br /><br /> For more information, see [About Nets](https://docs.outscale.com/en/userguide/About-Nets.html).

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


Deletes a specified Subnet.<br /> Before deleting the Subnet, you need to delete all resources associated with the Subnet:<br /><br />  * Virtual machines (VMs)<br /> * Network Interface Cards (NICs)<br /> * NAT services<br /> * Load balancers

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


Lists one or more of your Subnets.<br /> If you do not specify any Subnet ID, this action describes all of your Subnets.

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


Modifies the specified attribute of a Subnet.

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

