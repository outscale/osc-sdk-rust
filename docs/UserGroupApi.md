# \UserGroupApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_user_to_user_group**](UserGroupApi.md#add_user_to_user_group) | **POST** /AddUserToUserGroup | 
[**create_user_group**](UserGroupApi.md#create_user_group) | **POST** /CreateUserGroup | 
[**delete_user_group**](UserGroupApi.md#delete_user_group) | **POST** /DeleteUserGroup | 
[**read_user_group**](UserGroupApi.md#read_user_group) | **POST** /ReadUserGroup | 
[**read_user_groups**](UserGroupApi.md#read_user_groups) | **POST** /ReadUserGroups | 
[**read_user_groups_per_user**](UserGroupApi.md#read_user_groups_per_user) | **POST** /ReadUserGroupsPerUser | 
[**remove_user_from_user_group**](UserGroupApi.md#remove_user_from_user_group) | **POST** /RemoveUserFromUserGroup | 
[**update_user_group**](UserGroupApi.md#update_user_group) | **POST** /UpdateUserGroup | 



## add_user_to_user_group

> crate::models::AddUserToUserGroupResponse add_user_to_user_group(add_user_to_user_group_request)


Adds a user to a specified group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_user_to_user_group_request** | Option<[**AddUserToUserGroupRequest**](AddUserToUserGroupRequest.md)> |  |  |

### Return type

[**crate::models::AddUserToUserGroupResponse**](AddUserToUserGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_group

> crate::models::CreateUserGroupResponse create_user_group(create_user_group_request)


Creates a group to which you can add users.<br /> You can also add an inline policy or link a managed policy to the group, which is applied to all its users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_group_request** | Option<[**CreateUserGroupRequest**](CreateUserGroupRequest.md)> |  |  |

### Return type

[**crate::models::CreateUserGroupResponse**](CreateUserGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_group

> crate::models::DeleteUserGroupResponse delete_user_group(delete_user_group_request)


Deletes a specified user group.<br />  **[WARNING]**<br /> The user group must be empty of any user and must not have any linked policy. Otherwise, you need to force the deletion.<br /> If you force the deletion, all inline policies will be deleted with the user group.<br />

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_user_group_request** | Option<[**DeleteUserGroupRequest**](DeleteUserGroupRequest.md)> |  |  |

### Return type

[**crate::models::DeleteUserGroupResponse**](DeleteUserGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_user_group

> crate::models::ReadUserGroupResponse read_user_group(read_user_group_request)


Lists information about a specified user group, including its users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_user_group_request** | Option<[**ReadUserGroupRequest**](ReadUserGroupRequest.md)> |  |  |

### Return type

[**crate::models::ReadUserGroupResponse**](ReadUserGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_user_groups

> crate::models::ReadUserGroupsResponse read_user_groups(read_user_groups_request)


Lists all the user groups of the account.<br /> The response can be filtered using either the PathPrefix or the UserGroupIds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_user_groups_request** | Option<[**ReadUserGroupsRequest**](ReadUserGroupsRequest.md)> |  |  |

### Return type

[**crate::models::ReadUserGroupsResponse**](ReadUserGroupsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_user_groups_per_user

> crate::models::ReadUserGroupsPerUserResponse read_user_groups_per_user(read_user_groups_per_user_request)


Lists the groups a specified user belongs to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_user_groups_per_user_request** | Option<[**ReadUserGroupsPerUserRequest**](ReadUserGroupsPerUserRequest.md)> |  |  |

### Return type

[**crate::models::ReadUserGroupsPerUserResponse**](ReadUserGroupsPerUserResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_from_user_group

> crate::models::RemoveUserFromUserGroupResponse remove_user_from_user_group(remove_user_from_user_group_request)


Removes a specified user from a specified group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_user_from_user_group_request** | Option<[**RemoveUserFromUserGroupRequest**](RemoveUserFromUserGroupRequest.md)> |  |  |

### Return type

[**crate::models::RemoveUserFromUserGroupResponse**](RemoveUserFromUserGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_group

> crate::models::UpdateUserGroupResponse update_user_group(update_user_group_request)


Modifies the name and/or the path of a specified group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_group_request** | Option<[**UpdateUserGroupRequest**](UpdateUserGroupRequest.md)> |  |  |

### Return type

[**crate::models::UpdateUserGroupResponse**](UpdateUserGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

