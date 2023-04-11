# \CatalogApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_catalog**](CatalogApi.md#read_catalog) | **POST** /ReadCatalog | 
[**read_catalogs**](CatalogApi.md#read_catalogs) | **POST** /ReadCatalogs | 



## read_catalog

> crate::models::ReadCatalogResponse read_catalog(read_catalog_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_catalog_request** | Option<[**ReadCatalogRequest**](ReadCatalogRequest.md)> |  |  |

### Return type

[**crate::models::ReadCatalogResponse**](ReadCatalogResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_catalogs

> crate::models::ReadCatalogsResponse read_catalogs(read_catalogs_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_catalogs_request** | Option<[**ReadCatalogsRequest**](ReadCatalogsRequest.md)> |  |  |

### Return type

[**crate::models::ReadCatalogsResponse**](ReadCatalogsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

