# CreateNatServiceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_token** | Option<**String**> | A unique identifier which enables you to manage the idempotency.<br />With OSC CLI, if you want to specify a number for this value, you must wrap it in two pairs of quotes to make sure the value is parsed as a string: `--ClientToken '&quot;12345678&quot;'`. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**public_ip_id** | **String** | The allocation ID of the public IP to associate with the NAT service.<br /> If the public IP is already associated with another resource, you must first disassociate it. | 
**subnet_id** | **String** | The ID of the Subnet in which you want to create the NAT service. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


