# CreateLoadBalancerRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**listeners** | [**Vec<crate::models::ListenerForCreation>**](ListenerForCreation.md) | One or more listeners to create. | 
**load_balancer_name** | **String** | The unique name of the load balancer, with a maximum length of 32 alphanumeric characters and dashes (-). This name must not start or end with a dash. | 
**load_balancer_type** | Option<**String**> | The type of load balancer: `internet-facing` or `internal`. Use this parameter only for load balancers in a Net. | [optional]
**public_ip** | Option<**String**> | (internet-facing only) The public IP you want to associate with the load balancer. If not specified, a public IP owned by 3DS OUTSCALE is associated. | [optional]
**security_groups** | Option<**Vec<String>**> | (Net only) One or more IDs of security groups you want to assign to the load balancer. If not specified, the default security group of the Net is assigned to the load balancer. | [optional]
**subnets** | Option<**Vec<String>**> | (Net only) The ID of the Subnet in which you want to create the load balancer. Regardless of this Subnet, the load balancer can distribute traffic to all Subnets. This parameter is required in a Net. | [optional]
**subregion_names** | Option<**Vec<String>**> | (public Cloud only) The Subregion in which you want to create the load balancer. Regardless of this Subregion, the load balancer can distribute traffic to all Subregions. This parameter is required in the public Cloud. | [optional]
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags assigned to the load balancer. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


