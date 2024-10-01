/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> Throttling: To protect against overloads, the number of identical requests allowed in a given time period is limited.<br /> Brute force: To protect against brute force attacks, the number of failed authentication attempts in a given time period is limited.<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/About-the-APIs.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).<br /> # Authentication Schemes ### Access Key/Secret Key The main way to authenticate your requests to the OUTSCALE API is to use an access key and a secret key.<br /> The mechanism behind this is based on AWS Signature Version 4, whose technical implementation details are described in [Signature of API Requests](https://docs.outscale.com/en/userguide/Signature-of-API-Requests.html).<br /><br /> In practice, the way to specify your access key and secret key depends on the tool or SDK you want to use to interact with the API.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify your access key, secret key, and the Region of your account. > 2. You then specify the `--profile` option when executing OSC CLI commands. >  > For more information, see [Installing and Configuring OSC CLI](https://docs.outscale.com/en/userguide/Installing-and-Configuring-OSC-CLI.html).  See the code samples in each section of this documentation for specific examples in different programming languages.<br /> For more information about access keys, see [About Access Keys](https://docs.outscale.com/en/userguide/About-Access-Keys.html).  > If you try to sign requests with an invalid access key four times in a row, further authentication attempts will be prevented for 1 minute. This lockout time increases 1 minute every four failed attempts, for up to 10 minutes.  ### Login/Password For certain API actions, you can also use basic authentication with the login (email address) and password of your TINA account.<br /> This is useful only in special circumstances, for example if you do not know your access key/secret key and want to retrieve them programmatically.<br /> In most cases, however, you can use the Cockpit web interface to retrieve them.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify the Region of your account, but you leave the access key value and secret key value empty (`&quot;&quot;`). > 2. You then specify the `--profile`, `--authentication-method`, `--login`, and `--password` options when executing OSC CLI commands.  See the code samples in each section of this documentation for specific examples in different programming languages.  > If you try to sign requests with an invalid password four times in a row, further authentication attempts will be prevented for 1 minute. This lockout time increases 1 minute every four failed attempts, for up to 10 minutes.  ### No Authentication A few API actions do not require any authentication. They are indicated as such in this documentation.<br /> ### Other Security Mechanisms In parallel with the authentication schemes, you can add other security mechanisms to your OUTSCALE account, for example to restrict API requests by IP or other criteria.<br /> For more information, see [Managing Your API Accesses](https://docs.outscale.com/en/userguide/Managing-Your-API-Accesses.html). # Pagination Tutorial You can learn more about the pagination methods for read calls in the dedicated [pagination tutorial](https://docs.outscale.com/en/userguide/Tutorial-Paginating-an-API-Request.html). # Error Codes Reference You can learn more about errors returned by the API in the dedicated [errors page](api-errors.html).
 *
 * The version of the OpenAPI document: 1.33.1
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// HealthCheck : Information about the health check configuration.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HealthCheck {
    /// The number of seconds between two requests (between `5` and `600` both included).
    #[serde(rename = "CheckInterval")]
    pub check_interval: i32,
    /// The number of consecutive successful requests before considering the VM as healthy (between `2` and `10` both included).
    #[serde(rename = "HealthyThreshold")]
    pub healthy_threshold: i32,
    /// If you use the HTTP or HTTPS protocols, the request URL path.
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The port number (between `1` and `65535`, both included).
    #[serde(rename = "Port")]
    pub port: i32,
    /// The protocol for the URL of the VM (`HTTP` \\| `HTTPS` \\| `TCP` \\| `SSL`).
    #[serde(rename = "Protocol")]
    pub protocol: String,
    /// The maximum waiting time for a response before considering the VM as unhealthy, in seconds (between `2` and `60` both included).
    #[serde(rename = "Timeout")]
    pub timeout: i32,
    /// The number of consecutive failed requests before considering the VM as unhealthy (between `2` and `10` both included).
    #[serde(rename = "UnhealthyThreshold")]
    pub unhealthy_threshold: i32,
}

impl HealthCheck {
    /// Information about the health check configuration.
    pub fn new(
        check_interval: i32,
        healthy_threshold: i32,
        port: i32,
        protocol: String,
        timeout: i32,
        unhealthy_threshold: i32,
    ) -> HealthCheck {
        HealthCheck {
            check_interval,
            healthy_threshold,
            path: None,
            port,
            protocol,
            timeout,
            unhealthy_threshold,
        }
    }
}
