# \VolumeApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_volume**](VolumeApi.md#create_volume) | **POST** /CreateVolume | 
[**delete_volume**](VolumeApi.md#delete_volume) | **POST** /DeleteVolume | 
[**link_volume**](VolumeApi.md#link_volume) | **POST** /LinkVolume | 
[**read_volumes**](VolumeApi.md#read_volumes) | **POST** /ReadVolumes | 
[**unlink_volume**](VolumeApi.md#unlink_volume) | **POST** /UnlinkVolume | 
[**update_volume**](VolumeApi.md#update_volume) | **POST** /UpdateVolume | 



## create_volume

> crate::models::CreateVolumeResponse create_volume(create_volume_request)


Creates a Block Storage Unit (BSU) volume in a specified Region.<br /> BSU volumes can be attached to a virtual machine (VM) in the same Subregion. You can create an empty volume or restore a volume from an existing snapshot.<br /> You can create the following volume types: Enterprise (`io1`) for provisioned IOPS SSD volumes, Performance (`gp2`) for general purpose SSD volumes, or Magnetic (`standard`) volumes.<br /><br /> For more information, see [About Volumes](https://docs.outscale.com/en/userguide/About-Volumes.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_volume_request** | Option<[**CreateVolumeRequest**](CreateVolumeRequest.md)> |  |  |

### Return type

[**crate::models::CreateVolumeResponse**](CreateVolumeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_volume

> crate::models::DeleteVolumeResponse delete_volume(delete_volume_request)


Deletes a specified Block Storage Unit (BSU) volume.<br /> You can delete available volumes only, that is, volumes that are not attached to a virtual machine (VM).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_volume_request** | Option<[**DeleteVolumeRequest**](DeleteVolumeRequest.md)> |  |  |

### Return type

[**crate::models::DeleteVolumeResponse**](DeleteVolumeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_volume

> crate::models::LinkVolumeResponse link_volume(link_volume_request)


Attaches a Block Storage Unit (BSU) volume to a virtual machine (VM).<br /> The volume and the VM must be in the same Subregion. The VM can be running or stopped. The volume is attached to the specified VM device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_volume_request** | Option<[**LinkVolumeRequest**](LinkVolumeRequest.md)> |  |  |

### Return type

[**crate::models::LinkVolumeResponse**](LinkVolumeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_volumes

> crate::models::ReadVolumesResponse read_volumes(read_volumes_request)


Lists one or more specified Block Storage Unit (BSU) volumes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_volumes_request** | Option<[**ReadVolumesRequest**](ReadVolumesRequest.md)> |  |  |

### Return type

[**crate::models::ReadVolumesResponse**](ReadVolumesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_volume

> crate::models::UnlinkVolumeResponse unlink_volume(unlink_volume_request)


Detaches a Block Storage Unit (BSU) volume from a virtual machine (VM).<br /> To detach the root device of a VM, this VM must be stopped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unlink_volume_request** | Option<[**UnlinkVolumeRequest**](UnlinkVolumeRequest.md)> |  |  |

### Return type

[**crate::models::UnlinkVolumeResponse**](UnlinkVolumeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_volume

> crate::models::UpdateVolumeResponse update_volume(update_volume_request)


Modifies the specified attributes of a volume.<br /> Cold volumes are volumes that are attached to stopped or stopping VMs, or that are detached. Hot volumes are volumes that are attached to running VMs.<br /><br />  **[NOTE]**<br /> When the modification is not instantaneous, the response displays the previous value. You can use the [ReadVolumes](#readvolumes) method to see the new value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_volume_request** | Option<[**UpdateVolumeRequest**](UpdateVolumeRequest.md)> |  |  |

### Return type

[**crate::models::UpdateVolumeResponse**](UpdateVolumeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

