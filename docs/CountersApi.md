# \CountersApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_counters**](CountersApi.md#get_counters) | **GET** /counters | Gets the current counters for this NiFi
[**update_counter**](CountersApi.md#update_counter) | **PUT** /counters/{id} | Updates the specified counter. This will reset the counter value to 0



## get_counters

> crate::models::CountersEntity get_counters(nodewise, cluster_node_id)
Gets the current counters for this NiFi

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nodewise** | Option<**bool**> | Whether or not to include the breakdown per node. Optional, defaults to false |  |[default to false]
**cluster_node_id** | Option<**String**> | The id of the node where to get the status. |  |

### Return type

[**crate::models::CountersEntity**](CountersEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_counter

> crate::models::CounterEntity update_counter(id)
Updates the specified counter. This will reset the counter value to 0

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id of the counter. | [required] |

### Return type

[**crate::models::CounterEntity**](CounterEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

