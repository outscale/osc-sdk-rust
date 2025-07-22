# \SubregionApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_subregions**](SubregionApi.md#read_subregions) | **POST** /ReadSubregions | 



## read_subregions

> crate::models::ReadSubregionsResponse read_subregions(read_subregions_request)


Lists one or more of the enabled Subregions that you can access in the current Region.<br /><br />  For more information, see [About Regions and Subregions](https://docs.outscale.com/en/userguide/About-Regions-and-Subregions.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_subregions_request** | Option<[**ReadSubregionsRequest**](ReadSubregionsRequest.md)> |  |  |

### Return type

[**crate::models::ReadSubregionsResponse**](ReadSubregionsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

