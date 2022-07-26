# ConnectableDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The id of the connectable component. | 
**versioned_component_id** | Option<**String**> | The ID of the corresponding component that is under version control | [optional]
**_type** | **String** | The type of component the connectable is. | 
**group_id** | **String** | The id of the group that the connectable component resides in | 
**name** | Option<**String**> | The name of the connectable component | [optional]
**running** | Option<**bool**> | Reflects the current state of the connectable component. | [optional]
**transmitting** | Option<**bool**> | If the connectable component represents a remote port, indicates if the target is configured to transmit. | [optional]
**exists** | Option<**bool**> | If the connectable component represents a remote port, indicates if the target exists. | [optional]
**comments** | Option<**String**> | The comments for the connectable component. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


