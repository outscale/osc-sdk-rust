# \TagApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tags**](TagApi.md#create_tags) | **POST** /CreateTags | 
[**delete_tags**](TagApi.md#delete_tags) | **POST** /DeleteTags | 
[**read_tags**](TagApi.md#read_tags) | **POST** /ReadTags | 



## create_tags

> crate::models::CreateTagsResponse create_tags(create_tags_request)


Adds one or more tags to the specified resources.<br /> If a tag with the same key already exists for the resource, the tag value is replaced.<br /> You can tag the following resources using their IDs:<br /><br />  * Client gateways (cgw-xxxxxxxx)<br /> * DHCP options (dopt-xxxxxxxx)<br /> * Images (ami-xxxxxxxx)<br /> * Internet services (igw-xxxxxxxx)<br /> * Keypairs (key-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx)<br /> * NAT services (nat-xxxxxxxx)<br /> * Net endpoints (vpce-xxxxxxxx)<br /> * Net peerings (vpcx-xxxxxxxx)<br /> * Nets (vpc-xxxxxxxx)<br /> * Network interface cards (NIC) (eni-xxxxxxxx)<br /> * OMI export tasks (image-export-xxxxxxxx)<br /> * OMIs (ami-xxxxxxxx)<br /> * Public IPs (eipalloc-xxxxxxxx)<br /> * Route tables (rtb-xxxxxxxx)<br /> * Security groups (sg-xxxxxxxx)<br /> * Snapshot export tasks (snap-export-xxxxxxxx) * Snapshots (snap-xxxxxxxx)<br /> * Subnets (subnet-xxxxxxxx)<br /> * Virtual gateways (vgw-xxxxxxxx)<br /> * Virtual machines (VMs) (i-xxxxxxxx)<br /> * Volumes (vol-xxxxxxxx)<br /> * VPN connections (vpn-xxxxxxxx)<br />  For more information, see [About Tags](https://docs.outscale.com/en/userguide/About-Tags.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_tags_request** | Option<[**CreateTagsRequest**](CreateTagsRequest.md)> |  |  |

### Return type

[**crate::models::CreateTagsResponse**](CreateTagsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tags

> crate::models::DeleteTagsResponse delete_tags(delete_tags_request)


Deletes one or more tags from the specified resources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_tags_request** | Option<[**DeleteTagsRequest**](DeleteTagsRequest.md)> |  |  |

### Return type

[**crate::models::DeleteTagsResponse**](DeleteTagsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_tags

> crate::models::ReadTagsResponse read_tags(read_tags_request)


Lists one or more tags for your resources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_tags_request** | Option<[**ReadTagsRequest**](ReadTagsRequest.md)> |  |  |

### Return type

[**crate::models::ReadTagsResponse**](ReadTagsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

