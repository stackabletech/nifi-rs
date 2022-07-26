# \ConnectionsApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_connection**](ConnectionsApi.md#delete_connection) | **DELETE** /connections/{id} | Deletes a connection
[**get_connection**](ConnectionsApi.md#get_connection) | **GET** /connections/{id} | Gets a connection
[**update_connection**](ConnectionsApi.md#update_connection) | **PUT** /connections/{id} | Updates a connection



## delete_connection

> crate::models::ConnectionEntity delete_connection(id, version, client_id, disconnected_node_acknowledged)
Deletes a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The connection id. | [required] |
**version** | Option<**String**> | The revision is used to verify the client is working with the latest version of the flow. |  |
**client_id** | Option<**String**> | If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response. |  |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::ConnectionEntity**](ConnectionEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connection

> crate::models::ConnectionEntity get_connection(id)
Gets a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The connection id. | [required] |

### Return type

[**crate::models::ConnectionEntity**](ConnectionEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_connection

> crate::models::ConnectionEntity update_connection(id, body)
Updates a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The connection id. | [required] |
**body** | [**ConnectionEntity**](ConnectionEntity.md) | The connection configuration details. | [required] |

### Return type

[**crate::models::ConnectionEntity**](ConnectionEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

