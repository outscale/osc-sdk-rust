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


Allocates a flexible GPU (fGPU) to your account.<br /> You can then attach this fGPU to a virtual machine (VM).<br /><br /> For more information, see [About Flexible GPUs](https://docs.outscale.com/en/userguide/About-Flexible-GPUs.html).

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


Releases a flexible GPU (fGPU) from your account.<br /> The fGPU becomes free to be used by someone else.

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


Attaches one of your allocated flexible GPUs (fGPUs) to one of your virtual machines (VMs).<br /> To complete the linking of the fGPU, you need to do a stop/start of the VM. A simple restart is not sufficient, as the linking of the fGPU is done when the VM goes through the `stopped` state. For the difference between stop/start and restart, see [About VM Lifecycle](https://docs.outscale.com/en/userguide/About-VM-Lifecycle.html).<br /><br />  **[NOTE]**<br /> You can attach fGPUs only to VMs with the `highest` (1) performance flag. For more information see [About Flexible GPUs](https://docs.outscale.com/en/userguide/About-Flexible-GPUs.html) and [VM Types](https://docs.outscale.com/en/userguide/VM-Types.html).

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


Lists all flexible GPUs available in the public catalog.

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


Lists one or more flexible GPUs (fGPUs) allocated to your account.

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


Detaches a flexible GPU (fGPU) from a virtual machine (VM).<br /> The fGPU is in the `detaching` state until the VM is stopped, after which it becomes `allocated`. It is then available again for attachment to a VM.

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


Modifies a flexible GPU (fGPU) behavior.

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

