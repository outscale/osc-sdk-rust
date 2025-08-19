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
**state** | Option<**String**> | The state of the DirectLink. <ul><li>`pending`: The DirectLink request has been validated. It remains in the `pending` state until you establish the physical link.</li> <li>`available`: The physical link is established and the connection is ready to use.</li> <li>`disabled`: The network link is down.</li> <li>`deleted`: The DirectLink is deleted.</li> </ul>  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


