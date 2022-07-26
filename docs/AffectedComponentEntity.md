# AffectedComponentEntity

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
**component** | Option<[**crate::models::AffectedComponentDto**](AffectedComponentDTO.md)> |  | [optional]
**process_group** | Option<[**crate::models::ProcessGroupNameDto**](ProcessGroupNameDTO.md)> |  | [optional]
**reference_type** | Option<**String**> | The type of component referenced | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


