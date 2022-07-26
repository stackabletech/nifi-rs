# RuntimeManifest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identifier** | Option<**String**> | A unique identifier for the manifest | [optional]
**agent_type** | Option<**String**> | The type of the runtime binary, e.g., 'minifi-java' or 'minifi-cpp' | [optional]
**version** | Option<**String**> | The version of the runtime binary, e.g., '1.0.1' | [optional]
**build_info** | Option<[**crate::models::BuildInfo**](BuildInfo.md)> |  | [optional]
**bundles** | Option<[**Vec<crate::models::Bundle>**](Bundle.md)> | All extension bundles included with this runtime | [optional]
**scheduling_defaults** | Option<[**crate::models::SchedulingDefaults**](SchedulingDefaults.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


