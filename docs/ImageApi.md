# \ImageApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_image**](ImageApi.md#create_image) | **POST** /CreateImage | 
[**create_image_export_task**](ImageApi.md#create_image_export_task) | **POST** /CreateImageExportTask | 
[**delete_image**](ImageApi.md#delete_image) | **POST** /DeleteImage | 
[**read_image_export_tasks**](ImageApi.md#read_image_export_tasks) | **POST** /ReadImageExportTasks | 
[**read_images**](ImageApi.md#read_images) | **POST** /ReadImages | 
[**update_image**](ImageApi.md#update_image) | **POST** /UpdateImage | 



## create_image

> crate::models::CreateImageResponse create_image(create_image_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_image_request** | Option<[**CreateImageRequest**](CreateImageRequest.md)> |  |  |

### Return type

[**crate::models::CreateImageResponse**](CreateImageResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_image_export_task

> crate::models::CreateImageExportTaskResponse create_image_export_task(create_image_export_task_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_image_export_task_request** | Option<[**CreateImageExportTaskRequest**](CreateImageExportTaskRequest.md)> |  |  |

### Return type

[**crate::models::CreateImageExportTaskResponse**](CreateImageExportTaskResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_image

> crate::models::DeleteImageResponse delete_image(delete_image_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_image_request** | Option<[**DeleteImageRequest**](DeleteImageRequest.md)> |  |  |

### Return type

[**crate::models::DeleteImageResponse**](DeleteImageResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_image_export_tasks

> crate::models::ReadImageExportTasksResponse read_image_export_tasks(read_image_export_tasks_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_image_export_tasks_request** | Option<[**ReadImageExportTasksRequest**](ReadImageExportTasksRequest.md)> |  |  |

### Return type

[**crate::models::ReadImageExportTasksResponse**](ReadImageExportTasksResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_images

> crate::models::ReadImagesResponse read_images(read_images_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_images_request** | Option<[**ReadImagesRequest**](ReadImagesRequest.md)> |  |  |

### Return type

[**crate::models::ReadImagesResponse**](ReadImagesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_image

> crate::models::UpdateImageResponse update_image(update_image_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_image_request** | Option<[**UpdateImageRequest**](UpdateImageRequest.md)> |  |  |

### Return type

[**crate::models::UpdateImageResponse**](UpdateImageResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

