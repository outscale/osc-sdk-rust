# UpdateAccountRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_emails** | Option<**Vec<String>**> | One or more additional email addresses for the account. These addresses are used for notifications only. If you already have a list of additional emails registered, you cannot add to it, only replace it. To remove all registered additional emails, specify an empty list. | [optional]
**city** | Option<**String**> | The new city of the account owner. | [optional]
**company_name** | Option<**String**> | The new name of the company for the account. | [optional]
**country** | Option<**String**> | The new country of the account owner. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**email** | Option<**String**> | The main email address for the account. This address is used for your credentials and notifications. | [optional]
**first_name** | Option<**String**> | The new first name of the account owner. | [optional]
**job_title** | Option<**String**> | The new job title of the account owner. | [optional]
**last_name** | Option<**String**> | The new last name of the account owner. | [optional]
**mobile_number** | Option<**String**> | The new mobile phone number of the account owner. | [optional]
**phone_number** | Option<**String**> | The new landline phone number of the account owner. | [optional]
**state_province** | Option<**String**> | The new state/province of the account owner. | [optional]
**vat_number** | Option<**String**> | The new value added tax (VAT) number for the account. | [optional]
**zip_code** | Option<**String**> | The new ZIP code of the city. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


