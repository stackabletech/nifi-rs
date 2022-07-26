# FlowConfigurationDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**supports_managed_authorizer** | Option<**bool**> | Whether this NiFi supports a managed authorizer. Managed authorizers can visualize users, groups, and policies in the UI. | [optional]
**supports_configurable_authorizer** | Option<**bool**> | Whether this NiFi supports a configurable authorizer. | [optional]
**supports_configurable_users_and_groups** | Option<**bool**> | Whether this NiFi supports configurable users and groups. | [optional]
**auto_refresh_interval_seconds** | Option<**i64**> | The interval in seconds between the automatic NiFi refresh requests. | [optional]
**current_time** | Option<**String**> | The current time on the system. | [optional]
**time_offset** | Option<**i32**> | The time offset of the system. | [optional]
**default_back_pressure_object_threshold** | Option<**i64**> | The default back pressure object threshold. | [optional]
**default_back_pressure_data_size_threshold** | Option<**String**> | The default back pressure data size threshold. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


