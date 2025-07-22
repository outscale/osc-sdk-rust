# \SecurityGroupRuleApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_security_group_rule**](SecurityGroupRuleApi.md#create_security_group_rule) | **POST** /CreateSecurityGroupRule | 
[**delete_security_group_rule**](SecurityGroupRuleApi.md#delete_security_group_rule) | **POST** /DeleteSecurityGroupRule | 



## create_security_group_rule

> crate::models::CreateSecurityGroupRuleResponse create_security_group_rule(create_security_group_rule_request)


Adds one or more rules to a security group.<br /> Use the `SecurityGroupId` parameter to specify the security group for which you want to create a rule.<br /> Use the `Flow` parameter to specify if you want an inbound rule or an outbound rule.<br /><br /> An inbound rule allows the security group to receive traffic: * Either from a specific IP range (`IpRange` parameter) on a specific port range (`FromPortRange` and `ToPortRange` parameters) and specific protocol (`IpProtocol` parameter). * Or from another specific security group (`SecurityGroupAccountIdToLink` and `SecurityGroupNameToLink` parameters).<br />  (Net only) An outbound rule works similarly but allows the security group to send traffic rather than receive traffic.<br />  Alternatively, you can use the `Rules` parameter to add several rules at the same time.  **[NOTE]**<br /> * The modifications are effective as quickly as possible, but a small delay may occur.<br /> * By default, traffic between two security groups is allowed through both public and private IPs. To restrict traffic to private IPs only, contact our Support team at support@outscale.com.  For more information, see [About Security Group Rules](https://docs.outscale.com/en/userguide/About-Security-Group-Rules.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_security_group_rule_request** | Option<[**CreateSecurityGroupRuleRequest**](CreateSecurityGroupRuleRequest.md)> |  |  |

### Return type

[**crate::models::CreateSecurityGroupRuleResponse**](CreateSecurityGroupRuleResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_security_group_rule

> crate::models::DeleteSecurityGroupRuleResponse delete_security_group_rule(delete_security_group_rule_request)


Deletes one or more inbound or outbound rules from a security group. For the rule to be deleted, the values specified in the deletion request must exactly match the value of the existing rule.<br /> In case of TCP and UDP protocols, you have to indicate the destination port or range of ports. In case of ICMP protocol, you have to specify the ICMP type and code numbers.<br /> Rules (IP permissions) consist of the protocol, IP range or source security group.<br /> To remove outbound access to a destination security group, we recommend to use a set of IP permissions. We also recommend to specify the protocol in a set of IP permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_security_group_rule_request** | Option<[**DeleteSecurityGroupRuleRequest**](DeleteSecurityGroupRuleRequest.md)> |  |  |

### Return type

[**crate::models::DeleteSecurityGroupRuleResponse**](DeleteSecurityGroupRuleResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

