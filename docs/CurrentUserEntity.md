# CurrentUserEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identity** | Option<**String**> | The user identity being serialized. | [optional]
**anonymous** | Option<**bool**> | Whether the current user is anonymous. | [optional]
**provenance_permissions** | Option<[**crate::models::PermissionsDto**](PermissionsDTO.md)> |  | [optional]
**counters_permissions** | Option<[**crate::models::PermissionsDto**](PermissionsDTO.md)> |  | [optional]
**tenants_permissions** | Option<[**crate::models::PermissionsDto**](PermissionsDTO.md)> |  | [optional]
**controller_permissions** | Option<[**crate::models::PermissionsDto**](PermissionsDTO.md)> |  | [optional]
**policies_permissions** | Option<[**crate::models::PermissionsDto**](PermissionsDTO.md)> |  | [optional]
**system_permissions** | Option<[**crate::models::PermissionsDto**](PermissionsDTO.md)> |  | [optional]
**parameter_context_permissions** | Option<[**crate::models::PermissionsDto**](PermissionsDTO.md)> |  | [optional]
**restricted_components_permissions** | Option<[**crate::models::PermissionsDto**](PermissionsDTO.md)> |  | [optional]
**component_restriction_permissions** | Option<[**Vec<crate::models::ComponentRestrictionPermissionDto>**](ComponentRestrictionPermissionDTO.md)> | Permissions for specific component restrictions. | [optional]
**can_version_flows** | Option<**bool**> | Whether the current user can version flows. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


