# ProcessorEntity

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
**component** | Option<[**crate::models::ProcessorDto**](ProcessorDTO.md)> |  | [optional]
**input_requirement** | Option<**String**> | The input requirement for this processor. | [optional]
**status** | Option<[**crate::models::ProcessorStatusDto**](ProcessorStatusDTO.md)> |  | [optional]
**operate_permissions** | Option<[**crate::models::PermissionsDto**](PermissionsDTO.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


