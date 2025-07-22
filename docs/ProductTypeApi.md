# \ProductTypeApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_product_type**](ProductTypeApi.md#create_product_type) | **POST** /CreateProductType | 
[**delete_product_type**](ProductTypeApi.md#delete_product_type) | **POST** /DeleteProductType | 
[**read_product_types**](ProductTypeApi.md#read_product_types) | **POST** /ReadProductTypes | 



## create_product_type

> crate::models::CreateProductTypeResponse create_product_type(create_product_type_request)


Creates a product type you can associate with an OMI for consumption monitoring and billing purposes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_product_type_request** | Option<[**CreateProductTypeRequest**](CreateProductTypeRequest.md)> |  |  |

### Return type

[**crate::models::CreateProductTypeResponse**](CreateProductTypeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_product_type

> crate::models::DeleteProductTypeResponse delete_product_type(delete_product_type_request)


Deletes a specified product type that belongs to you.<br />  **[WARNING]**<br /> The product type must not be associated with one or more OMIs to be deleted. Otherwise, you need to force the deletion.<br /> If you force the deletion, the product type is deleted and remains associated with the OMIs.<br />

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_product_type_request** | Option<[**DeleteProductTypeRequest**](DeleteProductTypeRequest.md)> |  |  |

### Return type

[**crate::models::DeleteProductTypeResponse**](DeleteProductTypeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_product_types

> crate::models::ReadProductTypesResponse read_product_types(read_product_types_request)


Lists one or more product types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_product_types_request** | Option<[**ReadProductTypesRequest**](ReadProductTypesRequest.md)> |  |  |

### Return type

[**crate::models::ReadProductTypesResponse**](ReadProductTypesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

