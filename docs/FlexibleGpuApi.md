# \FlexibleGpuApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_flexible_gpu**](FlexibleGpuApi.md#create_flexible_gpu) | **POST** /CreateFlexibleGpu | 
[**delete_flexible_gpu**](FlexibleGpuApi.md#delete_flexible_gpu) | **POST** /DeleteFlexibleGpu | 
[**link_flexible_gpu**](FlexibleGpuApi.md#link_flexible_gpu) | **POST** /LinkFlexibleGpu | 
[**read_flexible_gpu_catalog**](FlexibleGpuApi.md#read_flexible_gpu_catalog) | **POST** /ReadFlexibleGpuCatalog | 
[**read_flexible_gpus**](FlexibleGpuApi.md#read_flexible_gpus) | **POST** /ReadFlexibleGpus | 
[**unlink_flexible_gpu**](FlexibleGpuApi.md#unlink_flexible_gpu) | **POST** /UnlinkFlexibleGpu | 
[**update_flexible_gpu**](FlexibleGpuApi.md#update_flexible_gpu) | **POST** /UpdateFlexibleGpu | 



## create_flexible_gpu

> crate::models::CreateFlexibleGpuResponse create_flexible_gpu(create_flexible_gpu_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_flexible_gpu_request** | Option<[**CreateFlexibleGpuRequest**](CreateFlexibleGpuRequest.md)> |  |  |

### Return type

[**crate::models::CreateFlexibleGpuResponse**](CreateFlexibleGpuResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_flexible_gpu

> crate::models::DeleteFlexibleGpuResponse delete_flexible_gpu(delete_flexible_gpu_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_flexible_gpu_request** | Option<[**DeleteFlexibleGpuRequest**](DeleteFlexibleGpuRequest.md)> |  |  |

### Return type

[**crate::models::DeleteFlexibleGpuResponse**](DeleteFlexibleGpuResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_flexible_gpu

> crate::models::LinkFlexibleGpuResponse link_flexible_gpu(link_flexible_gpu_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_flexible_gpu_request** | Option<[**LinkFlexibleGpuRequest**](LinkFlexibleGpuRequest.md)> |  |  |

### Return type

[**crate::models::LinkFlexibleGpuResponse**](LinkFlexibleGpuResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_flexible_gpu_catalog

> crate::models::ReadFlexibleGpuCatalogResponse read_flexible_gpu_catalog(read_flexible_gpu_catalog_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_flexible_gpu_catalog_request** | Option<[**ReadFlexibleGpuCatalogRequest**](ReadFlexibleGpuCatalogRequest.md)> |  |  |

### Return type

[**crate::models::ReadFlexibleGpuCatalogResponse**](ReadFlexibleGpuCatalogResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_flexible_gpus

> crate::models::ReadFlexibleGpusResponse read_flexible_gpus(read_flexible_gpus_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_flexible_gpus_request** | Option<[**ReadFlexibleGpusRequest**](ReadFlexibleGpusRequest.md)> |  |  |

### Return type

[**crate::models::ReadFlexibleGpusResponse**](ReadFlexibleGpusResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_flexible_gpu

> crate::models::UnlinkFlexibleGpuResponse unlink_flexible_gpu(unlink_flexible_gpu_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unlink_flexible_gpu_request** | Option<[**UnlinkFlexibleGpuRequest**](UnlinkFlexibleGpuRequest.md)> |  |  |

### Return type

[**crate::models::UnlinkFlexibleGpuResponse**](UnlinkFlexibleGpuResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_flexible_gpu

> crate::models::UpdateFlexibleGpuResponse update_flexible_gpu(update_flexible_gpu_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_flexible_gpu_request** | Option<[**UpdateFlexibleGpuRequest**](UpdateFlexibleGpuRequest.md)> |  |  |

### Return type

[**crate::models::UpdateFlexibleGpuResponse**](UpdateFlexibleGpuResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

