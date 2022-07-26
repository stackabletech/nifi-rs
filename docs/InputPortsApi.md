# \InputPortsApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_input_port**](InputPortsApi.md#get_input_port) | **GET** /input-ports/{id} | Gets an input port
[**remove_input_port**](InputPortsApi.md#remove_input_port) | **DELETE** /input-ports/{id} | Deletes an input port
[**update_input_port**](InputPortsApi.md#update_input_port) | **PUT** /input-ports/{id} | Updates an input port
[**update_run_status**](InputPortsApi.md#update_run_status) | **PUT** /input-ports/{id}/run-status | Updates run status of an input-port



## get_input_port

> crate::models::PortEntity get_input_port(id)
Gets an input port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The input port id. | [required] |

### Return type

[**crate::models::PortEntity**](PortEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_input_port

> crate::models::PortEntity remove_input_port(id, version, client_id, disconnected_node_acknowledged)
Deletes an input port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The input port id. | [required] |
**version** | Option<**String**> | The revision is used to verify the client is working with the latest version of the flow. |  |
**client_id** | Option<**String**> | If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response. |  |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::PortEntity**](PortEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_input_port

> crate::models::PortEntity update_input_port(id, body)
Updates an input port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The input port id. | [required] |
**body** | [**PortEntity**](PortEntity.md) | The input port configuration details. | [required] |

### Return type

[**crate::models::PortEntity**](PortEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_run_status

> crate::models::ProcessorEntity update_run_status(id, body)
Updates run status of an input-port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The port id. | [required] |
**body** | [**PortRunStatusEntity**](PortRunStatusEntity.md) | The port run status. | [required] |

### Return type

[**crate::models::ProcessorEntity**](ProcessorEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

