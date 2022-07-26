# ParameterContextDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The Name of the Parameter Context. | [optional]
**description** | Option<**String**> | The Description of the Parameter Context. | [optional]
**parameters** | Option<[**Vec<crate::models::ParameterEntity>**](ParameterEntity.md)> | The Parameters for the Parameter Context | [optional]
**bound_process_groups** | Option<[**Vec<crate::models::ProcessGroupEntity>**](ProcessGroupEntity.md)> | The Process Groups that are bound to this Parameter Context | [optional]
**inherited_parameter_contexts** | Option<[**Vec<crate::models::ParameterContextReferenceEntity>**](ParameterContextReferenceEntity.md)> | A list of references of Parameter Contexts from which this one inherits parameters | [optional]
**id** | Option<**String**> | The ID the Parameter Context. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


