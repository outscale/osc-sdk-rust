# SecurityGroupRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_port_range** | Option<**i32**> | The beginning of the port range for the TCP and UDP protocols, or an ICMP type number. | [optional]
**ip_protocol** | Option<**String**> | The IP protocol name (`tcp`, `udp`, `icmp`, or `-1` for all protocols). By default, `-1`. In a Net, this can also be an IP protocol number. For more information, see the [IANA.org website](https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml). | [optional]
**ip_ranges** | Option<**Vec<String>**> | One or more IP ranges for the security group rules, in CIDR notation (for example, `10.0.0.0/16`). | [optional]
**security_groups_members** | Option<[**Vec<crate::models::SecurityGroupsMember>**](SecurityGroupsMember.md)> | Information about one or more members of a security group. | [optional]
**service_ids** | Option<**Vec<String>**> | One or more service IDs to allow traffic from a Net to access the corresponding OUTSCALE services. For more information, see [ReadNetAccessPointServices](#readnetaccesspointservices). | [optional]
**to_port_range** | Option<**i32**> | The end of the port range for the TCP and UDP protocols, or an ICMP code number. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


