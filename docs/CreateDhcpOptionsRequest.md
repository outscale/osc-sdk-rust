# CreateDhcpOptionsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain_name** | Option<**String**> | Specify a domain name (for example, MyCompany.com). You can specify only one domain name. | [optional]
**domain_name_servers** | Option<**Vec<String>**> | The IP addresses of domain name servers. If no IP addresses are specified, the `OutscaleProvidedDNS` value is set by default. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**ntp_servers** | Option<**Vec<String>**> | The IP addresses of the Network Time Protocol (NTP) servers. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


