# \ParameterContextsApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_parameter_context**](ParameterContextsApi.md#create_parameter_context) | **POST** /parameter-contexts | Create a Parameter Context
[**delete_parameter_context**](ParameterContextsApi.md#delete_parameter_context) | **DELETE** /parameter-contexts/{id} | Deletes the Parameter Context with the given ID
[**delete_update_request**](ParameterContextsApi.md#delete_update_request) | **DELETE** /parameter-contexts/{contextId}/update-requests/{requestId} | Deletes the Update Request with the given ID
[**delete_validation_request**](ParameterContextsApi.md#delete_validation_request) | **DELETE** /parameter-contexts/{contextId}/validation-requests/{id} | Deletes the Validation Request with the given ID
[**get_parameter_context**](ParameterContextsApi.md#get_parameter_context) | **GET** /parameter-contexts/{id} | Returns the Parameter Context with the given ID
[**get_parameter_context_update**](ParameterContextsApi.md#get_parameter_context_update) | **GET** /parameter-contexts/{contextId}/update-requests/{requestId} | Returns the Update Request with the given ID
[**get_validation_request**](ParameterContextsApi.md#get_validation_request) | **GET** /parameter-contexts/{contextId}/validation-requests/{id} | Returns the Validation Request with the given ID
[**submit_parameter_context_update**](ParameterContextsApi.md#submit_parameter_context_update) | **POST** /parameter-contexts/{contextId}/update-requests | Initiate the Update Request of a Parameter Context
[**submit_validation_request**](ParameterContextsApi.md#submit_validation_request) | **POST** /parameter-contexts/{contextId}/validation-requests | Initiate a Validation Request to determine how the validity of components will change if a Parameter Context were to be updated
[**update_parameter_context**](ParameterContextsApi.md#update_parameter_context) | **PUT** /parameter-contexts/{id} | Modifies a Parameter Context



## create_parameter_context

> crate::models::ParameterContextEntity create_parameter_context(body)
Create a Parameter Context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ParameterContextEntity**](ParameterContextEntity.md) | The Parameter Context. | [required] |

### Return type

[**crate::models::ParameterContextEntity**](ParameterContextEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_parameter_context

> crate::models::ParameterContextEntity delete_parameter_context(id, version, client_id, disconnected_node_acknowledged)
Deletes the Parameter Context with the given ID

Deletes the Parameter Context with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The Parameter Context ID. | [required] |
**version** | Option<**String**> | The version is used to verify the client is working with the latest version of the flow. |  |
**client_id** | Option<**String**> | If the client id is not specified, a new one will be generated. This value (whether specified or generated) is included in the response. |  |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::ParameterContextEntity**](ParameterContextEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_update_request

> crate::models::ParameterContextUpdateRequestEntity delete_update_request(context_id, request_id, disconnected_node_acknowledged)
Deletes the Update Request with the given ID

Deletes the Update Request with the given ID. After a request is created via a POST to /nifi-api/parameter-contexts/update-requests, it is expected that the client will properly clean up the request by DELETE'ing it, once the Update process has completed. If the request is deleted before the request completes, then the Update request will finish the step that it is currently performing and then will cancel any subsequent steps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **String** | The ID of the ParameterContext | [required] |
**request_id** | **String** | The ID of the Update Request | [required] |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::ParameterContextUpdateRequestEntity**](ParameterContextUpdateRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_validation_request

> crate::models::ParameterContextValidationRequestEntity delete_validation_request(context_id, id, disconnected_node_acknowledged)
Deletes the Validation Request with the given ID

Deletes the Validation Request with the given ID. After a request is created via a POST to /nifi-api/validation-contexts, it is expected that the client will properly clean up the request by DELETE'ing it, once the validation process has completed. If the request is deleted before the request completes, then the Validation request will finish the step that it is currently performing and then will cancel any subsequent steps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **String** | The ID of the Parameter Context | [required] |
**id** | **String** | The ID of the Update Request | [required] |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::ParameterContextValidationRequestEntity**](ParameterContextValidationRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_parameter_context

> crate::models::ParameterContextEntity get_parameter_context(id, include_inherited_parameters)
Returns the Parameter Context with the given ID

Returns the Parameter Context with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the Parameter Context | [required] |
**include_inherited_parameters** | Option<**bool**> | Whether or not to include inherited parameters from other parameter contexts, and therefore also overridden values.  If true, the result will be the 'effective' parameter context. |  |[default to false]

### Return type

[**crate::models::ParameterContextEntity**](ParameterContextEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_parameter_context_update

> crate::models::ParameterContextUpdateRequestEntity get_parameter_context_update(context_id, request_id)
Returns the Update Request with the given ID

Returns the Update Request with the given ID. Once an Update Request has been created by performing a POST to /nifi-api/parameter-contexts, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **String** | The ID of the Parameter Context | [required] |
**request_id** | **String** | The ID of the Update Request | [required] |

### Return type

[**crate::models::ParameterContextUpdateRequestEntity**](ParameterContextUpdateRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_validation_request

> crate::models::ParameterContextValidationRequestEntity get_validation_request(context_id, id)
Returns the Validation Request with the given ID

Returns the Validation Request with the given ID. Once a Validation Request has been created by performing a POST to /nifi-api/validation-contexts, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **String** | The ID of the Parameter Context | [required] |
**id** | **String** | The ID of the Validation Request | [required] |

### Return type

[**crate::models::ParameterContextValidationRequestEntity**](ParameterContextValidationRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_parameter_context_update

> crate::models::ParameterContextUpdateRequestEntity submit_parameter_context_update(context_id, body)
Initiate the Update Request of a Parameter Context

This will initiate the process of updating a Parameter Context. Changing the value of a Parameter may require that one or more components be stopped and restarted, so this action may take significantly more time than many other REST API actions. As a result, this endpoint will immediately return a ParameterContextUpdateRequestEntity, and the process of updating the necessary components will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /parameter-contexts/update-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /parameter-contexts/update-requests/{requestId}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **String** |  | [required] |
**body** | [**ParameterContextEntity**](ParameterContextEntity.md) | The updated version of the parameter context. | [required] |

### Return type

[**crate::models::ParameterContextUpdateRequestEntity**](ParameterContextUpdateRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_validation_request

> crate::models::ParameterContextValidationRequestEntity submit_validation_request(context_id, body)
Initiate a Validation Request to determine how the validity of components will change if a Parameter Context were to be updated

This will initiate the process of validating all components whose Process Group is bound to the specified Parameter Context. Performing validation against an arbitrary number of components may be expect and take significantly more time than many other REST API actions. As a result, this endpoint will immediately return a ParameterContextValidationRequestEntity, and the process of validating the necessary components will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /parameter-contexts/validation-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /parameter-contexts/validation-requests/{requestId}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **String** |  | [required] |
**body** | [**ParameterContextValidationRequestEntity**](ParameterContextValidationRequestEntity.md) | The validation request | [required] |

### Return type

[**crate::models::ParameterContextValidationRequestEntity**](ParameterContextValidationRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_parameter_context

> crate::models::ParameterContextEntity update_parameter_context(id, body)
Modifies a Parameter Context

This endpoint will update a Parameter Context to match the provided entity. However, this request will fail if any component is running and is referencing a Parameter in the Parameter Context. Generally, this endpoint is not called directly. Instead, an update request should be submitted by making a POST to the /parameter-contexts/update-requests endpoint. That endpoint will, in turn, call this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**ParameterContextEntity**](ParameterContextEntity.md) | The updated Parameter Context | [required] |

### Return type

[**crate::models::ParameterContextEntity**](ParameterContextEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

