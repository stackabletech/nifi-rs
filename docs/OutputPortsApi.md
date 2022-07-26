# \OutputPortsApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_output_port**](OutputPortsApi.md#get_output_port) | **GET** /output-ports/{id} | Gets an output port
[**remove_output_port**](OutputPortsApi.md#remove_output_port) | **DELETE** /output-ports/{id} | Deletes an output port
[**update_output_port**](OutputPortsApi.md#update_output_port) | **PUT** /output-ports/{id} | Updates an output port
[**update_run_status**](OutputPortsApi.md#update_run_status) | **PUT** /output-ports/{id}/run-status | Updates run status of an output-port



## get_output_port

> crate::models::PortEntity get_output_port(id)
Gets an output port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The output port id. | [required] |

### Return type

[**crate::models::PortEntity**](PortEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_output_port

> crate::models::PortEntity remove_output_port(id, version, client_id, disconnected_node_acknowledged)
Deletes an output port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The output port id. | [required] |
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


## update_output_port

> crate::models::PortEntity update_output_port(id, body)
Updates an output port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The output port id. | [required] |
**body** | [**PortEntity**](PortEntity.md) | The output port configuration details. | [required] |

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
Updates run status of an output-port

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

