# ParameterDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the Parameter | [optional]
**description** | Option<**String**> | The description of the Parameter | [optional]
**sensitive** | Option<**bool**> | Whether or not the Parameter is sensitive | [optional]
**value** | Option<**String**> | The value of the Parameter | [optional]
**value_removed** | Option<**bool**> | Whether or not the value of the Parameter was removed. When a request is made to change a parameter, the value may be null. The absence of the value may be used either to indicate that the value is not to be changed, or that the value is to be set to null (i.e., removed). This denotes which of the two scenarios is being encountered. | [optional]
**referencing_components** | Option<[**Vec<crate::models::AffectedComponentEntity>**](AffectedComponentEntity.md)> | The set of all components in the flow that are referencing this Parameter | [optional]
**parameter_context** | Option<[**crate::models::ParameterContextReferenceEntity**](ParameterContextReferenceEntity.md)> |  | [optional]
**inherited** | Option<**bool**> | Whether or not the Parameter is inherited from another context | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


