# \PolicyApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_policy**](PolicyApi.md#create_policy) | **POST** /CreatePolicy | 
[**create_policy_version**](PolicyApi.md#create_policy_version) | **POST** /CreatePolicyVersion | 
[**delete_policy**](PolicyApi.md#delete_policy) | **POST** /DeletePolicy | 
[**delete_policy_version**](PolicyApi.md#delete_policy_version) | **POST** /DeletePolicyVersion | 
[**delete_user_group_policy**](PolicyApi.md#delete_user_group_policy) | **POST** /DeleteUserGroupPolicy | 
[**delete_user_policy**](PolicyApi.md#delete_user_policy) | **POST** /DeleteUserPolicy | 
[**link_managed_policy_to_user_group**](PolicyApi.md#link_managed_policy_to_user_group) | **POST** /LinkManagedPolicyToUserGroup | 
[**link_policy**](PolicyApi.md#link_policy) | **POST** /LinkPolicy | 
[**put_user_group_policy**](PolicyApi.md#put_user_group_policy) | **POST** /PutUserGroupPolicy | 
[**put_user_policy**](PolicyApi.md#put_user_policy) | **POST** /PutUserPolicy | 
[**read_entities_linked_to_policy**](PolicyApi.md#read_entities_linked_to_policy) | **POST** /ReadEntitiesLinkedToPolicy | 
[**read_linked_policies**](PolicyApi.md#read_linked_policies) | **POST** /ReadLinkedPolicies | 
[**read_managed_policies_linked_to_user_group**](PolicyApi.md#read_managed_policies_linked_to_user_group) | **POST** /ReadManagedPoliciesLinkedToUserGroup | 
[**read_policies**](PolicyApi.md#read_policies) | **POST** /ReadPolicies | 
[**read_policy**](PolicyApi.md#read_policy) | **POST** /ReadPolicy | 
[**read_policy_version**](PolicyApi.md#read_policy_version) | **POST** /ReadPolicyVersion | 
[**read_policy_versions**](PolicyApi.md#read_policy_versions) | **POST** /ReadPolicyVersions | 
[**read_user_group_policies**](PolicyApi.md#read_user_group_policies) | **POST** /ReadUserGroupPolicies | 
[**read_user_group_policy**](PolicyApi.md#read_user_group_policy) | **POST** /ReadUserGroupPolicy | 
[**read_user_policies**](PolicyApi.md#read_user_policies) | **POST** /ReadUserPolicies | 
[**read_user_policy**](PolicyApi.md#read_user_policy) | **POST** /ReadUserPolicy | 
[**set_default_policy_version**](PolicyApi.md#set_default_policy_version) | **POST** /SetDefaultPolicyVersion | 
[**unlink_managed_policy_from_user_group**](PolicyApi.md#unlink_managed_policy_from_user_group) | **POST** /UnlinkManagedPolicyFromUserGroup | 
[**unlink_policy**](PolicyApi.md#unlink_policy) | **POST** /UnlinkPolicy | 



## create_policy

> crate::models::CreatePolicyResponse create_policy(create_policy_request)


Creates a managed policy to apply to a user.<br /> This action creates a policy version and sets v1 as the default one.

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


Creates a version of a specified managed policy.<br /> A managed policy can have up to five versions. <br /><br />  **[IMPORTANT]**<br /> A delay of up to 15 seconds can occur when attaching, detaching, or updating a managed policy.

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


Deletes a managed policy.<br /> Before deleting a managed policy, you must unlink all users linked to it and delete all the versions of the policy, except the default one, using the `DeletePolicyVersion` method.

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


Deletes a specified version of a managed policy, if it is not set as the default one. <br /><br />  **[IMPORTANT]**<br /> A delay of up to 15 seconds can occur when attaching, detaching, or updating a managed policy.

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


## delete_user_group_policy

> crate::models::DeleteUserGroupPolicyResponse delete_user_group_policy(delete_user_group_policy_request)


Deletes a specified inline policy from a specific group. <br /><br />  **[IMPORTANT]**<br /> A delay of up to 15 seconds can occur when creating or deleting an inline policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_user_group_policy_request** | Option<[**DeleteUserGroupPolicyRequest**](DeleteUserGroupPolicyRequest.md)> |  |  |

### Return type

[**crate::models::DeleteUserGroupPolicyResponse**](DeleteUserGroupPolicyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_policy

> crate::models::DeleteUserPolicyResponse delete_user_policy(delete_user_policy_request)


Deletes a specified inline policy from a specific user. <br /><br />  **[IMPORTANT]**<br /> A delay of up to 15 seconds can occur when creating or deleting an inline policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_user_policy_request** | Option<[**DeleteUserPolicyRequest**](DeleteUserPolicyRequest.md)> |  |  |

### Return type

[**crate::models::DeleteUserPolicyResponse**](DeleteUserPolicyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_managed_policy_to_user_group

> crate::models::LinkManagedPolicyToUserGroupResponse link_managed_policy_to_user_group(link_managed_policy_to_user_group_request)


Links a managed policy to a specific group. This policy applies to all the users contained in this group. <br /><br />  **[IMPORTANT]**<br /> A delay of up to 15 seconds can occur when attaching, detaching, or updating a managed policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_managed_policy_to_user_group_request** | Option<[**LinkManagedPolicyToUserGroupRequest**](LinkManagedPolicyToUserGroupRequest.md)> |  |  |

### Return type

[**crate::models::LinkManagedPolicyToUserGroupResponse**](LinkManagedPolicyToUserGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_policy

> crate::models::LinkPolicyResponse link_policy(link_policy_request)


Links a managed policy to a specific user. <br /><br />  **[IMPORTANT]**<br /> A delay of up to 15 seconds can occur when attaching, detaching, or updating a managed policy.

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


## put_user_group_policy

> crate::models::PutUserGroupPolicyResponse put_user_group_policy(put_user_group_policy_request)


Creates or updates an inline policy included in a specified group.<br /> The policy is automatically applied to all the users of the group after its creation. <br /><br />  **[IMPORTANT]**<br /> A delay of up to 15 seconds can occur when creating or deleting an inline policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**put_user_group_policy_request** | Option<[**PutUserGroupPolicyRequest**](PutUserGroupPolicyRequest.md)> |  |  |

### Return type

[**crate::models::PutUserGroupPolicyResponse**](PutUserGroupPolicyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_user_policy

> crate::models::PutUserPolicyResponse put_user_policy(put_user_policy_request)


Creates or updates an inline policy included in a specified user.<br /> The policy is automatically applied to the user after its creation. <br /><br />  **[IMPORTANT]**<br /> A delay of up to 15 seconds can occur when creating or deleting an inline policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**put_user_policy_request** | Option<[**PutUserPolicyRequest**](PutUserPolicyRequest.md)> |  |  |

### Return type

[**crate::models::PutUserPolicyResponse**](PutUserPolicyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_entities_linked_to_policy

> crate::models::ReadEntitiesLinkedToPolicyResponse read_entities_linked_to_policy(read_entities_linked_to_policy_request)


Lists all entities (account, users, or user groups) linked to a specific managed policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_entities_linked_to_policy_request** | Option<[**ReadEntitiesLinkedToPolicyRequest**](ReadEntitiesLinkedToPolicyRequest.md)> |  |  |

### Return type

[**crate::models::ReadEntitiesLinkedToPolicyResponse**](ReadEntitiesLinkedToPolicyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_linked_policies

> crate::models::ReadLinkedPoliciesResponse read_linked_policies(read_linked_policies_request)


Lists the managed policies linked to a specified user.

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


## read_managed_policies_linked_to_user_group

> crate::models::ReadManagedPoliciesLinkedToUserGroupResponse read_managed_policies_linked_to_user_group(read_managed_policies_linked_to_user_group_request)


Lists the managed policies linked to a specified group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_managed_policies_linked_to_user_group_request** | Option<[**ReadManagedPoliciesLinkedToUserGroupRequest**](ReadManagedPoliciesLinkedToUserGroupRequest.md)> |  |  |

### Return type

[**crate::models::ReadManagedPoliciesLinkedToUserGroupResponse**](ReadManagedPoliciesLinkedToUserGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_policies

> crate::models::ReadPoliciesResponse read_policies(read_policies_request)


Lists all the managed policies available for your account.

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


Lists information about a specified managed policy.

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


Lists information about a specified version of a managed policy.

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


Lists information about all the policy versions of a specified managed policy.

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


## read_user_group_policies

> crate::models::ReadUserGroupPoliciesResponse read_user_group_policies(read_user_group_policies_request)


Lists the names of the inline policies embedded in a specific group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_user_group_policies_request** | Option<[**ReadUserGroupPoliciesRequest**](ReadUserGroupPoliciesRequest.md)> |  |  |

### Return type

[**crate::models::ReadUserGroupPoliciesResponse**](ReadUserGroupPoliciesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_user_group_policy

> crate::models::ReadUserGroupPolicyResponse read_user_group_policy(read_user_group_policy_request)


Returns information about an inline policy included in a specified group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_user_group_policy_request** | Option<[**ReadUserGroupPolicyRequest**](ReadUserGroupPolicyRequest.md)> |  |  |

### Return type

[**crate::models::ReadUserGroupPolicyResponse**](ReadUserGroupPolicyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_user_policies

> crate::models::ReadUserPoliciesResponse read_user_policies(read_user_policies_request)


Lists the names of inline policies included in a specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_user_policies_request** | Option<[**ReadUserPoliciesRequest**](ReadUserPoliciesRequest.md)> |  |  |

### Return type

[**crate::models::ReadUserPoliciesResponse**](ReadUserPoliciesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_user_policy

> crate::models::ReadUserPolicyResponse read_user_policy(read_user_policy_request)


Returns information about an inline policy included in a specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_user_policy_request** | Option<[**ReadUserPolicyRequest**](ReadUserPolicyRequest.md)> |  |  |

### Return type

[**crate::models::ReadUserPolicyResponse**](ReadUserPolicyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_default_policy_version

> crate::models::SetDefaultPolicyVersionResponse set_default_policy_version(set_default_policy_version_request)


Sets a specified version of a managed policy as the default (operative) one.<br /> You can modify the default version of a policy at any time. <br /><br />  **[IMPORTANT]**<br /> A delay of up to 15 seconds can occur when attaching, detaching, or updating a managed policy.

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


## unlink_managed_policy_from_user_group

> crate::models::UnlinkManagedPolicyFromUserGroupResponse unlink_managed_policy_from_user_group(unlink_managed_policy_from_user_group_request)


Unlinks a managed policy from a specific group. <br /><br />  **[IMPORTANT]**<br /> A delay of up to 15 seconds can occur when attaching, detaching, or updating a managed policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unlink_managed_policy_from_user_group_request** | Option<[**UnlinkManagedPolicyFromUserGroupRequest**](UnlinkManagedPolicyFromUserGroupRequest.md)> |  |  |

### Return type

[**crate::models::UnlinkManagedPolicyFromUserGroupResponse**](UnlinkManagedPolicyFromUserGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_policy

> crate::models::UnlinkPolicyResponse unlink_policy(unlink_policy_request)


Removes a managed policy from a specific user. <br /><br />  **[IMPORTANT]**<br /> A delay of up to 15 seconds can occur when attaching, detaching, or updating a managed policy.

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

