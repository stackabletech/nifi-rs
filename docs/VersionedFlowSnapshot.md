# VersionedFlowSnapshot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**snapshot_metadata** | [**crate::models::VersionedFlowSnapshotMetadata**](VersionedFlowSnapshotMetadata.md) |  | 
**flow_contents** | [**crate::models::VersionedProcessGroup**](VersionedProcessGroup.md) |  | 
**external_controller_services** | Option<[**::std::collections::HashMap<String, crate::models::ExternalControllerServiceReference>**](ExternalControllerServiceReference.md)> | The information about controller services that exist outside this versioned flow, but are referenced by components within the versioned flow. | [optional]
**parameter_contexts** | Option<[**::std::collections::HashMap<String, crate::models::VersionedParameterContext>**](VersionedParameterContext.md)> | The parameter contexts referenced by process groups in the flow contents. The mapping is from the name of the context to the context instance, and it is expected that any context in this map is referenced by at least one process group in this flow. | [optional]
**flow_encoding_version** | Option<**String**> | The optional encoding version of the flow contents. | [optional]
**flow** | Option<[**crate::models::VersionedFlow**](VersionedFlow.md)> |  | [optional]
**bucket** | Option<[**crate::models::Bucket**](Bucket.md)> |  | [optional]
**latest** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


