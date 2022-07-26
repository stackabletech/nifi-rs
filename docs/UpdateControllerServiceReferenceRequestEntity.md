# UpdateControllerServiceReferenceRequestEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The identifier of the Controller Service. | [optional]
**state** | Option<**String**> | The new state of the references for the controller service. | [optional]
**referencing_component_revisions** | Option<[**::std::collections::HashMap<String, crate::models::RevisionDto>**](RevisionDTO.md)> | The revisions for all referencing components. | [optional]
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. | [optional]
**ui_only** | Option<**bool**> | Indicates whether or not the response should only include fields necessary for rendering the NiFi User Interface. As such, when this value is set to true, some fields may be returned as null values, and the selected fields may change at any time without notice. As a result, this value should not be set to true by any client other than the UI. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


