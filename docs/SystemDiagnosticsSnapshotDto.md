# SystemDiagnosticsSnapshotDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_non_heap** | Option<**String**> | Total size of non heap. | [optional]
**total_non_heap_bytes** | Option<**i64**> | Total number of bytes allocated to the JVM not used for heap | [optional]
**used_non_heap** | Option<**String**> | Amount of use non heap. | [optional]
**used_non_heap_bytes** | Option<**i64**> | Total number of bytes used by the JVM not in the heap space | [optional]
**free_non_heap** | Option<**String**> | Amount of free non heap. | [optional]
**free_non_heap_bytes** | Option<**i64**> | Total number of free non-heap bytes available to the JVM | [optional]
**max_non_heap** | Option<**String**> | Maximum size of non heap. | [optional]
**max_non_heap_bytes** | Option<**i64**> | The maximum number of bytes that the JVM can use for non-heap purposes | [optional]
**non_heap_utilization** | Option<**String**> | Utilization of non heap. | [optional]
**total_heap** | Option<**String**> | Total size of heap. | [optional]
**total_heap_bytes** | Option<**i64**> | The total number of bytes that are available for the JVM heap to use | [optional]
**used_heap** | Option<**String**> | Amount of used heap. | [optional]
**used_heap_bytes** | Option<**i64**> | The number of bytes of JVM heap that are currently being used | [optional]
**free_heap** | Option<**String**> | Amount of free heap. | [optional]
**free_heap_bytes** | Option<**i64**> | The number of bytes that are allocated to the JVM heap but not currently being used | [optional]
**max_heap** | Option<**String**> | Maximum size of heap. | [optional]
**max_heap_bytes** | Option<**i64**> | The maximum number of bytes that can be used by the JVM | [optional]
**heap_utilization** | Option<**String**> | Utilization of heap. | [optional]
**available_processors** | Option<**i32**> | Number of available processors if supported by the underlying system. | [optional]
**processor_load_average** | Option<**f64**> | The processor load average if supported by the underlying system. | [optional]
**total_threads** | Option<**i32**> | Total number of threads. | [optional]
**daemon_threads** | Option<**i32**> | Number of daemon threads. | [optional]
**uptime** | Option<**String**> | The uptime of the Java virtual machine | [optional]
**flow_file_repository_storage_usage** | Option<[**crate::models::StorageUsageDto**](StorageUsageDTO.md)> |  | [optional]
**content_repository_storage_usage** | Option<[**Vec<crate::models::StorageUsageDto>**](StorageUsageDTO.md)> | The content repository storage usage. | [optional]
**provenance_repository_storage_usage** | Option<[**Vec<crate::models::StorageUsageDto>**](StorageUsageDTO.md)> | The provenance repository storage usage. | [optional]
**garbage_collection** | Option<[**Vec<crate::models::GarbageCollectionDto>**](GarbageCollectionDTO.md)> | The garbage collection details. | [optional]
**stats_last_refreshed** | Option<**String**> | When the diagnostics were generated. | [optional]
**version_info** | Option<[**crate::models::VersionInfoDto**](VersionInfoDTO.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


