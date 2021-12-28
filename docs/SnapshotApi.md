# \SnapshotApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_snapshot**](SnapshotApi.md#create_snapshot) | **POST** /CreateSnapshot | 
[**create_snapshot_export_task**](SnapshotApi.md#create_snapshot_export_task) | **POST** /CreateSnapshotExportTask | 
[**delete_snapshot**](SnapshotApi.md#delete_snapshot) | **POST** /DeleteSnapshot | 
[**read_snapshot_export_tasks**](SnapshotApi.md#read_snapshot_export_tasks) | **POST** /ReadSnapshotExportTasks | 
[**read_snapshots**](SnapshotApi.md#read_snapshots) | **POST** /ReadSnapshots | 
[**update_snapshot**](SnapshotApi.md#update_snapshot) | **POST** /UpdateSnapshot | 



## create_snapshot

> crate::models::CreateSnapshotResponse create_snapshot(create_snapshot_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_snapshot_request** | Option<[**CreateSnapshotRequest**](CreateSnapshotRequest.md)> |  |  |

### Return type

[**crate::models::CreateSnapshotResponse**](CreateSnapshotResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_snapshot_export_task

> crate::models::CreateSnapshotExportTaskResponse create_snapshot_export_task(create_snapshot_export_task_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_snapshot_export_task_request** | Option<[**CreateSnapshotExportTaskRequest**](CreateSnapshotExportTaskRequest.md)> |  |  |

### Return type

[**crate::models::CreateSnapshotExportTaskResponse**](CreateSnapshotExportTaskResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_snapshot

> crate::models::DeleteSnapshotResponse delete_snapshot(delete_snapshot_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_snapshot_request** | Option<[**DeleteSnapshotRequest**](DeleteSnapshotRequest.md)> |  |  |

### Return type

[**crate::models::DeleteSnapshotResponse**](DeleteSnapshotResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_snapshot_export_tasks

> crate::models::ReadSnapshotExportTasksResponse read_snapshot_export_tasks(read_snapshot_export_tasks_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_snapshot_export_tasks_request** | Option<[**ReadSnapshotExportTasksRequest**](ReadSnapshotExportTasksRequest.md)> |  |  |

### Return type

[**crate::models::ReadSnapshotExportTasksResponse**](ReadSnapshotExportTasksResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_snapshots

> crate::models::ReadSnapshotsResponse read_snapshots(read_snapshots_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_snapshots_request** | Option<[**ReadSnapshotsRequest**](ReadSnapshotsRequest.md)> |  |  |

### Return type

[**crate::models::ReadSnapshotsResponse**](ReadSnapshotsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_snapshot

> crate::models::UpdateSnapshotResponse update_snapshot(update_snapshot_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_snapshot_request** | Option<[**UpdateSnapshotRequest**](UpdateSnapshotRequest.md)> |  |  |

### Return type

[**crate::models::UpdateSnapshotResponse**](UpdateSnapshotResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

