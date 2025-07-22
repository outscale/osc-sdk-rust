# \PublicCatalogApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_public_catalog**](PublicCatalogApi.md#read_public_catalog) | **POST** /ReadPublicCatalog | 



## read_public_catalog

> crate::models::ReadPublicCatalogResponse read_public_catalog(read_public_catalog_request)


Returns the price list of OUTSCALE products and services for the Region specified in the endpoint of the request. For more information, see [About Regions and Subregions](https://docs.outscale.com/en/userguide/About-Regions-and-Subregions.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_public_catalog_request** | Option<[**ReadPublicCatalogRequest**](ReadPublicCatalogRequest.md)> |  |  |

### Return type

[**crate::models::ReadPublicCatalogResponse**](ReadPublicCatalogResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

