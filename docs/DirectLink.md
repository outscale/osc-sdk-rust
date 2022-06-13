# DirectLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The account ID of the owner of the DirectLink. | [optional]
**bandwidth** | Option<**String**> | The physical link bandwidth (either 1 Gbps or 10 Gbps). | [optional]
**direct_link_id** | Option<**String**> | The ID of the DirectLink (for example, `dxcon-xxxxxxxx`). | [optional]
**direct_link_name** | Option<**String**> | The name of the DirectLink. | [optional]
**location** | Option<**String**> | The datacenter where the DirectLink is located. | [optional]
**region_name** | Option<**String**> | The Region in which the DirectLink has been created. | [optional]
**state** | Option<**String**> | The state of the DirectLink.<br /> * `requested`: The DirectLink is requested but the request has not been validated yet.<br /> * `pending`: The DirectLink request has been validated. It remains in the `pending` state until you establish the physical link.<br /> * `available`: The physical link is established and the connection is ready to use.<br /> * `deleting`: The deletion process is in progress.<br /> * `deleted`: The DirectLink is deleted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


