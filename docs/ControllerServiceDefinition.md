# ControllerServiceDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group** | Option<**String**> | The group name of the bundle that provides the referenced type. | [optional]
**artifact** | Option<**String**> | The artifact name of the bundle that provides the referenced type. | [optional]
**version** | Option<**String**> | The version of the bundle that provides the referenced type. | [optional]
**_type** | **String** | The fully-qualified class type | 
**type_description** | Option<**String**> | The description of the type. | [optional]
**build_info** | Option<[**crate::models::BuildInfo**](BuildInfo.md)> |  | [optional]
**provided_api_implementations** | Option<[**Vec<crate::models::DefinedType>**](DefinedType.md)> | If this type represents a provider for an interface, this lists the APIs it implements | [optional]
**tags** | Option<**Vec<String>**> | The tags associated with this type | [optional]
**deprecated** | Option<**bool**> | Whether or not the component has been deprecated | [optional]
**deprecation_reason** | Option<**String**> | If this component has been deprecated, this optional field can be used to provide an explanation | [optional]
**property_descriptors** | Option<[**::std::collections::HashMap<String, crate::models::PropertyDescriptor>**](PropertyDescriptor.md)> | Descriptions of configuration properties applicable to this component. | [optional]
**supports_dynamic_properties** | Option<**bool**> | Whether or not this component makes use of dynamic (user-set) properties. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


