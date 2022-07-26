# JvmSystemDiagnosticsSnapshotDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**flow_file_repository_storage_usage** | Option<[**crate::models::RepositoryUsageDto**](RepositoryUsageDTO.md)> |  | [optional]
**content_repository_storage_usage** | Option<[**Vec<crate::models::RepositoryUsageDto>**](RepositoryUsageDTO.md)> | Information about the Content Repository's usage | [optional]
**provenance_repository_storage_usage** | Option<[**Vec<crate::models::RepositoryUsageDto>**](RepositoryUsageDTO.md)> | Information about the Provenance Repository's usage | [optional]
**max_heap_bytes** | Option<**i64**> | The maximum number of bytes that the JVM heap is configured to use for heap | [optional]
**max_heap** | Option<**String**> | The maximum number of bytes that the JVM heap is configured to use, as a human-readable value | [optional]
**garbage_collection_diagnostics** | Option<[**Vec<crate::models::GarbageCollectionDiagnosticsDto>**](GarbageCollectionDiagnosticsDTO.md)> | Diagnostic information about the JVM's garbage collections | [optional]
**cpu_cores** | Option<**i32**> | The number of CPU Cores available on the system | [optional]
**cpu_load_average** | Option<**f64**> | The 1-minute CPU Load Average | [optional]
**physical_memory_bytes** | Option<**i64**> | The number of bytes of RAM available on the system | [optional]
**physical_memory** | Option<**String**> | The number of bytes of RAM available on the system as a human-readable value | [optional]
**open_file_descriptors** | Option<**i64**> | The number of files that are open by the NiFi process | [optional]
**max_open_file_descriptors** | Option<**i64**> | The maximum number of open file descriptors that are available to each process | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


