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


Creates an OUTSCALE machine image (OMI).<br /> You can use this method for different use cases: * **Creating from a VM**: You create an OMI from one of your virtual machines (VMs).<br> * **Copying an OMI**: You copy an existing OMI. The source OMI can be one of your own OMIs, or an OMI owned by another account that has granted you permission via the [UpdateImage](#updateimage) method.<br> * **Registering from a snapshot**: You register an OMI from an existing snapshot. The source snapshot can be one of your own snapshots, or a snapshot owned by another account that has granted you permission via the [UpdateSnapshot](#updatesnapshot) method.<br> * **Registering from a bucket by using a manifest file**: You register an OMI from the manifest file of an OMI that was exported to an OUTSCALE Object Storage (OOS) bucket. First, the owner of the source OMI must export it to the bucket by using the [CreateImageExportTask](#createimageexporttask) method. Then, they must grant you permission to read the manifest file via a pre-signed URL. For more information, see [Creating a Pre-Signed URL](https://docs.outscale.com/en/userguide/Creating-a-Pre-Signed-URL.html).  **[TIP]**<br /> Registering from a bucket enables you to copy an OMI across Regions.  For more information, see [About OMIs](https://docs.outscale.com/en/userguide/About-OMIs.html).

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


Exports an OUTSCALE machine image (OMI) to an OUTSCALE Object Storage (OOS) bucket.<br /> This enables you to copy an OMI between accounts in different Regions.<br /><br /> This action creates the necessary snapshots and manifest file in the bucket. The OMI can then be imported to another account using a pre-signed URL of its manifest file. For more information, see [Creating a Pre-Signed URL](https://docs.outscale.com/en/userguide/Creating-a-Pre-Signed-URL.html).<br /><br /> To copy an OMI in the same Region, you can also use the [CreateImage](#createimage) method.<br />  **[IMPORTANT]**<br /> You cannot export a shared or public OMI, as they do not belong to you. To do so, you must first copy it to your account. The copy then belongs to you and you can export it.<br /><br /> For more information, see [About OMIs](https://docs.outscale.com/en/userguide/About-OMIs.html).

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


Deletes an OUTSCALE machine image (OMI) so that you cannot use it anymore to launch virtual machines (VMs). However, you can still use VMs already launched from this OMI.

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


Lists one or more image export tasks.

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


Lists one or more OUTSCALE machine images (OMIs) you can use.

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


Modifies the access permissions for an OUTSCALE machine image (OMI).<br /> You must specify either the `Additions` or the `Removals` parameter.<br /> After sharing an OMI with an account, the other account can create a copy of it that they own. For more information about copying OMIs, see [CreateImage](#createimage).

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

