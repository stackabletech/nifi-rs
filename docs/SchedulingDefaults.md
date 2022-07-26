# SchedulingDefaults

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_scheduling_strategy** | Option<**String**> | The name of the default scheduling strategy | [optional]
**default_scheduling_period_millis** | Option<**i64**> | The default scheduling period in milliseconds | [optional]
**penalization_period_millis** | Option<**i64**> | The default penalization period in milliseconds | [optional]
**yield_duration_millis** | Option<**i64**> | The default yield duration in milliseconds | [optional]
**default_run_duration_nanos** | Option<**i64**> | The default run duration in nano-seconds | [optional]
**default_max_concurrent_tasks** | Option<**String**> | The default concurrent tasks | [optional]
**default_concurrent_tasks_by_scheduling_strategy** | Option<**::std::collections::HashMap<String, i32>**> | The default concurrent tasks for each scheduling strategy | [optional]
**default_scheduling_periods_by_scheduling_strategy** | Option<**::std::collections::HashMap<String, String>**> | The default scheduling period for each scheduling strategy | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


