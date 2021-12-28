# CreateServerCertificateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**body** | **String** | The PEM-encoded X509 certificate. | 
**chain** | Option<**String**> | The PEM-encoded intermediate certification authorities. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**name** | **String** | A unique name for the certificate. Constraints: 1-128 alphanumeric characters, pluses (+), equals (=), commas (,), periods (.), at signs (@), minuses (-), or underscores (_). | 
**path** | Option<**String**> | The path to the server certificate, set to a slash (/) if not specified. | [optional]
**private_key** | **String** | The PEM-encoded private key matching the certificate. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


