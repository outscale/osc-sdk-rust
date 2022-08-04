# LoadBalancer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_log** | Option<[**crate::models::AccessLog**](AccessLog.md)> |  | [optional]
**application_sticky_cookie_policies** | Option<[**Vec<crate::models::ApplicationStickyCookiePolicy>**](ApplicationStickyCookiePolicy.md)> | The stickiness policies defined for the load balancer. | [optional]
**backend_ips** | Option<**Vec<String>**> | One or more public IPs of back-end VMs. | [optional]
**backend_vm_ids** | Option<**Vec<String>**> | One or more IDs of back-end VMs for the load balancer. | [optional]
**dns_name** | Option<**String**> | The DNS name of the load balancer. | [optional]
**health_check** | Option<[**crate::models::HealthCheck**](HealthCheck.md)> |  | [optional]
**listeners** | Option<[**Vec<crate::models::Listener>**](Listener.md)> | The listeners for the load balancer. | [optional]
**load_balancer_name** | Option<**String**> | The name of the load balancer. | [optional]
**load_balancer_sticky_cookie_policies** | Option<[**Vec<crate::models::LoadBalancerStickyCookiePolicy>**](LoadBalancerStickyCookiePolicy.md)> | The policies defined for the load balancer. | [optional]
**load_balancer_type** | Option<**String**> | The type of load balancer. Valid only for load balancers in a Net.<br /> If `LoadBalancerType` is `internet-facing`, the load balancer has a public DNS name that resolves to a public IP.<br /> If `LoadBalancerType` is `internal`, the load balancer has a public DNS name that resolves to a private IP. | [optional]
**net_id** | Option<**String**> | The ID of the Net for the load balancer. | [optional]
**public_ip** | Option<**String**> | (internet-facing only) The public IP associated with the load balancer. | [optional]
**secured_cookies** | Option<**bool**> | Whether secure cookies are enabled for the load balancer. | [optional]
**security_groups** | Option<**Vec<String>**> | One or more IDs of security groups for the load balancers. Valid only for load balancers in a Net. | [optional]
**source_security_group** | Option<[**crate::models::SourceSecurityGroup**](SourceSecurityGroup.md)> |  | [optional]
**subnets** | Option<**Vec<String>**> | The ID of the Subnet in which the load balancer was created. | [optional]
**subregion_names** | Option<**Vec<String>**> | The ID of the Subregion in which the load balancer was created. | [optional]
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags associated with the load balancer. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


