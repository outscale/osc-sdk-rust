# FiltersSecurityGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**descriptions** | Option<**Vec<String>**> | The descriptions of the security groups. | [optional]
**inbound_rule_account_ids** | Option<**Vec<String>**> | The account IDs that have been granted permissions. | [optional]
**inbound_rule_from_port_ranges** | Option<**Vec<i32>**> | The beginnings of the port ranges for the TCP and UDP protocols, or the ICMP type numbers. | [optional]
**inbound_rule_ip_ranges** | Option<**Vec<String>**> | The IP ranges that have been granted permissions, in CIDR notation (for example, `10.0.0.0/24`). | [optional]
**inbound_rule_protocols** | Option<**Vec<String>**> | The IP protocols for the permissions (`tcp` \\| `udp` \\| `icmp`, or a protocol number, or `-1` for all protocols). | [optional]
**inbound_rule_security_group_ids** | Option<**Vec<String>**> | The IDs of the security groups that have been granted permissions. | [optional]
**inbound_rule_security_group_names** | Option<**Vec<String>**> | The names of the security groups that have been granted permissions. | [optional]
**inbound_rule_to_port_ranges** | Option<**Vec<i32>**> | The ends of the port ranges for the TCP and UDP protocols, or the ICMP code numbers. | [optional]
**net_ids** | Option<**Vec<String>**> | The IDs of the Nets specified when the security groups were created. | [optional]
**outbound_rule_account_ids** | Option<**Vec<String>**> | The account IDs that have been granted permissions. | [optional]
**outbound_rule_from_port_ranges** | Option<**Vec<i32>**> | The beginnings of the port ranges for the TCP and UDP protocols, or the ICMP type numbers. | [optional]
**outbound_rule_ip_ranges** | Option<**Vec<String>**> | The IP ranges that have been granted permissions, in CIDR notation (for example, `10.0.0.0/24`). | [optional]
**outbound_rule_protocols** | Option<**Vec<String>**> | The IP protocols for the permissions (`tcp` \\| `udp` \\| `icmp`, or a protocol number, or `-1` for all protocols). | [optional]
**outbound_rule_security_group_ids** | Option<**Vec<String>**> | The IDs of the security groups that have been granted permissions. | [optional]
**outbound_rule_security_group_names** | Option<**Vec<String>**> | The names of the security groups that have been granted permissions. | [optional]
**outbound_rule_to_port_ranges** | Option<**Vec<i32>**> | The ends of the port ranges for the TCP and UDP protocols, or the ICMP code numbers. | [optional]
**security_group_ids** | Option<**Vec<String>**> | The IDs of the security groups. | [optional]
**security_group_names** | Option<**Vec<String>**> | The names of the security groups. | [optional]
**tag_keys** | Option<**Vec<String>**> | The keys of the tags associated with the security groups. | [optional]
**tag_values** | Option<**Vec<String>**> | The values of the tags associated with the security groups. | [optional]
**tags** | Option<**Vec<String>**> | The key/value combination of the tags associated with the security groups, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


