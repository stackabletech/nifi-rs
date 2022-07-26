# \RemoteProcessGroupsApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_remote_process_group**](RemoteProcessGroupsApi.md#get_remote_process_group) | **GET** /remote-process-groups/{id} | Gets a remote process group
[**get_state**](RemoteProcessGroupsApi.md#get_state) | **GET** /remote-process-groups/{id}/state | Gets the state for a RemoteProcessGroup
[**remove_remote_process_group**](RemoteProcessGroupsApi.md#remove_remote_process_group) | **DELETE** /remote-process-groups/{id} | Deletes a remote process group
[**update_remote_process_group**](RemoteProcessGroupsApi.md#update_remote_process_group) | **PUT** /remote-process-groups/{id} | Updates a remote process group
[**update_remote_process_group_input_port**](RemoteProcessGroupsApi.md#update_remote_process_group_input_port) | **PUT** /remote-process-groups/{id}/input-ports/{port-id} | Updates a remote port
[**update_remote_process_group_input_port_run_status**](RemoteProcessGroupsApi.md#update_remote_process_group_input_port_run_status) | **PUT** /remote-process-groups/{id}/input-ports/{port-id}/run-status | Updates run status of a remote port
[**update_remote_process_group_output_port**](RemoteProcessGroupsApi.md#update_remote_process_group_output_port) | **PUT** /remote-process-groups/{id}/output-ports/{port-id} | Updates a remote port
[**update_remote_process_group_output_port_run_status**](RemoteProcessGroupsApi.md#update_remote_process_group_output_port_run_status) | **PUT** /remote-process-groups/{id}/output-ports/{port-id}/run-status | Updates run status of a remote port
[**update_remote_process_group_run_status**](RemoteProcessGroupsApi.md#update_remote_process_group_run_status) | **PUT** /remote-process-groups/{id}/run-status | Updates run status of a remote process group
[**update_remote_process_group_run_statuses**](RemoteProcessGroupsApi.md#update_remote_process_group_run_statuses) | **PUT** /remote-process-groups/process-group/{id}/run-status | Updates run status of all remote process groups in a process group (recursively)



## get_remote_process_group

> crate::models::RemoteProcessGroupEntity get_remote_process_group(id)
Gets a remote process group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The remote process group id. | [required] |

### Return type

[**crate::models::RemoteProcessGroupEntity**](RemoteProcessGroupEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_state

> crate::models::ComponentStateEntity get_state(id)
Gets the state for a RemoteProcessGroup

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The processor id. | [required] |

### Return type

[**crate::models::ComponentStateEntity**](ComponentStateEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_remote_process_group

> crate::models::RemoteProcessGroupEntity remove_remote_process_group(id, version, client_id, disconnected_node_acknowledged)
Deletes a remote process group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The remote process group id. | [required] |
**version** | Option<**String**> | The revision is used to verify the client is working with the latest version of the flow. |  |
**client_id** | Option<**String**> | If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response. |  |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::RemoteProcessGroupEntity**](RemoteProcessGroupEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_remote_process_group

> crate::models::RemoteProcessGroupEntity update_remote_process_group(id, body)
Updates a remote process group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The remote process group id. | [required] |
**body** | [**RemoteProcessGroupEntity**](RemoteProcessGroupEntity.md) | The remote process group. | [required] |

### Return type

[**crate::models::RemoteProcessGroupEntity**](RemoteProcessGroupEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_remote_process_group_input_port

> crate::models::RemoteProcessGroupPortEntity update_remote_process_group_input_port(id, port_id, body)
Updates a remote port

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The remote process group id. | [required] |
**port_id** | **String** | The remote process group port id. | [required] |
**body** | [**RemoteProcessGroupPortEntity**](RemoteProcessGroupPortEntity.md) | The remote process group port. | [required] |

### Return type

[**crate::models::RemoteProcessGroupPortEntity**](RemoteProcessGroupPortEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_remote_process_group_input_port_run_status

> crate::models::RemoteProcessGroupPortEntity update_remote_process_group_input_port_run_status(id, port_id, body)
Updates run status of a remote port

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The remote process group id. | [required] |
**port_id** | **String** | The remote process group port id. | [required] |
**body** | [**RemotePortRunStatusEntity**](RemotePortRunStatusEntity.md) | The remote process group port. | [required] |

### Return type

[**crate::models::RemoteProcessGroupPortEntity**](RemoteProcessGroupPortEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_remote_process_group_output_port

> crate::models::RemoteProcessGroupPortEntity update_remote_process_group_output_port(id, port_id, body)
Updates a remote port

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The remote process group id. | [required] |
**port_id** | **String** | The remote process group port id. | [required] |
**body** | [**RemoteProcessGroupPortEntity**](RemoteProcessGroupPortEntity.md) | The remote process group port. | [required] |

### Return type

[**crate::models::RemoteProcessGroupPortEntity**](RemoteProcessGroupPortEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_remote_process_group_output_port_run_status

> crate::models::RemoteProcessGroupPortEntity update_remote_process_group_output_port_run_status(id, port_id, body)
Updates run status of a remote port

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The remote process group id. | [required] |
**port_id** | **String** | The remote process group port id. | [required] |
**body** | [**RemotePortRunStatusEntity**](RemotePortRunStatusEntity.md) | The remote process group port. | [required] |

### Return type

[**crate::models::RemoteProcessGroupPortEntity**](RemoteProcessGroupPortEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_remote_process_group_run_status

> crate::models::RemoteProcessGroupEntity update_remote_process_group_run_status(id, body)
Updates run status of a remote process group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The remote process group id. | [required] |
**body** | [**RemotePortRunStatusEntity**](RemotePortRunStatusEntity.md) | The remote process group run status. | [required] |

### Return type

[**crate::models::RemoteProcessGroupEntity**](RemoteProcessGroupEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_remote_process_group_run_statuses

> crate::models::RemoteProcessGroupEntity update_remote_process_group_run_statuses(id, body)
Updates run status of all remote process groups in a process group (recursively)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**RemotePortRunStatusEntity**](RemotePortRunStatusEntity.md) | The remote process groups run status. | [required] |

### Return type

[**crate::models::RemoteProcessGroupEntity**](RemoteProcessGroupEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

