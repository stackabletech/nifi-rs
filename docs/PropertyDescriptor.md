# PropertyDescriptor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the property key | 
**display_name** | Option<**String**> | The display name of the property key, if different from the name | [optional]
**description** | Option<**String**> | The description of what the property does | [optional]
**allowable_values** | Option<[**Vec<crate::models::PropertyAllowableValue>**](PropertyAllowableValue.md)> | A list of the allowable values for the property | [optional]
**default_value** | Option<**String**> | The default value if a user-set value is not specified | [optional]
**required** | Option<**bool**> | Whether or not  the property is required for the component | [optional]
**sensitive** | Option<**bool**> | Whether or not  the value of the property is considered sensitive (e.g., passwords and keys) | [optional]
**expression_language_scope** | Option<**String**> | The scope of expression language supported by this property | [optional]
**expression_language_scope_description** | Option<**String**> | The description of the expression language scope supported by this property | [optional][readonly]
**type_provided_by_value** | Option<[**crate::models::DefinedType**](DefinedType.md)> |  | [optional]
**valid_regex** | Option<**String**> | A regular expression that can be used to validate the value of this property | [optional]
**validator** | Option<**String**> | Name of the validator used for this property descriptor | [optional]
**dynamic** | Option<**bool**> | Whether or not the descriptor is for a dynamically added property | [optional]
**resource_definition** | Option<[**crate::models::PropertyResourceDefinition**](PropertyResourceDefinition.md)> |  | [optional]
**dependencies** | Option<[**Vec<crate::models::PropertyDependency>**](PropertyDependency.md)> | The dependencies that this property has on other properties | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


