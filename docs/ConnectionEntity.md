# ConnectionEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**revision** | Option<[**crate::models::RevisionDto**](RevisionDTO.md)> |  | [optional]
**id** | Option<**String**> | The id of the component. | [optional]
**uri** | Option<**String**> | The URI for futures requests to the component. | [optional]
**position** | Option<[**crate::models::PositionDto**](PositionDTO.md)> |  | [optional]
**permissions** | Option<[**crate::models::PermissionsDto**](PermissionsDTO.md)> |  | [optional]
**bulletins** | Option<[**Vec<crate::models::BulletinEntity>**](BulletinEntity.md)> | The bulletins for this component. | [optional]
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. | [optional]
**component** | Option<[**crate::models::ConnectionDto**](ConnectionDTO.md)> |  | [optional]
**status** | Option<[**crate::models::ConnectionStatusDto**](ConnectionStatusDTO.md)> |  | [optional]
**bends** | Option<[**Vec<crate::models::PositionDto>**](PositionDTO.md)> | The bend points on the connection. | [optional]
**label_index** | Option<**i32**> | The index of the bend point where to place the connection label. | [optional]
**getz_index** | Option<**i64**> | The z index of the connection. | [optional]
**source_id** | Option<**String**> | The identifier of the source of this connection. | [optional]
**source_group_id** | Option<**String**> | The identifier of the group of the source of this connection. | [optional]
**source_type** | **String** | The type of component the source connectable is. | 
**destination_id** | Option<**String**> | The identifier of the destination of this connection. | [optional]
**destination_group_id** | Option<**String**> | The identifier of the group of the destination of this connection. | [optional]
**destination_type** | **String** | The type of component the destination connectable is. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


