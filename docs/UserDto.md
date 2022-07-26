# UserDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the component. | [optional]
**versioned_component_id** | Option<**String**> | The ID of the corresponding component that is under version control | [optional]
**parent_group_id** | Option<**String**> | The id of parent process group of this component if applicable. | [optional]
**position** | Option<[**crate::models::PositionDto**](PositionDTO.md)> |  | [optional]
**identity** | Option<**String**> | The identity of the tenant. | [optional]
**configurable** | Option<**bool**> | Whether this tenant is configurable. | [optional]
**user_groups** | Option<[**Vec<crate::models::TenantEntity>**](TenantEntity.md)> | The groups to which the user belongs. This field is read only and it provided for convenience. | [optional]
**access_policies** | Option<[**Vec<crate::models::AccessPolicySummaryEntity>**](AccessPolicySummaryEntity.md)> | The access policies this user belongs to. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


