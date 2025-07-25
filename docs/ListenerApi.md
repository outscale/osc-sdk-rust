# \ListenerApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_listener_rule**](ListenerApi.md#create_listener_rule) | **POST** /CreateListenerRule | 
[**create_load_balancer_listeners**](ListenerApi.md#create_load_balancer_listeners) | **POST** /CreateLoadBalancerListeners | 
[**delete_listener_rule**](ListenerApi.md#delete_listener_rule) | **POST** /DeleteListenerRule | 
[**delete_load_balancer_listeners**](ListenerApi.md#delete_load_balancer_listeners) | **POST** /DeleteLoadBalancerListeners | 
[**read_listener_rules**](ListenerApi.md#read_listener_rules) | **POST** /ReadListenerRules | 
[**update_listener_rule**](ListenerApi.md#update_listener_rule) | **POST** /UpdateListenerRule | 



## create_listener_rule

> crate::models::CreateListenerRuleResponse create_listener_rule(create_listener_rule_request)


Creates a rule for traffic redirection for the specified listener. Each rule must have either the `HostNamePattern` or `PathPattern` parameter specified. Rules are treated in priority order, from the highest value to the lowest value.<br /> Once the rule is created, you need to register backend VMs with it. For more information, see the [RegisterVmsInLoadBalancer](#registervmsinloadbalancer) method.<br /><br /> For more information, see [About Load Balancers](https://docs.outscale.com/en/userguide/About-Load-Balancers.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_listener_rule_request** | Option<[**CreateListenerRuleRequest**](CreateListenerRuleRequest.md)> |  |  |

### Return type

[**crate::models::CreateListenerRuleResponse**](CreateListenerRuleResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_load_balancer_listeners

> crate::models::CreateLoadBalancerListenersResponse create_load_balancer_listeners(create_load_balancer_listeners_request)


Creates one or more listeners for a specified load balancer.<br /><br /> For more information, see [About Load Balancers](https://docs.outscale.com/en/userguide/About-Load-Balancers.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_load_balancer_listeners_request** | Option<[**CreateLoadBalancerListenersRequest**](CreateLoadBalancerListenersRequest.md)> |  |  |

### Return type

[**crate::models::CreateLoadBalancerListenersResponse**](CreateLoadBalancerListenersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_listener_rule

> crate::models::DeleteListenerRuleResponse delete_listener_rule(delete_listener_rule_request)


Deletes a listener rule.<br /> The previously active rule is disabled after deletion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_listener_rule_request** | Option<[**DeleteListenerRuleRequest**](DeleteListenerRuleRequest.md)> |  |  |

### Return type

[**crate::models::DeleteListenerRuleResponse**](DeleteListenerRuleResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_load_balancer_listeners

> crate::models::DeleteLoadBalancerListenersResponse delete_load_balancer_listeners(delete_load_balancer_listeners_request)


Deletes listeners of a specified load balancer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_load_balancer_listeners_request** | Option<[**DeleteLoadBalancerListenersRequest**](DeleteLoadBalancerListenersRequest.md)> |  |  |

### Return type

[**crate::models::DeleteLoadBalancerListenersResponse**](DeleteLoadBalancerListenersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_listener_rules

> crate::models::ReadListenerRulesResponse read_listener_rules(read_listener_rules_request)


Lists one or more listener rules. By default, this action returns the full list of listener rules for the account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_listener_rules_request** | Option<[**ReadListenerRulesRequest**](ReadListenerRulesRequest.md)> |  |  |

### Return type

[**crate::models::ReadListenerRulesResponse**](ReadListenerRulesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_listener_rule

> crate::models::UpdateListenerRuleResponse update_listener_rule(update_listener_rule_request)


Updates the pattern of the listener rule.<br /> This call updates the pattern matching algorithm for incoming traffic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_listener_rule_request** | Option<[**UpdateListenerRuleRequest**](UpdateListenerRuleRequest.md)> |  |  |

### Return type

[**crate::models::UpdateListenerRuleResponse**](UpdateListenerRuleResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

