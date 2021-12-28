# \VmApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_vms**](VmApi.md#create_vms) | **POST** /CreateVms | 
[**delete_vms**](VmApi.md#delete_vms) | **POST** /DeleteVms | 
[**read_admin_password**](VmApi.md#read_admin_password) | **POST** /ReadAdminPassword | 
[**read_console_output**](VmApi.md#read_console_output) | **POST** /ReadConsoleOutput | 
[**read_vm_types**](VmApi.md#read_vm_types) | **POST** /ReadVmTypes | 
[**read_vms**](VmApi.md#read_vms) | **POST** /ReadVms | 
[**read_vms_state**](VmApi.md#read_vms_state) | **POST** /ReadVmsState | 
[**reboot_vms**](VmApi.md#reboot_vms) | **POST** /RebootVms | 
[**start_vms**](VmApi.md#start_vms) | **POST** /StartVms | 
[**stop_vms**](VmApi.md#stop_vms) | **POST** /StopVms | 
[**update_vm**](VmApi.md#update_vm) | **POST** /UpdateVm | 



## create_vms

> crate::models::CreateVmsResponse create_vms(create_vms_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_vms_request** | Option<[**CreateVmsRequest**](CreateVmsRequest.md)> |  |  |

### Return type

[**crate::models::CreateVmsResponse**](CreateVmsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_vms

> crate::models::DeleteVmsResponse delete_vms(delete_vms_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_vms_request** | Option<[**DeleteVmsRequest**](DeleteVmsRequest.md)> |  |  |

### Return type

[**crate::models::DeleteVmsResponse**](DeleteVmsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_admin_password

> crate::models::ReadAdminPasswordResponse read_admin_password(read_admin_password_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_admin_password_request** | Option<[**ReadAdminPasswordRequest**](ReadAdminPasswordRequest.md)> |  |  |

### Return type

[**crate::models::ReadAdminPasswordResponse**](ReadAdminPasswordResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_console_output

> crate::models::ReadConsoleOutputResponse read_console_output(read_console_output_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_console_output_request** | Option<[**ReadConsoleOutputRequest**](ReadConsoleOutputRequest.md)> |  |  |

### Return type

[**crate::models::ReadConsoleOutputResponse**](ReadConsoleOutputResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_vm_types

> crate::models::ReadVmTypesResponse read_vm_types(read_vm_types_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_vm_types_request** | Option<[**ReadVmTypesRequest**](ReadVmTypesRequest.md)> |  |  |

### Return type

[**crate::models::ReadVmTypesResponse**](ReadVmTypesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_vms

> crate::models::ReadVmsResponse read_vms(read_vms_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_vms_request** | Option<[**ReadVmsRequest**](ReadVmsRequest.md)> |  |  |

### Return type

[**crate::models::ReadVmsResponse**](ReadVmsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_vms_state

> crate::models::ReadVmsStateResponse read_vms_state(read_vms_state_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_vms_state_request** | Option<[**ReadVmsStateRequest**](ReadVmsStateRequest.md)> |  |  |

### Return type

[**crate::models::ReadVmsStateResponse**](ReadVmsStateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reboot_vms

> crate::models::RebootVmsResponse reboot_vms(reboot_vms_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reboot_vms_request** | Option<[**RebootVmsRequest**](RebootVmsRequest.md)> |  |  |

### Return type

[**crate::models::RebootVmsResponse**](RebootVmsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_vms

> crate::models::StartVmsResponse start_vms(start_vms_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_vms_request** | Option<[**StartVmsRequest**](StartVmsRequest.md)> |  |  |

### Return type

[**crate::models::StartVmsResponse**](StartVmsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_vms

> crate::models::StopVmsResponse stop_vms(stop_vms_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stop_vms_request** | Option<[**StopVmsRequest**](StopVmsRequest.md)> |  |  |

### Return type

[**crate::models::StopVmsResponse**](StopVmsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_vm

> crate::models::UpdateVmResponse update_vm(update_vm_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_vm_request** | Option<[**UpdateVmRequest**](UpdateVmRequest.md)> |  |  |

### Return type

[**crate::models::UpdateVmResponse**](UpdateVmResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

