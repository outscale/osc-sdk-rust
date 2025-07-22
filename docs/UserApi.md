# \UserApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user**](UserApi.md#create_user) | **POST** /CreateUser | 
[**delete_user**](UserApi.md#delete_user) | **POST** /DeleteUser | 
[**read_users**](UserApi.md#read_users) | **POST** /ReadUsers | 
[**update_user**](UserApi.md#update_user) | **POST** /UpdateUser | 



## create_user

> crate::models::CreateUserResponse create_user(create_user_request)


Creates an EIM user for your account.<br /><br /> For more information, see [About EIM Users](https://docs.outscale.com/en/userguide/About-EIM-Users.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_request** | Option<[**CreateUserRequest**](CreateUserRequest.md)> |  |  |

### Return type

[**crate::models::CreateUserResponse**](CreateUserResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> crate::models::DeleteUserResponse delete_user(delete_user_request)


Deletes a specified EIM user. The EIM user must not belong to any group, nor have any key or linked policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_user_request** | Option<[**DeleteUserRequest**](DeleteUserRequest.md)> |  |  |

### Return type

[**crate::models::DeleteUserResponse**](DeleteUserResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_users

> crate::models::ReadUsersResponse read_users(read_users_request)


Lists all EIM users in the account.<br /> The response can be filtered using the UserIds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_users_request** | Option<[**ReadUsersRequest**](ReadUsersRequest.md)> |  |  |

### Return type

[**crate::models::ReadUsersResponse**](ReadUsersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::UpdateUserResponse update_user(update_user_request)


Modifies the name and/or the path of a specified EIM user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_request** | Option<[**UpdateUserRequest**](UpdateUserRequest.md)> |  |  |

### Return type

[**crate::models::UpdateUserResponse**](UpdateUserResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

