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


Creates virtual machines (VMs), and then launches them.<br /> This action enables you to create a specified number of VMs using an OUTSCALE machine image (OMI) that you are allowed to use, and then to automatically launch them.<br /> The VMs remain in the `pending` state until they are created and ready to be used. Once automatically launched, they are in the `running` state.<br /> To check the state of your VMs, call the [ReadVms](#readvms) method.<br /> If not specified, the security group used by the service is the default one.<br /> The metadata server enables you to get the public key provided when the VM is launched. Official OMIs contain a script to get this public key and put it inside the VM to provide secure access without password.<br /><br /> For more information, see [About VMs](https://docs.outscale.com/en/userguide/About-VMs.html).

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


Terminates one or more virtual machines (VMs).<br /> This operation is idempotent, that means that all calls succeed if you terminate a VM more than once.

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


Gets the administrator password for a Windows running virtual machine (VM).<br /> The administrator password is encrypted using the keypair you specified when launching the VM.<br /><br />  **[IMPORTANT]**<br /> * Only RSA keypairs can decrypt the password of a Windows VM.<br /> * The administrator password is generated only on the first boot of the Windows VM. It is not returned after the first boot.

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


Gets the console output for a virtual machine (VM). This console is not in real-time. It is refreshed every two seconds and provides the most recent 64 KiB output.<br /><br />  **[IMPORTANT]**<br /> On Windows VMs, the console is handled only on the first boot. It returns no output after the first boot.

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


Lists one or more predefined VM types.

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


Lists one or more of your virtual machines (VMs).<br /> If you provide one or more VM IDs, this action returns a description for all of these VMs. If you do not provide any VM ID, this action returns a description for all of the VMs that belong to you. If you provide an invalid VM ID, an error is returned. If you provide the ID of a VM that does not belong to you, the description of this VM is not included in the response. The refresh interval for data returned by this action is one hour, meaning that a terminated VM may appear in the response.

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


Lists the status of one or more virtual machines (VMs).

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


Reboots one or more virtual machines (VMs).<br /> This operation sends a reboot request to one or more specified VMs. This is an asynchronous action that queues this reboot request. This action only reboots VMs that are valid and that belong to you.

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


Start one or more virtual machines (VMs).<br /> You can start only VMs that are valid and that belong to you.

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


Stops one or more running virtual machines (VMs).<br /> You can stop only VMs that are valid and that belong to you. Data stored in the VM RAM is lost.

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


Modifies the specified attributes of a virtual machine (VM).<br /> You must stop the VM before modifying the following attributes:<br /> * `NestedVirtualization`<br /> * `Performance`<br /> * `UserData`<br /> * `VmType`  To complete the update of secure boot, you need to do a stop/start of the VM. A simple restart is not sufficient, as the update is done when the VM goes through the stopped state. For the difference between stop/start and restart, see [About VM Lifecycle](https://docs.outscale.com/en/userguide/About-VM-Lifecycle.html).

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

