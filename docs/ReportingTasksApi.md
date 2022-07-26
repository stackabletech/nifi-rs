# \ReportingTasksApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**analyze_configuration**](ReportingTasksApi.md#analyze_configuration) | **POST** /reporting-tasks/{id}/config/analysis | Performs analysis of the component's configuration, providing information about which attributes are referenced.
[**clear_state**](ReportingTasksApi.md#clear_state) | **POST** /reporting-tasks/{id}/state/clear-requests | Clears the state for a reporting task
[**delete_validation_request**](ReportingTasksApi.md#delete_validation_request) | **DELETE** /reporting-tasks/{id}/config/verification-requests/{requestId} | Deletes the Verification Request with the given ID
[**get_property_descriptor**](ReportingTasksApi.md#get_property_descriptor) | **GET** /reporting-tasks/{id}/descriptors | Gets a reporting task property descriptor
[**get_reporting_task**](ReportingTasksApi.md#get_reporting_task) | **GET** /reporting-tasks/{id} | Gets a reporting task
[**get_state**](ReportingTasksApi.md#get_state) | **GET** /reporting-tasks/{id}/state | Gets the state for a reporting task
[**get_verification_request**](ReportingTasksApi.md#get_verification_request) | **GET** /reporting-tasks/{id}/config/verification-requests/{requestId} | Returns the Verification Request with the given ID
[**remove_reporting_task**](ReportingTasksApi.md#remove_reporting_task) | **DELETE** /reporting-tasks/{id} | Deletes a reporting task
[**submit_config_verification_request**](ReportingTasksApi.md#submit_config_verification_request) | **POST** /reporting-tasks/{id}/config/verification-requests | Performs verification of the Reporting Task's configuration
[**update_reporting_task**](ReportingTasksApi.md#update_reporting_task) | **PUT** /reporting-tasks/{id} | Updates a reporting task
[**update_run_status**](ReportingTasksApi.md#update_run_status) | **PUT** /reporting-tasks/{id}/run-status | Updates run status of a reporting task



## analyze_configuration

> crate::models::ConfigurationAnalysisEntity analyze_configuration(id, body)
Performs analysis of the component's configuration, providing information about which attributes are referenced.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The reporting task id. | [required] |
**body** | [**ConfigurationAnalysisEntity**](ConfigurationAnalysisEntity.md) | The configuration analysis request. | [required] |

### Return type

[**crate::models::ConfigurationAnalysisEntity**](ConfigurationAnalysisEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_state

> crate::models::ComponentStateEntity clear_state(id)
Clears the state for a reporting task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The reporting task id. | [required] |

### Return type

[**crate::models::ComponentStateEntity**](ComponentStateEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_validation_request

> crate::models::VerifyConfigRequestEntity delete_validation_request(id, request_id)
Deletes the Verification Request with the given ID

Deletes the Verification Request with the given ID. After a request is created, it is expected that the client will properly clean up the request by DELETE'ing it, once the Verification process has completed. If the request is deleted before the request completes, then the Verification request will finish the step that it is currently performing and then will cancel any subsequent steps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the Reporting Task | [required] |
**request_id** | **String** | The ID of the Verification Request | [required] |

### Return type

[**crate::models::VerifyConfigRequestEntity**](VerifyConfigRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_property_descriptor

> crate::models::PropertyDescriptorEntity get_property_descriptor(id, property_name)
Gets a reporting task property descriptor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The reporting task id. | [required] |
**property_name** | **String** | The property name. | [required] |

### Return type

[**crate::models::PropertyDescriptorEntity**](PropertyDescriptorEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reporting_task

> crate::models::ReportingTaskEntity get_reporting_task(id)
Gets a reporting task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The reporting task id. | [required] |

### Return type

[**crate::models::ReportingTaskEntity**](ReportingTaskEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_state

> crate::models::ComponentStateEntity get_state(id)
Gets the state for a reporting task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The reporting task id. | [required] |

### Return type

[**crate::models::ComponentStateEntity**](ComponentStateEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_verification_request

> crate::models::VerifyConfigRequestEntity get_verification_request(id, request_id)
Returns the Verification Request with the given ID

Returns the Verification Request with the given ID. Once an Verification Request has been created, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the Reporting Task | [required] |
**request_id** | **String** | The ID of the Verification Request | [required] |

### Return type

[**crate::models::VerifyConfigRequestEntity**](VerifyConfigRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_reporting_task

> crate::models::ReportingTaskEntity remove_reporting_task(id, version, client_id, disconnected_node_acknowledged)
Deletes a reporting task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The reporting task id. | [required] |
**version** | Option<**String**> | The revision is used to verify the client is working with the latest version of the flow. |  |
**client_id** | Option<**String**> | If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response. |  |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::ReportingTaskEntity**](ReportingTaskEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_config_verification_request

> crate::models::VerifyConfigRequestEntity submit_config_verification_request(id, body)
Performs verification of the Reporting Task's configuration

This will initiate the process of verifying a given Reporting Task configuration. This may be a long-running task. As a result, this endpoint will immediately return a ReportingTaskConfigVerificationRequestEntity, and the process of performing the verification will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /reporting-tasks/{serviceId}/verification-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /reporting-tasks/{serviceId}/verification-requests/{requestId}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The reporting task id. | [required] |
**body** | [**VerifyConfigRequestEntity**](VerifyConfigRequestEntity.md) | The reporting task configuration verification request. | [required] |

### Return type

[**crate::models::VerifyConfigRequestEntity**](VerifyConfigRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_reporting_task

> crate::models::ReportingTaskEntity update_reporting_task(id, body)
Updates a reporting task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The reporting task id. | [required] |
**body** | [**ReportingTaskEntity**](ReportingTaskEntity.md) | The reporting task configuration details. | [required] |

### Return type

[**crate::models::ReportingTaskEntity**](ReportingTaskEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_run_status

> crate::models::ReportingTaskEntity update_run_status(id, body)
Updates run status of a reporting task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The reporting task id. | [required] |
**body** | [**ReportingTaskRunStatusEntity**](ReportingTaskRunStatusEntity.md) | The reporting task run status. | [required] |

### Return type

[**crate::models::ReportingTaskEntity**](ReportingTaskEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

