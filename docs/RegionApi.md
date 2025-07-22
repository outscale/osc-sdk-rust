# \RegionApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_regions**](RegionApi.md#read_regions) | **POST** /ReadRegions | 



## read_regions

> crate::models::ReadRegionsResponse read_regions(read_regions_request)


Lists one or more Regions of the OUTSCALE Cloud.<br /><br /> For more information, see [About Regions and Subregions](https://docs.outscale.com/en/userguide/About-Regions-and-Subregions.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_regions_request** | Option<[**ReadRegionsRequest**](ReadRegionsRequest.md)> |  |  |

### Return type

[**crate::models::ReadRegionsResponse**](ReadRegionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

