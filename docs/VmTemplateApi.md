# \VmTemplateApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_vm_template**](VmTemplateApi.md#create_vm_template) | **POST** /CreateVmTemplate | 
[**delete_vm_template**](VmTemplateApi.md#delete_vm_template) | **POST** /DeleteVmTemplate | 
[**read_vm_templates**](VmTemplateApi.md#read_vm_templates) | **POST** /ReadVmTemplates | 
[**update_vm_template**](VmTemplateApi.md#update_vm_template) | **POST** /UpdateVmTemplate | 



## create_vm_template

> crate::models::CreateVmTemplateResponse create_vm_template(create_vm_template_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_vm_template_request** | Option<[**CreateVmTemplateRequest**](CreateVmTemplateRequest.md)> |  |  |

### Return type

[**crate::models::CreateVmTemplateResponse**](CreateVmTemplateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_vm_template

> crate::models::DeleteVmTemplateResponse delete_vm_template(delete_vm_template_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_vm_template_request** | Option<[**DeleteVmTemplateRequest**](DeleteVmTemplateRequest.md)> |  |  |

### Return type

[**crate::models::DeleteVmTemplateResponse**](DeleteVmTemplateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_vm_templates

> crate::models::ReadVmTemplatesResponse read_vm_templates(read_vm_templates_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_vm_templates_request** | Option<[**ReadVmTemplatesRequest**](ReadVmTemplatesRequest.md)> |  |  |

### Return type

[**crate::models::ReadVmTemplatesResponse**](ReadVmTemplatesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_vm_template

> crate::models::UpdateVmTemplateResponse update_vm_template(update_vm_template_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_vm_template_request** | Option<[**UpdateVmTemplateRequest**](UpdateVmTemplateRequest.md)> |  |  |

### Return type

[**crate::models::UpdateVmTemplateResponse**](UpdateVmTemplateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

