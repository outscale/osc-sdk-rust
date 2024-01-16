# \PolicyApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_policy**](PolicyApi.md#create_policy) | **POST** /CreatePolicy | 
[**create_policy_version**](PolicyApi.md#create_policy_version) | **POST** /CreatePolicyVersion | 
[**delete_policy**](PolicyApi.md#delete_policy) | **POST** /DeletePolicy | 
[**delete_policy_version**](PolicyApi.md#delete_policy_version) | **POST** /DeletePolicyVersion | 
[**link_policy**](PolicyApi.md#link_policy) | **POST** /LinkPolicy | 
[**read_linked_policies**](PolicyApi.md#read_linked_policies) | **POST** /ReadLinkedPolicies | 
[**read_policies**](PolicyApi.md#read_policies) | **POST** /ReadPolicies | 
[**read_policy**](PolicyApi.md#read_policy) | **POST** /ReadPolicy | 
[**read_policy_version**](PolicyApi.md#read_policy_version) | **POST** /ReadPolicyVersion | 
[**read_policy_versions**](PolicyApi.md#read_policy_versions) | **POST** /ReadPolicyVersions | 
[**set_default_policy_version**](PolicyApi.md#set_default_policy_version) | **POST** /SetDefaultPolicyVersion | 
[**unlink_policy**](PolicyApi.md#unlink_policy) | **POST** /UnlinkPolicy | 



## create_policy

> crate::models::CreatePolicyResponse create_policy(create_policy_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_policy_request** | Option<[**CreatePolicyRequest**](CreatePolicyRequest.md)> |  |  |

### Return type

[**crate::models::CreatePolicyResponse**](CreatePolicyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_policy_version

> crate::models::CreatePolicyVersionResponse create_policy_version(create_policy_version_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_policy_version_request** | Option<[**CreatePolicyVersionRequest**](CreatePolicyVersionRequest.md)> |  |  |

### Return type

[**crate::models::CreatePolicyVersionResponse**](CreatePolicyVersionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_policy

> crate::models::DeletePolicyResponse delete_policy(delete_policy_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_policy_request** | Option<[**DeletePolicyRequest**](DeletePolicyRequest.md)> |  |  |

### Return type

[**crate::models::DeletePolicyResponse**](DeletePolicyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_policy_version

> crate::models::DeletePolicyVersionResponse delete_policy_version(delete_policy_version_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_policy_version_request** | Option<[**DeletePolicyVersionRequest**](DeletePolicyVersionRequest.md)> |  |  |

### Return type

[**crate::models::DeletePolicyVersionResponse**](DeletePolicyVersionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_policy

> crate::models::LinkPolicyResponse link_policy(link_policy_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_policy_request** | Option<[**LinkPolicyRequest**](LinkPolicyRequest.md)> |  |  |

### Return type

[**crate::models::LinkPolicyResponse**](LinkPolicyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_linked_policies

> crate::models::ReadLinkedPoliciesResponse read_linked_policies(read_linked_policies_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_linked_policies_request** | Option<[**ReadLinkedPoliciesRequest**](ReadLinkedPoliciesRequest.md)> |  |  |

### Return type

[**crate::models::ReadLinkedPoliciesResponse**](ReadLinkedPoliciesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_policies

> crate::models::ReadPoliciesResponse read_policies(read_policies_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_policies_request** | Option<[**ReadPoliciesRequest**](ReadPoliciesRequest.md)> |  |  |

### Return type

[**crate::models::ReadPoliciesResponse**](ReadPoliciesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_policy

> crate::models::ReadPolicyResponse read_policy(read_policy_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_policy_request** | Option<[**ReadPolicyRequest**](ReadPolicyRequest.md)> |  |  |

### Return type

[**crate::models::ReadPolicyResponse**](ReadPolicyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_policy_version

> crate::models::ReadPolicyVersionResponse read_policy_version(read_policy_version_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_policy_version_request** | Option<[**ReadPolicyVersionRequest**](ReadPolicyVersionRequest.md)> |  |  |

### Return type

[**crate::models::ReadPolicyVersionResponse**](ReadPolicyVersionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_policy_versions

> crate::models::ReadPolicyVersionsResponse read_policy_versions(read_policy_versions_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_policy_versions_request** | Option<[**ReadPolicyVersionsRequest**](ReadPolicyVersionsRequest.md)> |  |  |

### Return type

[**crate::models::ReadPolicyVersionsResponse**](ReadPolicyVersionsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_default_policy_version

> crate::models::SetDefaultPolicyVersionResponse set_default_policy_version(set_default_policy_version_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_default_policy_version_request** | Option<[**SetDefaultPolicyVersionRequest**](SetDefaultPolicyVersionRequest.md)> |  |  |

### Return type

[**crate::models::SetDefaultPolicyVersionResponse**](SetDefaultPolicyVersionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_policy

> crate::models::UnlinkPolicyResponse unlink_policy(unlink_policy_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unlink_policy_request** | Option<[**UnlinkPolicyRequest**](UnlinkPolicyRequest.md)> |  |  |

### Return type

[**crate::models::UnlinkPolicyResponse**](UnlinkPolicyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

