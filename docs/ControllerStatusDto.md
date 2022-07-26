# ControllerStatusDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_thread_count** | Option<**i32**> | The number of active threads in the NiFi. | [optional]
**terminated_thread_count** | Option<**i32**> | The number of terminated threads in the NiFi. | [optional]
**queued** | Option<**String**> | The number of flowfiles queued in the NiFi. | [optional]
**flow_files_queued** | Option<**i32**> | The number of FlowFiles queued across the entire flow | [optional]
**bytes_queued** | Option<**i64**> | The size of the FlowFiles queued across the entire flow | [optional]
**running_count** | Option<**i32**> | The number of running components in the NiFi. | [optional]
**stopped_count** | Option<**i32**> | The number of stopped components in the NiFi. | [optional]
**invalid_count** | Option<**i32**> | The number of invalid components in the NiFi. | [optional]
**disabled_count** | Option<**i32**> | The number of disabled components in the NiFi. | [optional]
**active_remote_port_count** | Option<**i32**> | The number of active remote ports in the NiFi. | [optional]
**inactive_remote_port_count** | Option<**i32**> | The number of inactive remote ports in the NiFi. | [optional]
**up_to_date_count** | Option<**i32**> | The number of up to date versioned process groups in the NiFi. | [optional]
**locally_modified_count** | Option<**i32**> | The number of locally modified versioned process groups in the NiFi. | [optional]
**stale_count** | Option<**i32**> | The number of stale versioned process groups in the NiFi. | [optional]
**locally_modified_and_stale_count** | Option<**i32**> | The number of locally modified and stale versioned process groups in the NiFi. | [optional]
**sync_failure_count** | Option<**i32**> | The number of versioned process groups in the NiFi that are unable to sync to a registry. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


