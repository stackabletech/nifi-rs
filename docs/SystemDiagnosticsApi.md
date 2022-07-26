# \SystemDiagnosticsApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_system_diagnostics**](SystemDiagnosticsApi.md#get_system_diagnostics) | **GET** /system-diagnostics | Gets the diagnostics for the system NiFi is running on



## get_system_diagnostics

> crate::models::SystemDiagnosticsEntity get_system_diagnostics(nodewise, cluster_node_id)
Gets the diagnostics for the system NiFi is running on

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nodewise** | Option<**bool**> | Whether or not to include the breakdown per node. Optional, defaults to false |  |[default to false]
**cluster_node_id** | Option<**String**> | The id of the node where to get the status. |  |

### Return type

[**crate::models::SystemDiagnosticsEntity**](SystemDiagnosticsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

