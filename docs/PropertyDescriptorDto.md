# PropertyDescriptorDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name for the property. | [optional]
**display_name** | Option<**String**> | The human readable name for the property. | [optional]
**description** | Option<**String**> | The description for the property. Used to relay additional details to a user or provide a mechanism of documenting intent. | [optional]
**default_value** | Option<**String**> | The default value for the property. | [optional]
**allowable_values** | Option<[**Vec<crate::models::AllowableValueEntity>**](AllowableValueEntity.md)> | Allowable values for the property. If empty then the allowed values are not constrained. | [optional]
**required** | Option<**bool**> | Whether the property is required. | [optional]
**sensitive** | Option<**bool**> | Whether the property is sensitive and protected whenever stored or represented. | [optional]
**dynamic** | Option<**bool**> | Whether the property is dynamic (user-defined). | [optional]
**supports_el** | Option<**bool**> | Whether the property supports expression language. | [optional]
**expression_language_scope** | Option<**String**> | Scope of the Expression Language evaluation for the property. | [optional]
**identifies_controller_service** | Option<**String**> | If the property identifies a controller service this returns the fully qualified type. | [optional]
**identifies_controller_service_bundle** | Option<[**crate::models::BundleDto**](BundleDTO.md)> |  | [optional]
**dependencies** | Option<[**Vec<crate::models::PropertyDependencyDto>**](PropertyDependencyDTO.md)> | A list of dependencies that must be met in order for this Property to be relevant. If any of these dependencies is not met, the property described by this Property Descriptor is not relevant. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


