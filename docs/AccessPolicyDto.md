# AccessPolicyDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the component. | [optional]
**versioned_component_id** | Option<**String**> | The ID of the corresponding component that is under version control | [optional]
**parent_group_id** | Option<**String**> | The id of parent process group of this component if applicable. | [optional]
**position** | Option<[**crate::models::PositionDto**](PositionDTO.md)> |  | [optional]
**resource** | Option<**String**> | The resource for this access policy. | [optional]
**action** | Option<**String**> | The action associated with this access policy. | [optional]
**component_reference** | Option<[**crate::models::ComponentReferenceEntity**](ComponentReferenceEntity.md)> |  | [optional]
**configurable** | Option<**bool**> | Whether this policy is configurable. | [optional]
**users** | Option<[**Vec<crate::models::TenantEntity>**](TenantEntity.md)> | The set of user IDs associated with this access policy. | [optional]
**user_groups** | Option<[**Vec<crate::models::TenantEntity>**](TenantEntity.md)> | The set of user group IDs associated with this access policy. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


