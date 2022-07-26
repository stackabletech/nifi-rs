# \ProvenanceEventsApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_input_content**](ProvenanceEventsApi.md#get_input_content) | **GET** /provenance-events/{id}/content/input | Gets the input content for a provenance event
[**get_output_content**](ProvenanceEventsApi.md#get_output_content) | **GET** /provenance-events/{id}/content/output | Gets the output content for a provenance event
[**get_provenance_event**](ProvenanceEventsApi.md#get_provenance_event) | **GET** /provenance-events/{id} | Gets a provenance event
[**submit_replay**](ProvenanceEventsApi.md#submit_replay) | **POST** /provenance-events/replays | Replays content from a provenance event



## get_input_content

> serde_json::Value get_input_content(id, cluster_node_id)
Gets the input content for a provenance event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The provenance event id. | [required] |
**cluster_node_id** | Option<**String**> | The id of the node where the content exists if clustered. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_output_content

> serde_json::Value get_output_content(id, cluster_node_id)
Gets the output content for a provenance event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The provenance event id. | [required] |
**cluster_node_id** | Option<**String**> | The id of the node where the content exists if clustered. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_provenance_event

> crate::models::ProvenanceEventEntity get_provenance_event(id, cluster_node_id)
Gets a provenance event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The provenance event id. | [required] |
**cluster_node_id** | Option<**String**> | The id of the node where this event exists if clustered. |  |

### Return type

[**crate::models::ProvenanceEventEntity**](ProvenanceEventEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_replay

> crate::models::ProvenanceEventEntity submit_replay(body)
Replays content from a provenance event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SubmitReplayRequestEntity**](SubmitReplayRequestEntity.md) | The replay request. | [required] |

### Return type

[**crate::models::ProvenanceEventEntity**](ProvenanceEventEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

