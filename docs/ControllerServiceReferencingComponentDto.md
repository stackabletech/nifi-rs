# ControllerServiceReferencingComponentDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group_id** | Option<**String**> | The group id for the component referencing a controller service. If this component is another controller service or a reporting task, this field is blank. | [optional]
**id** | Option<**String**> | The id of the component referencing a controller service. | [optional]
**name** | Option<**String**> | The name of the component referencing a controller service. | [optional]
**_type** | Option<**String**> | The type of the component referencing a controller service in simple Java class name format without package name. | [optional]
**state** | Option<**String**> | The scheduled state of a processor or reporting task referencing a controller service. If this component is another controller service, this field represents the controller service state. | [optional]
**properties** | Option<**::std::collections::HashMap<String, String>**> | The properties for the component. | [optional]
**descriptors** | Option<[**::std::collections::HashMap<String, crate::models::PropertyDescriptorDto>**](PropertyDescriptorDTO.md)> | The descriptors for the component properties. | [optional]
**validation_errors** | Option<**Vec<String>**> | The validation errors for the component. | [optional]
**reference_type** | Option<**String**> | The type of reference this is. | [optional]
**active_thread_count** | Option<**i32**> | The number of active threads for the referencing component. | [optional]
**reference_cycle** | Option<**bool**> | If the referencing component represents a controller service, this indicates whether it has already been represented in this hierarchy. | [optional]
**referencing_components** | Option<[**Vec<crate::models::ControllerServiceReferencingComponentEntity>**](ControllerServiceReferencingComponentEntity.md)> | If the referencing component represents a controller service, these are the components that reference it. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


