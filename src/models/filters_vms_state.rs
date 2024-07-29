/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> Throttling: To protect against overloads, the number of identical requests allowed in a given time period is limited.<br /> Brute force: To protect against brute force attacks, the number of failed authentication attempts in a given time period is limited.<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/About-the-APIs.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).<br /> # Authentication Schemes ### Access Key/Secret Key The main way to authenticate your requests to the OUTSCALE API is to use an access key and a secret key.<br /> The mechanism behind this is based on AWS Signature Version 4, whose technical implementation details are described in [Signature of API Requests](https://docs.outscale.com/en/userguide/Signature-of-API-Requests.html).<br /><br /> In practice, the way to specify your access key and secret key depends on the tool or SDK you want to use to interact with the API.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify your access key, secret key, and the Region of your account. > 2. You then specify the `--profile` option when executing OSC CLI commands. >  > For more information, see [Installing and Configuring OSC CLI](https://docs.outscale.com/en/userguide/Installing-and-Configuring-OSC-CLI.html).  See the code samples in each section of this documentation for specific examples in different programming languages.<br /> For more information about access keys, see [About Access Keys](https://docs.outscale.com/en/userguide/About-Access-Keys.html).  > If you try to sign requests with an invalid access key four times in a row, further authentication attempts will be prevented for 1 minute. This lockout time increases 1 minute every four failed attempts, for up to 10 minutes.  ### Login/Password For certain API actions, you can also use basic authentication with the login (email address) and password of your TINA account.<br /> This is useful only in special circumstances, for example if you do not know your access key/secret key and want to retrieve them programmatically.<br /> In most cases, however, you can use the Cockpit web interface to retrieve them.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify the Region of your account, but you leave the access key value and secret key value empty (`&quot;&quot;`). > 2. You then specify the `--profile`, `--authentication-method`, `--login`, and `--password` options when executing OSC CLI commands.  See the code samples in each section of this documentation for specific examples in different programming languages.  > If you try to sign requests with an invalid password four times in a row, further authentication attempts will be prevented for 1 minute. This lockout time increases 1 minute every four failed attempts, for up to 10 minutes.  ### No Authentication A few API actions do not require any authentication. They are indicated as such in this documentation.<br /> ### Other Security Mechanisms In parallel with the authentication schemes, you can add other security mechanisms to your OUTSCALE account, for example to restrict API requests by IP or other criteria.<br /> For more information, see [Managing Your API Accesses](https://docs.outscale.com/en/userguide/Managing-Your-API-Accesses.html).<br /> # Error Codes Reference You can learn more about errors returned by the API in the dedicated [errors page](api-errors.html).
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersVmsState : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersVmsState {
    /// The code for the scheduled event (`system-reboot` \\| `system-maintenance`).
    #[serde(
        rename = "MaintenanceEventCodes",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_event_codes: Option<Vec<String>>,
    /// The description of the scheduled event.
    #[serde(
        rename = "MaintenanceEventDescriptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_event_descriptions: Option<Vec<String>>,
    /// The latest date and time (UTC) the event can end.
    #[serde(
        rename = "MaintenanceEventsNotAfter",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_events_not_after: Option<Vec<String>>,
    /// The earliest date and time (UTC) the event can start.
    #[serde(
        rename = "MaintenanceEventsNotBefore",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_events_not_before: Option<Vec<String>>,
    /// The names of the Subregions of the VMs.
    #[serde(rename = "SubregionNames", skip_serializing_if = "Option::is_none")]
    pub subregion_names: Option<Vec<String>>,
    /// One or more IDs of VMs.
    #[serde(rename = "VmIds", skip_serializing_if = "Option::is_none")]
    pub vm_ids: Option<Vec<String>>,
    /// The states of the VMs (`pending` \\| `running` \\| `stopping` \\| `stopped` \\| `shutting-down` \\| `terminated` \\| `quarantine`).
    #[serde(rename = "VmStates", skip_serializing_if = "Option::is_none")]
    pub vm_states: Option<Vec<String>>,
}

impl FiltersVmsState {
    /// One or more filters.
    pub fn new() -> FiltersVmsState {
        FiltersVmsState {
            maintenance_event_codes: None,
            maintenance_event_descriptions: None,
            maintenance_events_not_after: None,
            maintenance_events_not_before: None,
            subregion_names: None,
            vm_ids: None,
            vm_states: None,
        }
    }
}
