# DeleteSecurityGroupRuleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**flow** | **String** | The direction of the flow: `Inbound` or `Outbound`. You can specify `Outbound` for Nets only. | 
**from_port_range** | Option<**i32**> | The beginning of the port range for the TCP and UDP protocols, or an ICMP type number. | [optional]
**ip_protocol** | Option<**String**> | The IP protocol name (`tcp`, `udp`, `icmp`, or `-1` for all protocols). By default, `-1`. In a Net, this can also be an IP protocol number. For more information, see the [IANA.org website](https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml). | [optional]
**ip_range** | Option<**String**> | The IP range for the security group rule, in CIDR notation (for example, `10.0.0.0/16`). | [optional]
**rules** | Option<[**Vec<crate::models::SecurityGroupRule>**](SecurityGroupRule.md)> | One or more rules you want to delete from the security group. | [optional]
**security_group_account_id_to_unlink** | Option<**String**> | The account ID of the owner of the security group you want to delete a rule from. | [optional]
**security_group_id** | **String** | The ID of the security group you want to delete a rule from. | 
**security_group_name_to_unlink** | Option<**String**> | The ID of the source security group. If you are in the Public Cloud, you can also specify the name of the source security group. | [optional]
**to_port_range** | Option<**i32**> | The end of the port range for the TCP and UDP protocols, or an ICMP code number. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


