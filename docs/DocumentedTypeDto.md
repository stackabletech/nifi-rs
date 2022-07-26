# DocumentedTypeDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | Option<**String**> | The fully qualified name of the type. | [optional]
**bundle** | Option<[**crate::models::BundleDto**](BundleDTO.md)> |  | [optional]
**controller_service_apis** | Option<[**Vec<crate::models::ControllerServiceApiDto>**](ControllerServiceApiDTO.md)> | If this type represents a ControllerService, this lists the APIs it implements. | [optional]
**description** | Option<**String**> | The description of the type. | [optional]
**restricted** | Option<**bool**> | Whether this type is restricted. | [optional]
**usage_restriction** | Option<**String**> | The optional description of why the usage of this component is restricted. | [optional]
**explicit_restrictions** | Option<[**Vec<crate::models::ExplicitRestrictionDto>**](ExplicitRestrictionDTO.md)> | An optional collection of explicit restrictions. If specified, these explicit restrictions will be enfored. | [optional]
**deprecation_reason** | Option<**String**> | The description of why the usage of this component is restricted. | [optional]
**tags** | Option<**Vec<String>**> | The tags associated with this type. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


