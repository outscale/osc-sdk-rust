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

