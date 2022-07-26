# \DataTransferApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**commit_input_port_transaction**](DataTransferApi.md#commit_input_port_transaction) | **DELETE** /data-transfer/input-ports/{portId}/transactions/{transactionId} | Commit or cancel the specified transaction
[**commit_output_port_transaction**](DataTransferApi.md#commit_output_port_transaction) | **DELETE** /data-transfer/output-ports/{portId}/transactions/{transactionId} | Commit or cancel the specified transaction
[**create_port_transaction**](DataTransferApi.md#create_port_transaction) | **POST** /data-transfer/{portType}/{portId}/transactions | Create a transaction to the specified output port or input port
[**extend_input_port_transaction_ttl**](DataTransferApi.md#extend_input_port_transaction_ttl) | **PUT** /data-transfer/input-ports/{portId}/transactions/{transactionId} | Extend transaction TTL
[**extend_output_port_transaction_ttl**](DataTransferApi.md#extend_output_port_transaction_ttl) | **PUT** /data-transfer/output-ports/{portId}/transactions/{transactionId} | Extend transaction TTL
[**receive_flow_files**](DataTransferApi.md#receive_flow_files) | **POST** /data-transfer/input-ports/{portId}/transactions/{transactionId}/flow-files | Transfer flow files to the input port
[**transfer_flow_files**](DataTransferApi.md#transfer_flow_files) | **GET** /data-transfer/output-ports/{portId}/transactions/{transactionId}/flow-files | Transfer flow files from the output port



## commit_input_port_transaction

> crate::models::TransactionResultEntity commit_input_port_transaction(response_code, port_id, transaction_id)
Commit or cancel the specified transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**response_code** | **i32** | The response code. Available values are BAD_CHECKSUM(19), CONFIRM_TRANSACTION(12) or CANCEL_TRANSACTION(15). | [required] |
**port_id** | **String** | The input port id. | [required] |
**transaction_id** | **String** | The transaction id. | [required] |

### Return type

[**crate::models::TransactionResultEntity**](TransactionResultEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## commit_output_port_transaction

> crate::models::TransactionResultEntity commit_output_port_transaction(response_code, checksum, port_id, transaction_id)
Commit or cancel the specified transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**response_code** | **i32** | The response code. Available values are CONFIRM_TRANSACTION(12) or CANCEL_TRANSACTION(15). | [required] |
**checksum** | **String** | A checksum calculated at client side using CRC32 to check flow file content integrity. It must match with the value calculated at server side. | [required] |
**port_id** | **String** | The output port id. | [required] |
**transaction_id** | **String** | The transaction id. | [required] |

### Return type

[**crate::models::TransactionResultEntity**](TransactionResultEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_port_transaction

> crate::models::TransactionResultEntity create_port_transaction(port_type, port_id)
Create a transaction to the specified output port or input port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**port_type** | **String** | The port type. | [required] |
**port_id** | **String** |  | [required] |

### Return type

[**crate::models::TransactionResultEntity**](TransactionResultEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extend_input_port_transaction_ttl

> crate::models::TransactionResultEntity extend_input_port_transaction_ttl(port_id, transaction_id)
Extend transaction TTL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**port_id** | **String** |  | [required] |
**transaction_id** | **String** |  | [required] |

### Return type

[**crate::models::TransactionResultEntity**](TransactionResultEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extend_output_port_transaction_ttl

> crate::models::TransactionResultEntity extend_output_port_transaction_ttl(port_id, transaction_id)
Extend transaction TTL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**port_id** | **String** |  | [required] |
**transaction_id** | **String** |  | [required] |

### Return type

[**crate::models::TransactionResultEntity**](TransactionResultEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## receive_flow_files

> String receive_flow_files(port_id, transaction_id)
Transfer flow files to the input port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**port_id** | **String** | The input port id. | [required] |
**transaction_id** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_flow_files

> serde_json::Value transfer_flow_files(port_id, transaction_id)
Transfer flow files from the output port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**port_id** | **String** | The output port id. | [required] |
**transaction_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

