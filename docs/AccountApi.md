# \AccountApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_authentication**](AccountApi.md#check_authentication) | **POST** /CheckAuthentication | 
[**create_account**](AccountApi.md#create_account) | **POST** /CreateAccount | 
[**read_accounts**](AccountApi.md#read_accounts) | **POST** /ReadAccounts | 
[**read_consumption_account**](AccountApi.md#read_consumption_account) | **POST** /ReadConsumptionAccount | 
[**update_account**](AccountApi.md#update_account) | **POST** /UpdateAccount | 



## check_authentication

> crate::models::CheckAuthenticationResponse check_authentication(check_authentication_request)


Validates the authenticity of the account.

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


Creates an OUTSCALE account.<br /><br />  **[IMPORTANT]**<br /> * You need OUTSCALE credentials and the appropriate quotas to create an account via API. To get quotas, you can send an email to sales@outscale.com.<br /> * If you want to pass a numeral value as a string instead of an integer, you must wrap your string in additional quotes (for example, `'&quot;92000&quot;'`).  For more information, see [About Your Account](https://docs.outscale.com/en/userguide/About-Your-Account.html).

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


Gets information about the account that sent the request.

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


Gets information about the consumption of your account for each billable resource within the specified time period.

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


## update_account

> crate::models::UpdateAccountResponse update_account(update_account_request)


Updates the account information for the account that sends the request.

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

