# \ProcessorsApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**analyze_configuration**](ProcessorsApi.md#analyze_configuration) | **POST** /processors/{id}/config/analysis | Performs analysis of the component's configuration, providing information about which attributes are referenced.
[**clear_state**](ProcessorsApi.md#clear_state) | **POST** /processors/{id}/state/clear-requests | Clears the state for a processor
[**delete_processor**](ProcessorsApi.md#delete_processor) | **DELETE** /processors/{id} | Deletes a processor
[**delete_verification_request**](ProcessorsApi.md#delete_verification_request) | **DELETE** /processors/{id}/config/verification-requests/{requestId} | Deletes the Verification Request with the given ID
[**get_processor**](ProcessorsApi.md#get_processor) | **GET** /processors/{id} | Gets a processor
[**get_processor_diagnostics**](ProcessorsApi.md#get_processor_diagnostics) | **GET** /processors/{id}/diagnostics | Gets diagnostics information about a processor
[**get_processor_run_status_details**](ProcessorsApi.md#get_processor_run_status_details) | **POST** /processors/run-status-details/queries | Submits a query to retrieve the run status details of all processors that are in the given list of Processor IDs
[**get_property_descriptor**](ProcessorsApi.md#get_property_descriptor) | **GET** /processors/{id}/descriptors | Gets the descriptor for a processor property
[**get_state**](ProcessorsApi.md#get_state) | **GET** /processors/{id}/state | Gets the state for a processor
[**get_verification_request**](ProcessorsApi.md#get_verification_request) | **GET** /processors/{id}/config/verification-requests/{requestId} | Returns the Verification Request with the given ID
[**submit_processor_verification_request**](ProcessorsApi.md#submit_processor_verification_request) | **POST** /processors/{id}/config/verification-requests | Performs verification of the Processor's configuration
[**terminate_processor**](ProcessorsApi.md#terminate_processor) | **DELETE** /processors/{id}/threads | Terminates a processor, essentially \"deleting\" its threads and any active tasks
[**update_processor**](ProcessorsApi.md#update_processor) | **PUT** /processors/{id} | Updates a processor
[**update_run_status**](ProcessorsApi.md#update_run_status) | **PUT** /processors/{id}/run-status | Updates run status of a processor



## analyze_configuration

> crate::models::ConfigurationAnalysisEntity analyze_configuration(id, body)
Performs analysis of the component's configuration, providing information about which attributes are referenced.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The processor id. | [required] |
**body** | [**ConfigurationAnalysisEntity**](ConfigurationAnalysisEntity.md) | The processor configuration analysis request. | [required] |

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
Clears the state for a processor

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


## delete_processor

> crate::models::ProcessorEntity delete_processor(id, version, client_id, disconnected_node_acknowledged)
Deletes a processor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The processor id. | [required] |
**version** | Option<**String**> | The revision is used to verify the client is working with the latest version of the flow. |  |
**client_id** | Option<**String**> | If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response. |  |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::ProcessorEntity**](ProcessorEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_verification_request

> crate::models::VerifyConfigRequestEntity delete_verification_request(id, request_id)
Deletes the Verification Request with the given ID

Deletes the Verification Request with the given ID. After a request is created, it is expected that the client will properly clean up the request by DELETE'ing it, once the Verification process has completed. If the request is deleted before the request completes, then the Verification request will finish the step that it is currently performing and then will cancel any subsequent steps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the Processor | [required] |
**request_id** | **String** | The ID of the Verification Request | [required] |

### Return type

[**crate::models::VerifyConfigRequestEntity**](VerifyConfigRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_processor

> crate::models::ProcessorEntity get_processor(id)
Gets a processor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The processor id. | [required] |

### Return type

[**crate::models::ProcessorEntity**](ProcessorEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_processor_diagnostics

> crate::models::ProcessorEntity get_processor_diagnostics(id)
Gets diagnostics information about a processor

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The processor id. | [required] |

### Return type

[**crate::models::ProcessorEntity**](ProcessorEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_processor_run_status_details

> crate::models::ProcessorsRunStatusDetailsEntity get_processor_run_status_details(body)
Submits a query to retrieve the run status details of all processors that are in the given list of Processor IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**RunStatusDetailsRequestEntity**](RunStatusDetailsRequestEntity.md)> | The request for the processors that should be included in the results |  |

### Return type

[**crate::models::ProcessorsRunStatusDetailsEntity**](ProcessorsRunStatusDetailsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_property_descriptor

> crate::models::PropertyDescriptorEntity get_property_descriptor(id, property_name, client_id)
Gets the descriptor for a processor property

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The processor id. | [required] |
**property_name** | **String** | The property name. | [required] |
**client_id** | Option<**String**> | If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response. |  |

### Return type

[**crate::models::PropertyDescriptorEntity**](PropertyDescriptorEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_state

> crate::models::ComponentStateEntity get_state(id)
Gets the state for a processor

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


## get_verification_request

> crate::models::VerifyConfigRequestEntity get_verification_request(id, request_id)
Returns the Verification Request with the given ID

Returns the Verification Request with the given ID. Once an Verification Request has been created, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the Processor | [required] |
**request_id** | **String** | The ID of the Verification Request | [required] |

### Return type

[**crate::models::VerifyConfigRequestEntity**](VerifyConfigRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_processor_verification_request

> crate::models::VerifyConfigRequestEntity submit_processor_verification_request(id, body)
Performs verification of the Processor's configuration

This will initiate the process of verifying a given Processor configuration. This may be a long-running task. As a result, this endpoint will immediately return a ProcessorConfigVerificationRequestEntity, and the process of performing the verification will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /processors/{processorId}/verification-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /processors/{processorId}/verification-requests/{requestId}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The processor id. | [required] |
**body** | [**VerifyConfigRequestEntity**](VerifyConfigRequestEntity.md) | The processor configuration verification request. | [required] |

### Return type

[**crate::models::VerifyConfigRequestEntity**](VerifyConfigRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## terminate_processor

> crate::models::ProcessorEntity terminate_processor(id)
Terminates a processor, essentially \"deleting\" its threads and any active tasks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The processor id. | [required] |

### Return type

[**crate::models::ProcessorEntity**](ProcessorEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_processor

> crate::models::ProcessorEntity update_processor(id, body)
Updates a processor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The processor id. | [required] |
**body** | [**ProcessorEntity**](ProcessorEntity.md) | The processor configuration details. | [required] |

### Return type

[**crate::models::ProcessorEntity**](ProcessorEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_run_status

> crate::models::ProcessorEntity update_run_status(id, body)
Updates run status of a processor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The processor id. | [required] |
**body** | [**ProcessorRunStatusEntity**](ProcessorRunStatusEntity.md) | The processor run status. | [required] |

### Return type

[**crate::models::ProcessorEntity**](ProcessorEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

