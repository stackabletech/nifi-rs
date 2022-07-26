# JvmDiagnosticsDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**clustered** | Option<**bool**> | Whether or not the NiFi instance is clustered | [optional]
**connected** | Option<**bool**> | Whether or not the node is connected to the cluster | [optional]
**aggregate_snapshot** | Option<[**crate::models::JvmDiagnosticsSnapshotDto**](JVMDiagnosticsSnapshotDTO.md)> |  | [optional]
**node_snapshots** | Option<[**Vec<crate::models::NodeJvmDiagnosticsSnapshotDto>**](NodeJVMDiagnosticsSnapshotDTO.md)> | Node-wise breakdown of JVM diagnostic information | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


