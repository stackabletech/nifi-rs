# \ControllerServicesApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**analyze_configuration**](ControllerServicesApi.md#analyze_configuration) | **POST** /controller-services/{id}/config/analysis | Performs analysis of the component's configuration, providing information about which attributes are referenced.
[**clear_state**](ControllerServicesApi.md#clear_state) | **POST** /controller-services/{id}/state/clear-requests | Clears the state for a controller service
[**delete_validation_request**](ControllerServicesApi.md#delete_validation_request) | **DELETE** /controller-services/{id}/config/verification-requests/{requestId} | Deletes the Verification Request with the given ID
[**get_controller_service**](ControllerServicesApi.md#get_controller_service) | **GET** /controller-services/{id} | Gets a controller service
[**get_controller_service_references**](ControllerServicesApi.md#get_controller_service_references) | **GET** /controller-services/{id}/references | Gets a controller service
[**get_property_descriptor**](ControllerServicesApi.md#get_property_descriptor) | **GET** /controller-services/{id}/descriptors | Gets a controller service property descriptor
[**get_state**](ControllerServicesApi.md#get_state) | **GET** /controller-services/{id}/state | Gets the state for a controller service
[**get_verification_request**](ControllerServicesApi.md#get_verification_request) | **GET** /controller-services/{id}/config/verification-requests/{requestId} | Returns the Verification Request with the given ID
[**remove_controller_service**](ControllerServicesApi.md#remove_controller_service) | **DELETE** /controller-services/{id} | Deletes a controller service
[**submit_config_verification_request**](ControllerServicesApi.md#submit_config_verification_request) | **POST** /controller-services/{id}/config/verification-requests | Performs verification of the Controller Service's configuration
[**update_controller_service**](ControllerServicesApi.md#update_controller_service) | **PUT** /controller-services/{id} | Updates a controller service
[**update_controller_service_references**](ControllerServicesApi.md#update_controller_service_references) | **PUT** /controller-services/{id}/references | Updates a controller services references
[**update_run_status**](ControllerServicesApi.md#update_run_status) | **PUT** /controller-services/{id}/run-status | Updates run status of a controller service



## analyze_configuration

> crate::models::ConfigurationAnalysisEntity analyze_configuration(id, body)
Performs analysis of the component's configuration, providing information about which attributes are referenced.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The controller service id. | [required] |
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
Clears the state for a controller service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The controller service id. | [required] |

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
**id** | **String** | The ID of the Controller Service | [required] |
**request_id** | **String** | The ID of the Verification Request | [required] |

### Return type

[**crate::models::VerifyConfigRequestEntity**](VerifyConfigRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_controller_service

> crate::models::ControllerServiceEntity get_controller_service(id, ui_only)
Gets a controller service

If the uiOnly query parameter is provided with a value of true, the returned entity may only contain fields that are necessary for rendering the NiFi User Interface. As such, the selected fields may change at any time, even during incremental releases, without warning. As a result, this parameter should not be provided by any client other than the UI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The controller service id. | [required] |
**ui_only** | Option<**bool**> |  |  |[default to false]

### Return type

[**crate::models::ControllerServiceEntity**](ControllerServiceEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_controller_service_references

> crate::models::ControllerServiceReferencingComponentsEntity get_controller_service_references(id)
Gets a controller service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The controller service id. | [required] |

### Return type

[**crate::models::ControllerServiceReferencingComponentsEntity**](ControllerServiceReferencingComponentsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_property_descriptor

> crate::models::PropertyDescriptorEntity get_property_descriptor(id, property_name)
Gets a controller service property descriptor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The controller service id. | [required] |
**property_name** | **String** | The property name to return the descriptor for. | [required] |

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
Gets the state for a controller service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The controller service id. | [required] |

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
**id** | **String** | The ID of the Controller Service | [required] |
**request_id** | **String** | The ID of the Verification Request | [required] |

### Return type

[**crate::models::VerifyConfigRequestEntity**](VerifyConfigRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_controller_service

> crate::models::ControllerServiceEntity remove_controller_service(id, version, client_id, disconnected_node_acknowledged)
Deletes a controller service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The controller service id. | [required] |
**version** | Option<**String**> | The revision is used to verify the client is working with the latest version of the flow. |  |
**client_id** | Option<**String**> | If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response. |  |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::ControllerServiceEntity**](ControllerServiceEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_config_verification_request

> crate::models::VerifyConfigRequestEntity submit_config_verification_request(id, body)
Performs verification of the Controller Service's configuration

This will initiate the process of verifying a given Controller Service configuration. This may be a long-running task. As a result, this endpoint will immediately return a ControllerServiceConfigVerificationRequestEntity, and the process of performing the verification will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /controller-services/{serviceId}/verification-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /controller-services/{serviceId}/verification-requests/{requestId}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The controller service id. | [required] |
**body** | [**VerifyConfigRequestEntity**](VerifyConfigRequestEntity.md) | The controller service configuration verification request. | [required] |

### Return type

[**crate::models::VerifyConfigRequestEntity**](VerifyConfigRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_controller_service

> crate::models::ControllerServiceEntity update_controller_service(id, body)
Updates a controller service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The controller service id. | [required] |
**body** | [**ControllerServiceEntity**](ControllerServiceEntity.md) | The controller service configuration details. | [required] |

### Return type

[**crate::models::ControllerServiceEntity**](ControllerServiceEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_controller_service_references

> crate::models::ControllerServiceReferencingComponentsEntity update_controller_service_references(id, body)
Updates a controller services references

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The controller service id. | [required] |
**body** | [**UpdateControllerServiceReferenceRequestEntity**](UpdateControllerServiceReferenceRequestEntity.md) | The controller service request update request. | [required] |

### Return type

[**crate::models::ControllerServiceReferencingComponentsEntity**](ControllerServiceReferencingComponentsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_run_status

> crate::models::ControllerServiceEntity update_run_status(id, body)
Updates run status of a controller service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The controller service id. | [required] |
**body** | [**ControllerServiceRunStatusEntity**](ControllerServiceRunStatusEntity.md) | The controller service run status. | [required] |

### Return type

[**crate::models::ControllerServiceEntity**](ControllerServiceEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

