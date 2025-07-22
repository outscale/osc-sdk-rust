# \SecurityGroupApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_security_group**](SecurityGroupApi.md#create_security_group) | **POST** /CreateSecurityGroup | 
[**delete_security_group**](SecurityGroupApi.md#delete_security_group) | **POST** /DeleteSecurityGroup | 
[**read_security_groups**](SecurityGroupApi.md#read_security_groups) | **POST** /ReadSecurityGroups | 



## create_security_group

> crate::models::CreateSecurityGroupResponse create_security_group(create_security_group_request)


Creates a security group.<br /> This action creates a security group either in the public Cloud or in a specified Net. By default, a default security group for use in the public Cloud and a default security group for use in a Net are created.<br /> When launching a virtual machine (VM), if no security group is explicitly specified, the appropriate default security group is assigned to the VM. Default security groups include a default rule granting VMs network access to each other.<br /> When creating a security group, you specify a name. Two security groups for use in the public Cloud or for use in a Net cannot have the same name.<br /> You can have up to 500 security groups in the public Cloud. You can create up to 500 security groups per Net.<br /> To add or remove rules, use the [CreateSecurityGroupRule](#createsecuritygrouprule) method.<br /><br /> For more information, see [About Security Groups](https://docs.outscale.com/en/userguide/About-Security-Groups.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_security_group_request** | Option<[**CreateSecurityGroupRequest**](CreateSecurityGroupRequest.md)> |  |  |

### Return type

[**crate::models::CreateSecurityGroupResponse**](CreateSecurityGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_security_group

> crate::models::DeleteSecurityGroupResponse delete_security_group(delete_security_group_request)


Deletes a specified security group.<br /> You can specify either the name of the security group or its ID.<br /> This action fails if the specified group is associated with a virtual machine (VM) or referenced by another security group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_security_group_request** | Option<[**DeleteSecurityGroupRequest**](DeleteSecurityGroupRequest.md)> |  |  |

### Return type

[**crate::models::DeleteSecurityGroupResponse**](DeleteSecurityGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_security_groups

> crate::models::ReadSecurityGroupsResponse read_security_groups(read_security_groups_request)


Lists one or more security groups.<br /> You can specify either the name of the security groups or their IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_security_groups_request** | Option<[**ReadSecurityGroupsRequest**](ReadSecurityGroupsRequest.md)> |  |  |

### Return type

[**crate::models::ReadSecurityGroupsResponse**](ReadSecurityGroupsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

