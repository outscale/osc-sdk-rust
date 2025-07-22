# \IdentityProviderApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**disable_outscale_login**](IdentityProviderApi.md#disable_outscale_login) | **POST** /DisableOutscaleLogin | 
[**disable_outscale_login_for_users**](IdentityProviderApi.md#disable_outscale_login_for_users) | **POST** /DisableOutscaleLoginForUsers | 
[**disable_outscale_login_per_users**](IdentityProviderApi.md#disable_outscale_login_per_users) | **POST** /DisableOutscaleLoginPerUsers | 
[**enable_outscale_login**](IdentityProviderApi.md#enable_outscale_login) | **POST** /EnableOutscaleLogin | 
[**enable_outscale_login_for_users**](IdentityProviderApi.md#enable_outscale_login_for_users) | **POST** /EnableOutscaleLoginForUsers | 
[**enable_outscale_login_per_users**](IdentityProviderApi.md#enable_outscale_login_per_users) | **POST** /EnableOutscaleLoginPerUsers | 



## disable_outscale_login

> crate::models::DisableOutscaleLoginResponse disable_outscale_login(disable_outscale_login_request)


Disables the possibility of logging in using the Outscale credentials of your root account when identity federation is activated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**disable_outscale_login_request** | Option<[**DisableOutscaleLoginRequest**](DisableOutscaleLoginRequest.md)> |  |  |

### Return type

[**crate::models::DisableOutscaleLoginResponse**](DisableOutscaleLoginResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_outscale_login_for_users

> crate::models::DisableOutscaleLoginResponse disable_outscale_login_for_users(disable_outscale_login_request)


Disables the possibility of logging in using the Outscale credentials of your EIM users when identity federation is activated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**disable_outscale_login_request** | Option<[**DisableOutscaleLoginRequest**](DisableOutscaleLoginRequest.md)> |  |  |

### Return type

[**crate::models::DisableOutscaleLoginResponse**](DisableOutscaleLoginResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_outscale_login_per_users

> crate::models::DisableOutscaleLoginPerUsersResponse disable_outscale_login_per_users(disable_outscale_login_per_users_request)


Disables the possibility for one or more specific users to log in using their Outscale credentials when identity federation is activated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**disable_outscale_login_per_users_request** | Option<[**DisableOutscaleLoginPerUsersRequest**](DisableOutscaleLoginPerUsersRequest.md)> |  |  |

### Return type

[**crate::models::DisableOutscaleLoginPerUsersResponse**](DisableOutscaleLoginPerUsersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_outscale_login

> crate::models::EnableOutscaleLoginResponse enable_outscale_login(enable_outscale_login_request)


Enables the possibility of logging in using the Outscale credentials of your root account when identity federation is activated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enable_outscale_login_request** | Option<[**EnableOutscaleLoginRequest**](EnableOutscaleLoginRequest.md)> |  |  |

### Return type

[**crate::models::EnableOutscaleLoginResponse**](EnableOutscaleLoginResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_outscale_login_for_users

> crate::models::EnableOutscaleLoginForUsersResponse enable_outscale_login_for_users(enable_outscale_login_for_users_request)


Enables the possibility for all your EIM users to log in using their Outscale credentials when identity federation is activated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enable_outscale_login_for_users_request** | Option<[**EnableOutscaleLoginForUsersRequest**](EnableOutscaleLoginForUsersRequest.md)> |  |  |

### Return type

[**crate::models::EnableOutscaleLoginForUsersResponse**](EnableOutscaleLoginForUsersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_outscale_login_per_users

> crate::models::EnableOutscaleLoginPerUsersResponse enable_outscale_login_per_users(enable_outscale_login_per_users_request)


Enables the possibility for one or more specific users to log in using their Outscale credentials when identity federation is activated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enable_outscale_login_per_users_request** | Option<[**EnableOutscaleLoginPerUsersRequest**](EnableOutscaleLoginPerUsersRequest.md)> |  |  |

### Return type

[**crate::models::EnableOutscaleLoginPerUsersResponse**](EnableOutscaleLoginPerUsersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

