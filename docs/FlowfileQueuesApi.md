# \FlowfileQueuesApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_drop_request**](FlowfileQueuesApi.md#create_drop_request) | **POST** /flowfile-queues/{id}/drop-requests | Creates a request to drop the contents of the queue in this connection.
[**create_flow_file_listing**](FlowfileQueuesApi.md#create_flow_file_listing) | **POST** /flowfile-queues/{id}/listing-requests | Lists the contents of the queue in this connection.
[**delete_listing_request**](FlowfileQueuesApi.md#delete_listing_request) | **DELETE** /flowfile-queues/{id}/listing-requests/{listing-request-id} | Cancels and/or removes a request to list the contents of this connection.
[**download_flow_file_content**](FlowfileQueuesApi.md#download_flow_file_content) | **GET** /flowfile-queues/{id}/flowfiles/{flowfile-uuid}/content | Gets the content for a FlowFile in a Connection.
[**get_drop_request**](FlowfileQueuesApi.md#get_drop_request) | **GET** /flowfile-queues/{id}/drop-requests/{drop-request-id} | Gets the current status of a drop request for the specified connection.
[**get_flow_file**](FlowfileQueuesApi.md#get_flow_file) | **GET** /flowfile-queues/{id}/flowfiles/{flowfile-uuid} | Gets a FlowFile from a Connection.
[**get_listing_request**](FlowfileQueuesApi.md#get_listing_request) | **GET** /flowfile-queues/{id}/listing-requests/{listing-request-id} | Gets the current status of a listing request for the specified connection.
[**remove_drop_request**](FlowfileQueuesApi.md#remove_drop_request) | **DELETE** /flowfile-queues/{id}/drop-requests/{drop-request-id} | Cancels and/or removes a request to drop the contents of this connection.



## create_drop_request

> crate::models::DropRequestEntity create_drop_request(id)
Creates a request to drop the contents of the queue in this connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The connection id. | [required] |

### Return type

[**crate::models::DropRequestEntity**](DropRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_flow_file_listing

> crate::models::ListingRequestEntity create_flow_file_listing(id)
Lists the contents of the queue in this connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The connection id. | [required] |

### Return type

[**crate::models::ListingRequestEntity**](ListingRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_listing_request

> crate::models::ListingRequestEntity delete_listing_request(id, listing_request_id)
Cancels and/or removes a request to list the contents of this connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The connection id. | [required] |
**listing_request_id** | **String** | The listing request id. | [required] |

### Return type

[**crate::models::ListingRequestEntity**](ListingRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_flow_file_content

> serde_json::Value download_flow_file_content(id, flowfile_uuid, client_id, cluster_node_id)
Gets the content for a FlowFile in a Connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The connection id. | [required] |
**flowfile_uuid** | **String** | The flowfile uuid. | [required] |
**client_id** | Option<**String**> | If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response. |  |
**cluster_node_id** | Option<**String**> | The id of the node where the content exists if clustered. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_drop_request

> crate::models::DropRequestEntity get_drop_request(id, drop_request_id)
Gets the current status of a drop request for the specified connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The connection id. | [required] |
**drop_request_id** | **String** | The drop request id. | [required] |

### Return type

[**crate::models::DropRequestEntity**](DropRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flow_file

> crate::models::FlowFileEntity get_flow_file(id, flowfile_uuid, cluster_node_id)
Gets a FlowFile from a Connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The connection id. | [required] |
**flowfile_uuid** | **String** | The flowfile uuid. | [required] |
**cluster_node_id** | Option<**String**> | The id of the node where the content exists if clustered. |  |

### Return type

[**crate::models::FlowFileEntity**](FlowFileEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_listing_request

> crate::models::ListingRequestEntity get_listing_request(id, listing_request_id)
Gets the current status of a listing request for the specified connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The connection id. | [required] |
**listing_request_id** | **String** | The listing request id. | [required] |

### Return type

[**crate::models::ListingRequestEntity**](ListingRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_drop_request

> crate::models::DropRequestEntity remove_drop_request(id, drop_request_id)
Cancels and/or removes a request to drop the contents of this connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The connection id. | [required] |
**drop_request_id** | **String** | The drop request id. | [required] |

### Return type

[**crate::models::DropRequestEntity**](DropRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

