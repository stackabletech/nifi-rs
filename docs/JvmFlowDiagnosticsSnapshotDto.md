# JvmFlowDiagnosticsSnapshotDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uptime** | Option<**String**> | How long this node has been running, formatted as hours:minutes:seconds.milliseconds | [optional]
**time_zone** | Option<**String**> | The name of the Time Zone that is configured, if available | [optional]
**active_timer_driven_threads** | Option<**i32**> | The number of timer-driven threads that are active | [optional]
**active_event_driven_threads** | Option<**i32**> | The number of event-driven threads that are active | [optional]
**bundles_loaded** | Option<[**Vec<crate::models::BundleDto>**](BundleDTO.md)> | The NiFi Bundles (NARs) that are loaded by NiFi | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


