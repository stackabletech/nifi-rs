# \ProvenanceApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lineage**](ProvenanceApi.md#delete_lineage) | **DELETE** /provenance/lineage/{id} | Deletes a lineage query
[**delete_provenance**](ProvenanceApi.md#delete_provenance) | **DELETE** /provenance/{id} | Deletes a provenance query
[**get_lineage**](ProvenanceApi.md#get_lineage) | **GET** /provenance/lineage/{id} | Gets a lineage query
[**get_provenance**](ProvenanceApi.md#get_provenance) | **GET** /provenance/{id} | Gets a provenance query
[**get_search_options**](ProvenanceApi.md#get_search_options) | **GET** /provenance/search-options | Gets the searchable attributes for provenance events
[**submit_lineage_request**](ProvenanceApi.md#submit_lineage_request) | **POST** /provenance/lineage | Submits a lineage query
[**submit_provenance_request**](ProvenanceApi.md#submit_provenance_request) | **POST** /provenance | Submits a provenance query



## delete_lineage

> crate::models::LineageEntity delete_lineage(id, cluster_node_id)
Deletes a lineage query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id of the lineage query. | [required] |
**cluster_node_id** | Option<**String**> | The id of the node where this query exists if clustered. |  |

### Return type

[**crate::models::LineageEntity**](LineageEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_provenance

> crate::models::ProvenanceEntity delete_provenance(id, cluster_node_id)
Deletes a provenance query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id of the provenance query. | [required] |
**cluster_node_id** | Option<**String**> | The id of the node where this query exists if clustered. |  |

### Return type

[**crate::models::ProvenanceEntity**](ProvenanceEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lineage

> crate::models::LineageEntity get_lineage(id, cluster_node_id)
Gets a lineage query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id of the lineage query. | [required] |
**cluster_node_id** | Option<**String**> | The id of the node where this query exists if clustered. |  |

### Return type

[**crate::models::LineageEntity**](LineageEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_provenance

> crate::models::ProvenanceEntity get_provenance(id, cluster_node_id, summarize, incremental_results)
Gets a provenance query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id of the provenance query. | [required] |
**cluster_node_id** | Option<**String**> | The id of the node where this query exists if clustered. |  |
**summarize** | Option<**bool**> | Whether or not incremental results are returned. If false, provenance events are only returned once the query completes. This property is true by default. |  |[default to false]
**incremental_results** | Option<**bool**> | Whether or not to summarize provenance events returned. This property is false by default. |  |[default to true]

### Return type

[**crate::models::ProvenanceEntity**](ProvenanceEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_options

> crate::models::ProvenanceOptionsEntity get_search_options()
Gets the searchable attributes for provenance events

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ProvenanceOptionsEntity**](ProvenanceOptionsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_lineage_request

> crate::models::LineageEntity submit_lineage_request(body)
Submits a lineage query

Lineage queries may be long running so this endpoint submits a request. The response will include the current state of the query. If the request is not completed the URI in the response can be used at a later time to get the updated state of the query. Once the query has completed the lineage request should be deleted by the client who originally submitted it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LineageEntity**](LineageEntity.md) | The lineage query details. | [required] |

### Return type

[**crate::models::LineageEntity**](LineageEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_provenance_request

> crate::models::ProvenanceEntity submit_provenance_request(body)
Submits a provenance query

Provenance queries may be long running so this endpoint submits a request. The response will include the current state of the query. If the request is not completed the URI in the response can be used at a later time to get the updated state of the query. Once the query has completed the provenance request should be deleted by the client who originally submitted it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ProvenanceEntity**](ProvenanceEntity.md) | The provenance query details. | [required] |

### Return type

[**crate::models::ProvenanceEntity**](ProvenanceEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

