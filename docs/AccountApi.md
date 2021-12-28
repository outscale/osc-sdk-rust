# \AccountApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_authentication**](AccountApi.md#check_authentication) | **POST** /CheckAuthentication | 
[**create_account**](AccountApi.md#create_account) | **POST** /CreateAccount | 
[**read_accounts**](AccountApi.md#read_accounts) | **POST** /ReadAccounts | 
[**read_consumption_account**](AccountApi.md#read_consumption_account) | **POST** /ReadConsumptionAccount | 
[**reset_account_password**](AccountApi.md#reset_account_password) | **POST** /ResetAccountPassword | 
[**send_reset_password_email**](AccountApi.md#send_reset_password_email) | **POST** /SendResetPasswordEmail | 
[**update_account**](AccountApi.md#update_account) | **POST** /UpdateAccount | 



## check_authentication

> crate::models::CheckAuthenticationResponse check_authentication(check_authentication_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_authentication_request** | Option<[**CheckAuthenticationRequest**](CheckAuthenticationRequest.md)> |  |  |

### Return type

[**crate::models::CheckAuthenticationResponse**](CheckAuthenticationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_account

> crate::models::CreateAccountResponse create_account(create_account_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_account_request** | Option<[**CreateAccountRequest**](CreateAccountRequest.md)> |  |  |

### Return type

[**crate::models::CreateAccountResponse**](CreateAccountResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_accounts

> crate::models::ReadAccountsResponse read_accounts(read_accounts_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_accounts_request** | Option<[**ReadAccountsRequest**](ReadAccountsRequest.md)> |  |  |

### Return type

[**crate::models::ReadAccountsResponse**](ReadAccountsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_consumption_account

> crate::models::ReadConsumptionAccountResponse read_consumption_account(read_consumption_account_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_consumption_account_request** | Option<[**ReadConsumptionAccountRequest**](ReadConsumptionAccountRequest.md)> |  |  |

### Return type

[**crate::models::ReadConsumptionAccountResponse**](ReadConsumptionAccountResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_account_password

> crate::models::ResetAccountPasswordResponse reset_account_password(reset_account_password_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reset_account_password_request** | Option<[**ResetAccountPasswordRequest**](ResetAccountPasswordRequest.md)> |  |  |

### Return type

[**crate::models::ResetAccountPasswordResponse**](ResetAccountPasswordResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_reset_password_email

> crate::models::SendResetPasswordEmailResponse send_reset_password_email(send_reset_password_email_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**send_reset_password_email_request** | Option<[**SendResetPasswordEmailRequest**](SendResetPasswordEmailRequest.md)> |  |  |

### Return type

[**crate::models::SendResetPasswordEmailResponse**](SendResetPasswordEmailResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_account

> crate::models::UpdateAccountResponse update_account(update_account_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_account_request** | Option<[**UpdateAccountRequest**](UpdateAccountRequest.md)> |  |  |

### Return type

[**crate::models::UpdateAccountResponse**](UpdateAccountResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

