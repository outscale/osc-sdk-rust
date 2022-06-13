# CreateSecurityGroupRuleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**flow** | **String** | The direction of the flow: `Inbound` or `Outbound`. You can specify `Outbound` for Nets only. | 
**from_port_range** | Option<**i32**> | The beginning of the port range for the TCP and UDP protocols, or an ICMP type number. If you specify this parameter, you cannot specify the `Rules` parameter and its subparameters. | [optional]
**ip_protocol** | Option<**String**> | The IP protocol name (`tcp`, `udp`, `icmp`, or `-1` for all protocols). By default, `-1`. In a Net, this can also be an IP protocol number. For more information, see the [IANA.org website](https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml). If you specify this parameter, you cannot specify the `Rules` parameter and its subparameters. | [optional]
**ip_range** | Option<**String**> | The IP range for the security group rule, in CIDR notation (for example, 10.0.0.0/16). If you specify this parameter, you cannot specify the `Rules` parameter and its subparameters. | [optional]
**rules** | Option<[**Vec<crate::models::SecurityGroupRule>**](SecurityGroupRule.md)> | Information about the security group rule to create. If you specify this parent parameter and its subparameters, you cannot specify the following parent parameters: `FromPortRange`, `IpProtocol`, `IpRange`, and `ToPortRange`. | [optional]
**security_group_account_id_to_link** | Option<**String**> | The account ID of the owner of the security group for which you want to create a rule. | [optional]
**security_group_id** | **String** | The ID of the security group for which you want to create a rule. | 
**security_group_name_to_link** | Option<**String**> | The ID of the source security group. If you are in the Public Cloud, you can also specify the name of the source security group. | [optional]
**to_port_range** | Option<**i32**> | The end of the port range for the TCP and UDP protocols, or an ICMP code number. If you specify this parameter, you cannot specify the `Rules` parameter and its subparameters. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


