# ControllerServiceDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the component. | [optional]
**versioned_component_id** | Option<**String**> | The ID of the corresponding component that is under version control | [optional]
**parent_group_id** | Option<**String**> | The id of parent process group of this component if applicable. | [optional]
**position** | Option<[**crate::models::PositionDto**](PositionDTO.md)> |  | [optional]
**name** | Option<**String**> | The name of the controller service. | [optional]
**_type** | Option<**String**> | The type of the controller service. | [optional]
**bundle** | Option<[**crate::models::BundleDto**](BundleDTO.md)> |  | [optional]
**controller_service_apis** | Option<[**Vec<crate::models::ControllerServiceApiDto>**](ControllerServiceApiDTO.md)> | Lists the APIs this Controller Service implements. | [optional]
**comments** | Option<**String**> | The comments for the controller service. | [optional]
**state** | Option<**String**> | The state of the controller service. | [optional]
**persists_state** | Option<**bool**> | Whether the controller service persists state. | [optional]
**restricted** | Option<**bool**> | Whether the controller service requires elevated privileges. | [optional]
**deprecated** | Option<**bool**> | Whether the ontroller service has been deprecated. | [optional]
**multiple_versions_available** | Option<**bool**> | Whether the controller service has multiple versions available. | [optional]
**properties** | Option<**::std::collections::HashMap<String, String>**> | The properties of the controller service. | [optional]
**descriptors** | Option<[**::std::collections::HashMap<String, crate::models::PropertyDescriptorDto>**](PropertyDescriptorDTO.md)> | The descriptors for the controller service properties. | [optional]
**custom_ui_url** | Option<**String**> | The URL for the controller services custom configuration UI if applicable. | [optional]
**annotation_data** | Option<**String**> | The annotation for the controller service. This is how the custom UI relays configuration to the controller service. | [optional]
**referencing_components** | Option<[**Vec<crate::models::ControllerServiceReferencingComponentEntity>**](ControllerServiceReferencingComponentEntity.md)> | All components referencing this controller service. | [optional]
**validation_errors** | Option<**Vec<String>**> | The validation errors from the controller service. These validation errors represent the problems with the controller service that must be resolved before it can be enabled. | [optional]
**validation_status** | Option<**String**> | Indicates whether the ControllerService is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the ControllerService is valid) | [optional]
**extension_missing** | Option<**bool**> | Whether the underlying extension is missing. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


