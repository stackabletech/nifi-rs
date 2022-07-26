# NodeDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_id** | Option<**String**> | The id of the node. | [optional]
**address** | Option<**String**> | The node's host/ip address. | [optional]
**api_port** | Option<**i32**> | The port the node is listening for API requests. | [optional]
**status** | Option<**String**> | The node's status. | [optional]
**heartbeat** | Option<**String**> | the time of the nodes's last heartbeat. | [optional]
**connection_requested** | Option<**String**> | The time of the node's last connection request. | [optional]
**roles** | Option<**Vec<String>**> | The roles of this node. | [optional]
**active_thread_count** | Option<**i32**> | The active threads for the NiFi on the node. | [optional]
**queued** | Option<**String**> | The queue the NiFi on the node. | [optional]
**events** | Option<[**Vec<crate::models::NodeEventDto>**](NodeEventDTO.md)> | The node's events. | [optional]
**node_start_time** | Option<**String**> | The time at which this Node was last refreshed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


