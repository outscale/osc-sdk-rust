# CreateAccountRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_emails** | Option<**Vec<String>**> | One or more additional email addresses for the account. These addresses are used for notifications only. If you already have a list of additional emails registered, you cannot add to it, only replace it. To remove all registered additional emails, specify an empty list. | [optional]
**city** | **String** | The city of the account owner. | 
**company_name** | **String** | The name of the company for the account. | 
**country** | **String** | The country of the account owner. | 
**customer_id** | **String** | The ID of the customer. It must be 8 digits. | 
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**email** | **String** | The main email address for the account. This address is used for your credentials and notifications. | 
**first_name** | **String** | The first name of the account owner. | 
**job_title** | Option<**String**> | The job title of the account owner. | [optional]
**last_name** | **String** | The last name of the account owner. | 
**mobile_number** | Option<**String**> | The mobile phone number of the account owner. | [optional]
**phone_number** | Option<**String**> | The landline phone number of the account owner. | [optional]
**state_province** | Option<**String**> | The state/province of the account. | [optional]
**vat_number** | Option<**String**> | The value added tax (VAT) number for the account. | [optional]
**zip_code** | **String** | The ZIP code of the city. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


