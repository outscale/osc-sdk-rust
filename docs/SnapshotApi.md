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


Creates a snapshot. Snapshots are point-in-time images of a volume that you can use to back up your data or to create replicas of this volume.<br /> You can use this method in three different ways: * **Creating from a volume**: You create a snapshot from one of your volumes.<br /> * **Copying a snapshot**: You copy an existing snapshot. The source snapshot can be one of your own snapshots, or a snapshot owned by another account that has granted you permission via the [UpdateSnapshot](#updatesnapshot) method.<br /> * **Importing from a bucket**: You import a snapshot located in an OUTSCALE Object Storage (OOS) bucket. First, the owner of the source snapshot must export it to a bucket by using the [CreateSnapshotExportTask](#createsnapshotexporttask) method. Then, they must grant you permission to read the snapshot via a pre-signed URL. For more information, see [Creating a Pre-Signed URL](https://docs.outscale.com/en/userguide/Creating-a-Pre-Signed-URL.html).  **[NOTE]**<br /> In case of excessive use of the snapshot creation feature on the same volume over a short period of time, 3DS OUTSCALE reserves the right to temporarily block the feature.  For more information, see [About Snapshots](https://docs.outscale.com/en/userguide/About-Snapshots.html).

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


Exports a snapshot to an OUTSCALE Object Storage (OOS) bucket that belongs to you. This action enables you to create a backup of your snapshot.<br /><br /> You can share this snapshot with others accounts by granting permission to read it via pre-signed URLs. For more information, see [Creating a Pre-Signed URL](https://docs.outscale.com/en/userguide/Creating-a-Pre-Signed-URL.html).<br /><br /> For more information, see [About Snapshots](https://docs.outscale.com/en/userguide/About-Snapshots.html).

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


Deletes a specified snapshot.<br /> You cannot delete a snapshot that is currently used by an OUTSCALE machine image (OMI). To do so, you first need to delete the corresponding OMI. For more information, see the [DeleteImage](#deleteimage) method.

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


Lists one or more snapshot export tasks.

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


Lists one or more snapshots that are available to you and the permissions to create volumes from them.

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


Modifies the permissions for a specified snapshot.<br /> You must specify either the `Additions` or the `Removals` parameter.<br /> After sharing a snapshot with an account, the other account can create a copy of it that they own. For more information about copying snapshots, see [CreateSnapshot](#createsnapshot).

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

