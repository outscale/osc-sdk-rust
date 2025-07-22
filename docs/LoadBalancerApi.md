# \LoadBalancerApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_load_balancer**](LoadBalancerApi.md#create_load_balancer) | **POST** /CreateLoadBalancer | 
[**create_load_balancer_tags**](LoadBalancerApi.md#create_load_balancer_tags) | **POST** /CreateLoadBalancerTags | 
[**delete_load_balancer**](LoadBalancerApi.md#delete_load_balancer) | **POST** /DeleteLoadBalancer | 
[**delete_load_balancer_tags**](LoadBalancerApi.md#delete_load_balancer_tags) | **POST** /DeleteLoadBalancerTags | 
[**deregister_vms_in_load_balancer**](LoadBalancerApi.md#deregister_vms_in_load_balancer) | **POST** /DeregisterVmsInLoadBalancer | 
[**link_load_balancer_backend_machines**](LoadBalancerApi.md#link_load_balancer_backend_machines) | **POST** /LinkLoadBalancerBackendMachines | 
[**read_load_balancer_tags**](LoadBalancerApi.md#read_load_balancer_tags) | **POST** /ReadLoadBalancerTags | 
[**read_load_balancers**](LoadBalancerApi.md#read_load_balancers) | **POST** /ReadLoadBalancers | 
[**read_vms_health**](LoadBalancerApi.md#read_vms_health) | **POST** /ReadVmsHealth | 
[**register_vms_in_load_balancer**](LoadBalancerApi.md#register_vms_in_load_balancer) | **POST** /RegisterVmsInLoadBalancer | 
[**unlink_load_balancer_backend_machines**](LoadBalancerApi.md#unlink_load_balancer_backend_machines) | **POST** /UnlinkLoadBalancerBackendMachines | 
[**update_load_balancer**](LoadBalancerApi.md#update_load_balancer) | **POST** /UpdateLoadBalancer | 



## create_load_balancer

> crate::models::CreateLoadBalancerResponse create_load_balancer(create_load_balancer_request)


Creates a load balancer.<br /> The load balancer is created with a unique Domain Name Service (DNS) name. It receives the incoming traffic and routes it to its registered virtual machines (VMs).<br /> By default, this action creates an Internet-facing load balancer, resolving to public IPs. To create an internal load balancer in a Net, resolving to private IPs, use the `LoadBalancerType` parameter.<br /> You must specify either the `Subnets` or the `SubregionNames` parameters.<br /><br /> For more information, see [About Load Balancers](https://docs.outscale.com/en/userguide/About-Load-Balancers.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_load_balancer_request** | Option<[**CreateLoadBalancerRequest**](CreateLoadBalancerRequest.md)> |  |  |

### Return type

[**crate::models::CreateLoadBalancerResponse**](CreateLoadBalancerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_load_balancer_tags

> crate::models::CreateLoadBalancerTagsResponse create_load_balancer_tags(create_load_balancer_tags_request)


Adds one or more tags to the specified load balancers.<br /> If a tag with the same key already exists for the load balancer, the tag value is replaced.<br /><br /> For more information, see [About Tags](https://docs.outscale.com/en/userguide/About-Tags.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_load_balancer_tags_request** | Option<[**CreateLoadBalancerTagsRequest**](CreateLoadBalancerTagsRequest.md)> |  |  |

### Return type

[**crate::models::CreateLoadBalancerTagsResponse**](CreateLoadBalancerTagsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_load_balancer

> crate::models::DeleteLoadBalancerResponse delete_load_balancer(delete_load_balancer_request)


Deletes a specified load balancer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_load_balancer_request** | Option<[**DeleteLoadBalancerRequest**](DeleteLoadBalancerRequest.md)> |  |  |

### Return type

[**crate::models::DeleteLoadBalancerResponse**](DeleteLoadBalancerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_load_balancer_tags

> crate::models::DeleteLoadBalancerTagsResponse delete_load_balancer_tags(delete_load_balancer_tags_request)


Deletes one or more tags from the specified load balancers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_load_balancer_tags_request** | Option<[**DeleteLoadBalancerTagsRequest**](DeleteLoadBalancerTagsRequest.md)> |  |  |

### Return type

[**crate::models::DeleteLoadBalancerTagsResponse**](DeleteLoadBalancerTagsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deregister_vms_in_load_balancer

> crate::models::DeregisterVmsInLoadBalancerResponse deregister_vms_in_load_balancer(deregister_vms_in_load_balancer_request)


> [WARNING]<br /> > Deprecated: This call is deprecated and will be removed.<br />  Deregisters a specified virtual machine (VM) from a load balancer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deregister_vms_in_load_balancer_request** | Option<[**DeregisterVmsInLoadBalancerRequest**](DeregisterVmsInLoadBalancerRequest.md)> |  |  |

### Return type

[**crate::models::DeregisterVmsInLoadBalancerResponse**](DeregisterVmsInLoadBalancerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_load_balancer_backend_machines

> crate::models::LinkLoadBalancerBackendMachinesResponse link_load_balancer_backend_machines(link_load_balancer_backend_machines_request)


Attaches one or more virtual machines (VMs) to a specified load balancer. You need to specify at least the `BackendIps` or the `BackendVmIds` parameter.<br /> The VMs can be in different Subnets and different Subregions than the load balancer, as long as the VMs and load balancers are all in the public Cloud or all in the same Net. It may take a little time for a VM to be registered with the load balancer. Once the VM is registered with a load balancer, it receives traffic and requests from this load balancer and is called a backend VM.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_load_balancer_backend_machines_request** | Option<[**LinkLoadBalancerBackendMachinesRequest**](LinkLoadBalancerBackendMachinesRequest.md)> |  |  |

### Return type

[**crate::models::LinkLoadBalancerBackendMachinesResponse**](LinkLoadBalancerBackendMachinesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_load_balancer_tags

> crate::models::ReadLoadBalancerTagsResponse read_load_balancer_tags(read_load_balancer_tags_request)


Lists the tags associated with one or more specified load balancers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_load_balancer_tags_request** | Option<[**ReadLoadBalancerTagsRequest**](ReadLoadBalancerTagsRequest.md)> |  |  |

### Return type

[**crate::models::ReadLoadBalancerTagsResponse**](ReadLoadBalancerTagsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_load_balancers

> crate::models::ReadLoadBalancersResponse read_load_balancers(read_load_balancers_request)


Lists one or more load balancers and their attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_load_balancers_request** | Option<[**ReadLoadBalancersRequest**](ReadLoadBalancersRequest.md)> |  |  |

### Return type

[**crate::models::ReadLoadBalancersResponse**](ReadLoadBalancersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_vms_health

> crate::models::ReadVmsHealthResponse read_vms_health(read_vms_health_request)


Lists the state of one or more backend virtual machines (VMs) registered with a specified load balancer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_vms_health_request** | Option<[**ReadVmsHealthRequest**](ReadVmsHealthRequest.md)> |  |  |

### Return type

[**crate::models::ReadVmsHealthResponse**](ReadVmsHealthResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_vms_in_load_balancer

> crate::models::RegisterVmsInLoadBalancerResponse register_vms_in_load_balancer(register_vms_in_load_balancer_request)


> [WARNING]<br /> > Deprecated: This call is deprecated and will be removed.<br />  Registers one or more virtual machines (VMs) with a specified load balancer.<br /> The VMs can be in different Subnets and different Subregions than the load balancer, as long as the VMs and load balancers are all in the public Cloud or all in the same Net. It may take a little time for a VM to be registered with the load balancer. Once the VM is registered with a load balancer, it receives traffic and requests from this load balancer and is called a backend VM.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_vms_in_load_balancer_request** | Option<[**RegisterVmsInLoadBalancerRequest**](RegisterVmsInLoadBalancerRequest.md)> |  |  |

### Return type

[**crate::models::RegisterVmsInLoadBalancerResponse**](RegisterVmsInLoadBalancerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_load_balancer_backend_machines

> crate::models::UnlinkLoadBalancerBackendMachinesResponse unlink_load_balancer_backend_machines(unlink_load_balancer_backend_machines_request)


Detaches one or more backend virtual machines (VMs) from a load balancer. You need to specify at least the `BackendIps` or the `BackendVmIds` parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unlink_load_balancer_backend_machines_request** | Option<[**UnlinkLoadBalancerBackendMachinesRequest**](UnlinkLoadBalancerBackendMachinesRequest.md)> |  |  |

### Return type

[**crate::models::UnlinkLoadBalancerBackendMachinesResponse**](UnlinkLoadBalancerBackendMachinesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_load_balancer

> crate::models::UpdateLoadBalancerResponse update_load_balancer(update_load_balancer_request)


Modifies the specified attribute of a load balancer. You can specify only one attribute at a time.<br /><br />  You can set a new SSL certificate to an SSL or HTTPS listener of a load balancer.<br /> This certificate replaces any certificate used on the same load balancer and port.<br /><br />  You can also replace the currently enabled policy for the load balancer with another one.<br /> If the `PolicyNames` parameter is empty, the currently enabled policy is disabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_load_balancer_request** | Option<[**UpdateLoadBalancerRequest**](UpdateLoadBalancerRequest.md)> |  |  |

### Return type

[**crate::models::UpdateLoadBalancerResponse**](UpdateLoadBalancerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

