/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://wiki.outscale.net/display/EN/3DS+OUTSCALE+APIs+Reference).<br /><br />  You can also manage your resources using the [Cockpit](https://wiki.outscale.net/display/EN/About+Cockpit) web interface.
 *
 * The version of the OpenAPI document: 1.16
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    /// The city of the account owner.
    #[serde(rename = "City")]
    pub city: String,
    /// The name of the company for the account.
    #[serde(rename = "CompanyName")]
    pub company_name: String,
    /// The country of the account owner.
    #[serde(rename = "Country")]
    pub country: String,
    /// The ID of the customer. It must be 8 digits.
    #[serde(rename = "CustomerId")]
    pub customer_id: String,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The email address for the account.
    #[serde(rename = "Email")]
    pub email: String,
    /// The first name of the account owner.
    #[serde(rename = "FirstName")]
    pub first_name: String,
    /// The job title of the account owner.
    #[serde(rename = "JobTitle", skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// The last name of the account owner.
    #[serde(rename = "LastName")]
    pub last_name: String,
    /// The mobile phone number of the account owner.
    #[serde(rename = "MobileNumber", skip_serializing_if = "Option::is_none")]
    pub mobile_number: Option<String>,
    /// The landline phone number of the account owner.
    #[serde(rename = "PhoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The state/province of the account.
    #[serde(rename = "StateProvince", skip_serializing_if = "Option::is_none")]
    pub state_province: Option<String>,
    /// The value added tax (VAT) number for the account.
    #[serde(rename = "VatNumber", skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
    /// The ZIP code of the city.
    #[serde(rename = "ZipCode")]
    pub zip_code: String,
}

impl CreateAccountRequest {
    pub fn new(
        city: String,
        company_name: String,
        country: String,
        customer_id: String,
        email: String,
        first_name: String,
        last_name: String,
        zip_code: String,
    ) -> CreateAccountRequest {
        CreateAccountRequest {
            city,
            company_name,
            country,
            customer_id,
            dry_run: None,
            email,
            first_name,
            job_title: None,
            last_name,
            mobile_number: None,
            phone_number: None,
            state_province: None,
            vat_number: None,
            zip_code,
        }
    }
}
