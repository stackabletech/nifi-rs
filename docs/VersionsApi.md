# \VersionsApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_version_control_request**](VersionsApi.md#create_version_control_request) | **POST** /versions/active-requests | Create a version control request
[**delete_revert_request**](VersionsApi.md#delete_revert_request) | **DELETE** /versions/revert-requests/{id} | Deletes the Revert Request with the given ID
[**delete_update_request**](VersionsApi.md#delete_update_request) | **DELETE** /versions/update-requests/{id} | Deletes the Update Request with the given ID
[**delete_version_control_request**](VersionsApi.md#delete_version_control_request) | **DELETE** /versions/active-requests/{id} | Deletes the version control request with the given ID
[**export_flow_version**](VersionsApi.md#export_flow_version) | **GET** /versions/process-groups/{id}/download | Gets the latest version of a Process Group for download
[**get_revert_request**](VersionsApi.md#get_revert_request) | **GET** /versions/revert-requests/{id} | Returns the Revert Request with the given ID
[**get_update_request**](VersionsApi.md#get_update_request) | **GET** /versions/update-requests/{id} | Returns the Update Request with the given ID
[**get_version_information**](VersionsApi.md#get_version_information) | **GET** /versions/process-groups/{id} | Gets the Version Control information for a process group
[**initiate_revert_flow_version**](VersionsApi.md#initiate_revert_flow_version) | **POST** /versions/revert-requests/process-groups/{id} | Initiate the Revert Request of a Process Group with the given ID
[**initiate_version_control_update**](VersionsApi.md#initiate_version_control_update) | **POST** /versions/update-requests/process-groups/{id} | Initiate the Update Request of a Process Group with the given ID
[**save_to_flow_registry**](VersionsApi.md#save_to_flow_registry) | **POST** /versions/process-groups/{id} | Save the Process Group with the given ID
[**stop_version_control**](VersionsApi.md#stop_version_control) | **DELETE** /versions/process-groups/{id} | Stops version controlling the Process Group with the given ID
[**update_flow_version**](VersionsApi.md#update_flow_version) | **PUT** /versions/process-groups/{id} | Update the version of a Process Group with the given ID
[**update_version_control_request**](VersionsApi.md#update_version_control_request) | **PUT** /versions/active-requests/{id} | Updates the request with the given ID



## create_version_control_request

> String create_version_control_request(body)
Create a version control request

Creates a request so that a Process Group can be placed under Version Control or have its Version Control configuration changed. Creating this request will prevent any other threads from simultaneously saving local changes to Version Control. It will not, however, actually save the local flow to the Flow Registry. A POST to /versions/process-groups/{id} should be used to initiate saving of the local flow to the Flow Registry. Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateActiveRequestEntity**](CreateActiveRequestEntity.md) | The versioned flow details. | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_revert_request

> crate::models::VersionedFlowUpdateRequestEntity delete_revert_request(id, disconnected_node_acknowledged)
Deletes the Revert Request with the given ID

Deletes the Revert Request with the given ID. After a request is created via a POST to /versions/revert-requests/process-groups/{id}, it is expected that the client will properly clean up the request by DELETE'ing it, once the Revert process has completed. If the request is deleted before the request completes, then the Revert request will finish the step that it is currently performing and then will cancel any subsequent steps. Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the Revert Request | [required] |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::VersionedFlowUpdateRequestEntity**](VersionedFlowUpdateRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_update_request

> crate::models::VersionedFlowUpdateRequestEntity delete_update_request(id, disconnected_node_acknowledged)
Deletes the Update Request with the given ID

Deletes the Update Request with the given ID. After a request is created via a POST to /versions/update-requests/process-groups/{id}, it is expected that the client will properly clean up the request by DELETE'ing it, once the Update process has completed. If the request is deleted before the request completes, then the Update request will finish the step that it is currently performing and then will cancel any subsequent steps. Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the Update Request | [required] |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::VersionedFlowUpdateRequestEntity**](VersionedFlowUpdateRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_version_control_request

> delete_version_control_request(id, disconnected_node_acknowledged)
Deletes the version control request with the given ID

Deletes the Version Control Request with the given ID. This will allow other threads to save flows to the Flow Registry. See also the documentation for POSTing to /versions/active-requests for information regarding why this is done. Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The request ID. | [required] |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_flow_version

> String export_flow_version(id)
Gets the latest version of a Process Group for download

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_revert_request

> crate::models::VersionedFlowUpdateRequestEntity get_revert_request(id)
Returns the Revert Request with the given ID

Returns the Revert Request with the given ID. Once a Revert Request has been created by performing a POST to /versions/revert-requests/process-groups/{id}, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures. Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the Revert Request | [required] |

### Return type

[**crate::models::VersionedFlowUpdateRequestEntity**](VersionedFlowUpdateRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_update_request

> crate::models::VersionedFlowUpdateRequestEntity get_update_request(id)
Returns the Update Request with the given ID

Returns the Update Request with the given ID. Once an Update Request has been created by performing a POST to /versions/update-requests/process-groups/{id}, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures. Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the Update Request | [required] |

### Return type

[**crate::models::VersionedFlowUpdateRequestEntity**](VersionedFlowUpdateRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_version_information

> crate::models::VersionControlInformationEntity get_version_information(id)
Gets the Version Control information for a process group

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |

### Return type

[**crate::models::VersionControlInformationEntity**](VersionControlInformationEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## initiate_revert_flow_version

> crate::models::VersionedFlowUpdateRequestEntity initiate_revert_flow_version(id, body)
Initiate the Revert Request of a Process Group with the given ID

For a Process Group that is already under Version Control, this will initiate the action of reverting any local changes that have been made to the Process Group since it was last synchronized with the Flow Registry. This will result in the flow matching the Versioned Flow that exists in the Flow Registry. This can be a lengthy process, as it will stop any Processors and disable any Controller Services necessary to perform the action and then restart them. As a result, the endpoint will immediately return a VersionedFlowUpdateRequestEntity, and the process of updating the flow will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /versions/revert-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /versions/revert-requests/{requestId}. Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**VersionControlInformationEntity**](VersionControlInformationEntity.md) | The controller service configuration details. | [required] |

### Return type

[**crate::models::VersionedFlowUpdateRequestEntity**](VersionedFlowUpdateRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## initiate_version_control_update

> crate::models::VersionedFlowUpdateRequestEntity initiate_version_control_update(id, body)
Initiate the Update Request of a Process Group with the given ID

For a Process Group that is already under Version Control, this will initiate the action of changing from a specific version of the flow in the Flow Registry to a different version of the flow. This can be a lengthy process, as it will stop any Processors and disable any Controller Services necessary to perform the action and then restart them. As a result, the endpoint will immediately return a VersionedFlowUpdateRequestEntity, and the process of updating the flow will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /versions/update-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /versions/update-requests/{requestId}. Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**VersionControlInformationEntity**](VersionControlInformationEntity.md) | The controller service configuration details. | [required] |

### Return type

[**crate::models::VersionedFlowUpdateRequestEntity**](VersionedFlowUpdateRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_to_flow_registry

> crate::models::VersionControlInformationEntity save_to_flow_registry(id, body)
Save the Process Group with the given ID

Begins version controlling the Process Group with the given ID or commits changes to the Versioned Flow, depending on if the provided VersionControlInformation includes a flowId. Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**StartVersionControlRequestEntity**](StartVersionControlRequestEntity.md) | The versioned flow details. | [required] |

### Return type

[**crate::models::VersionControlInformationEntity**](VersionControlInformationEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_version_control

> crate::models::VersionControlInformationEntity stop_version_control(id, version, client_id, disconnected_node_acknowledged)
Stops version controlling the Process Group with the given ID

Stops version controlling the Process Group with the given ID. The Process Group will no longer track to any Versioned Flow. Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**version** | Option<**String**> | The version is used to verify the client is working with the latest version of the flow. |  |
**client_id** | Option<**String**> | If the client id is not specified, a new one will be generated. This value (whether specified or generated) is included in the response. |  |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::VersionControlInformationEntity**](VersionControlInformationEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_flow_version

> crate::models::VersionControlInformationEntity update_flow_version(id, body)
Update the version of a Process Group with the given ID

For a Process Group that is already under Version Control, this will update the version of the flow to a different version. This endpoint expects that the given snapshot will not modify any Processor that is currently running or any Controller Service that is enabled. Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**VersionedFlowSnapshotEntity**](VersionedFlowSnapshotEntity.md) | The controller service configuration details. | [required] |

### Return type

[**crate::models::VersionControlInformationEntity**](VersionControlInformationEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_version_control_request

> crate::models::VersionControlInformationEntity update_version_control_request(id, body)
Updates the request with the given ID

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The request ID. | [required] |
**body** | [**VersionControlComponentMappingEntity**](VersionControlComponentMappingEntity.md) | The version control component mapping. | [required] |

### Return type

[**crate::models::VersionControlInformationEntity**](VersionControlInformationEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

