/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> Throttling: To protect against overloads, the number of identical requests allowed in a given time period is limited.<br /> Brute force: To protect against brute force attacks, the number of failed authentication attempts in a given time period is limited.<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/About-the-APIs.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).<br /> # Authentication Schemes ### Access Key/Secret Key The main way to authenticate your requests to the OUTSCALE API is to use an access key and a secret key.<br /> The mechanism behind this is based on AWS Signature Version 4, whose technical implementation details are described in [Signature of API Requests](https://docs.outscale.com/en/userguide/Signature-of-API-Requests.html).<br /><br /> In practice, the way to specify your access key and secret key depends on the tool or SDK you want to use to interact with the API.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify your access key, secret key, and the Region of your account. > 2. You then specify the `--profile` option when executing OSC CLI commands. >  > For more information, see [Installing and Configuring OSC CLI](https://docs.outscale.com/en/userguide/Installing-and-Configuring-OSC-CLI.html).  See the code samples in each section of this documentation for specific examples in different programming languages.<br /> For more information about access keys, see [About Access Keys](https://docs.outscale.com/en/userguide/About-Access-Keys.html).  > If you try to sign requests with an invalid access key four times in a row, further authentication attempts will be prevented for 1 minute. This lockout time increases 1 minute every four failed attempts, for up to 10 minutes.  ### Login/Password For certain API actions, you can also use basic authentication with the login (email address) and password of your TINA account.<br /> This is useful only in special circumstances, for example if you do not know your access key/secret key and want to retrieve them programmatically.<br /> In most cases, however, you can use the Cockpit web interface to retrieve them.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify the Region of your account, but you leave the access key value and secret key value empty (`&quot;&quot;`). > 2. You then specify the `--profile`, `--authentication-method`, `--login`, and `--password` options when executing OSC CLI commands.  See the code samples in each section of this documentation for specific examples in different programming languages.  > If you try to sign requests with an invalid password four times in a row, further authentication attempts will be prevented for 1 minute. This lockout time increases 1 minute every four failed attempts, for up to 10 minutes.  ### No Authentication A few API actions do not require any authentication. They are indicated as such in this documentation.<br /> ### Other Security Mechanisms In parallel with the authentication schemes, you can add other security mechanisms to your OUTSCALE account, for example to restrict API requests by IP or other criteria.<br /> For more information, see [Managing Your API Accesses](https://docs.outscale.com/en/userguide/Managing-Your-API-Accesses.html). # Pagination Tutorial You can learn more about the pagination methods for read calls in the dedicated [pagination tutorial](https://docs.outscale.com/en/userguide/Tutorial-Paginating-an-API-Request.html). # Error Codes Reference You can learn more about errors returned by the API in the dedicated [errors page](api-errors.html).
 *
 * The version of the OpenAPI document: 1.33.1
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersRouteTable : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersRouteTable {
    /// The IDs of the route tables involved in the associations.
    #[serde(rename = "LinkRouteTableIds", skip_serializing_if = "Option::is_none")]
    pub link_route_table_ids: Option<Vec<String>>,
    /// The IDs of the associations between the route tables and the Subnets.
    #[serde(
        rename = "LinkRouteTableLinkRouteTableIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub link_route_table_link_route_table_ids: Option<Vec<String>>,
    /// If true, the route tables are the main ones for their Nets.
    #[serde(rename = "LinkRouteTableMain", skip_serializing_if = "Option::is_none")]
    pub link_route_table_main: Option<bool>,
    /// The IDs of the Subnets involved in the associations.
    #[serde(rename = "LinkSubnetIds", skip_serializing_if = "Option::is_none")]
    pub link_subnet_ids: Option<Vec<String>>,
    /// The IDs of the Nets for the route tables.
    #[serde(rename = "NetIds", skip_serializing_if = "Option::is_none")]
    pub net_ids: Option<Vec<String>>,
    /// The methods used to create a route.
    #[serde(
        rename = "RouteCreationMethods",
        skip_serializing_if = "Option::is_none"
    )]
    pub route_creation_methods: Option<Vec<String>>,
    /// The IP ranges specified in routes in the tables.
    #[serde(
        rename = "RouteDestinationIpRanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub route_destination_ip_ranges: Option<Vec<String>>,
    /// The service IDs specified in routes in the tables.
    #[serde(
        rename = "RouteDestinationServiceIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub route_destination_service_ids: Option<Vec<String>>,
    /// The IDs of the gateways specified in routes in the tables.
    #[serde(rename = "RouteGatewayIds", skip_serializing_if = "Option::is_none")]
    pub route_gateway_ids: Option<Vec<String>>,
    /// The IDs of the NAT services specified in routes in the tables.
    #[serde(rename = "RouteNatServiceIds", skip_serializing_if = "Option::is_none")]
    pub route_nat_service_ids: Option<Vec<String>>,
    /// The IDs of the Net peerings specified in routes in the tables.
    #[serde(rename = "RouteNetPeeringIds", skip_serializing_if = "Option::is_none")]
    pub route_net_peering_ids: Option<Vec<String>>,
    /// The states of routes in the route tables (always `active`).
    #[serde(rename = "RouteStates", skip_serializing_if = "Option::is_none")]
    pub route_states: Option<Vec<String>>,
    /// The IDs of the route tables.
    #[serde(rename = "RouteTableIds", skip_serializing_if = "Option::is_none")]
    pub route_table_ids: Option<Vec<String>>,
    /// The IDs of the VMs specified in routes in the tables.
    #[serde(rename = "RouteVmIds", skip_serializing_if = "Option::is_none")]
    pub route_vm_ids: Option<Vec<String>>,
    /// The keys of the tags associated with the route tables.
    #[serde(rename = "TagKeys", skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// The values of the tags associated with the route tables.
    #[serde(rename = "TagValues", skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
    /// The key/value combination of the tags associated with the route tables, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl FiltersRouteTable {
    /// One or more filters.
    pub fn new() -> FiltersRouteTable {
        FiltersRouteTable {
            link_route_table_ids: None,
            link_route_table_link_route_table_ids: None,
            link_route_table_main: None,
            link_subnet_ids: None,
            net_ids: None,
            route_creation_methods: None,
            route_destination_ip_ranges: None,
            route_destination_service_ids: None,
            route_gateway_ids: None,
            route_nat_service_ids: None,
            route_net_peering_ids: None,
            route_states: None,
            route_table_ids: None,
            route_vm_ids: None,
            tag_keys: None,
            tag_values: None,
            tags: None,
        }
    }
}
