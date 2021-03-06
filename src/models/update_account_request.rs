/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateAccountRequest {
    /// One or more additional email addresses for the account. These addresses are used for notifications only. If you already have a list of additional emails registered, you cannot add to it, only replace it. To remove all registered additional emails, specify an empty list.
    #[serde(rename = "AdditionalEmails", skip_serializing_if = "Option::is_none")]
    pub additional_emails: Option<Vec<String>>,
    /// The new city of the account owner.
    #[serde(rename = "City", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The new name of the company for the account.
    #[serde(rename = "CompanyName", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// The new country of the account owner.
    #[serde(rename = "Country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The main email address for the account. This address is used for your credentials and notifications.
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The new first name of the account owner.
    #[serde(rename = "FirstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The new job title of the account owner.
    #[serde(rename = "JobTitle", skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// The new last name of the account owner.
    #[serde(rename = "LastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The new mobile phone number of the account owner.
    #[serde(rename = "MobileNumber", skip_serializing_if = "Option::is_none")]
    pub mobile_number: Option<String>,
    /// The new landline phone number of the account owner.
    #[serde(rename = "PhoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The new state/province of the account owner.
    #[serde(rename = "StateProvince", skip_serializing_if = "Option::is_none")]
    pub state_province: Option<String>,
    /// The new value added tax (VAT) number for the account.
    #[serde(rename = "VatNumber", skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
    /// The new ZIP code of the city.
    #[serde(rename = "ZipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

impl UpdateAccountRequest {
    pub fn new() -> UpdateAccountRequest {
        UpdateAccountRequest {
            additional_emails: None,
            city: None,
            company_name: None,
            country: None,
            dry_run: None,
            email: None,
            first_name: None,
            job_title: None,
            last_name: None,
            mobile_number: None,
            phone_number: None,
            state_province: None,
            vat_number: None,
            zip_code: None,
        }
    }
}
