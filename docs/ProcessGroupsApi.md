# \ProcessGroupsApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**copy_snippet**](ProcessGroupsApi.md#copy_snippet) | **POST** /process-groups/{id}/snippet-instance | Copies a snippet and discards it.
[**create_connection**](ProcessGroupsApi.md#create_connection) | **POST** /process-groups/{id}/connections | Creates a connection
[**create_controller_service**](ProcessGroupsApi.md#create_controller_service) | **POST** /process-groups/{id}/controller-services | Creates a new controller service
[**create_empty_all_connections_request**](ProcessGroupsApi.md#create_empty_all_connections_request) | **POST** /process-groups/{id}/empty-all-connections-requests | Creates a request to drop all flowfiles of all connection queues in this process group.
[**create_funnel**](ProcessGroupsApi.md#create_funnel) | **POST** /process-groups/{id}/funnels | Creates a funnel
[**create_input_port**](ProcessGroupsApi.md#create_input_port) | **POST** /process-groups/{id}/input-ports | Creates an input port
[**create_label**](ProcessGroupsApi.md#create_label) | **POST** /process-groups/{id}/labels | Creates a label
[**create_output_port**](ProcessGroupsApi.md#create_output_port) | **POST** /process-groups/{id}/output-ports | Creates an output port
[**create_process_group**](ProcessGroupsApi.md#create_process_group) | **POST** /process-groups/{id}/process-groups | Creates a process group
[**create_processor**](ProcessGroupsApi.md#create_processor) | **POST** /process-groups/{id}/processors | Creates a new processor
[**create_remote_process_group**](ProcessGroupsApi.md#create_remote_process_group) | **POST** /process-groups/{id}/remote-process-groups | Creates a new process group
[**create_template**](ProcessGroupsApi.md#create_template) | **POST** /process-groups/{id}/templates | Creates a template and discards the specified snippet.
[**delete_replace_process_group_request**](ProcessGroupsApi.md#delete_replace_process_group_request) | **DELETE** /process-groups/replace-requests/{id} | Deletes the Replace Request with the given ID
[**delete_variable_registry_update_request**](ProcessGroupsApi.md#delete_variable_registry_update_request) | **DELETE** /process-groups/{groupId}/variable-registry/update-requests/{updateId} | Deletes an update request for a process group's variable registry. If the request is not yet complete, it will automatically be cancelled.
[**export_process_group**](ProcessGroupsApi.md#export_process_group) | **GET** /process-groups/{id}/download | Gets a process group for download
[**get_connections**](ProcessGroupsApi.md#get_connections) | **GET** /process-groups/{id}/connections | Gets all connections
[**get_drop_all_flowfiles_request**](ProcessGroupsApi.md#get_drop_all_flowfiles_request) | **GET** /process-groups/{id}/empty-all-connections-requests/{drop-request-id} | Gets the current status of a drop all flowfiles request.
[**get_funnels**](ProcessGroupsApi.md#get_funnels) | **GET** /process-groups/{id}/funnels | Gets all funnels
[**get_input_ports**](ProcessGroupsApi.md#get_input_ports) | **GET** /process-groups/{id}/input-ports | Gets all input ports
[**get_labels**](ProcessGroupsApi.md#get_labels) | **GET** /process-groups/{id}/labels | Gets all labels
[**get_local_modifications**](ProcessGroupsApi.md#get_local_modifications) | **GET** /process-groups/{id}/local-modifications | Gets a list of local modifications to the Process Group since it was last synchronized with the Flow Registry
[**get_output_ports**](ProcessGroupsApi.md#get_output_ports) | **GET** /process-groups/{id}/output-ports | Gets all output ports
[**get_process_group**](ProcessGroupsApi.md#get_process_group) | **GET** /process-groups/{id} | Gets a process group
[**get_process_groups**](ProcessGroupsApi.md#get_process_groups) | **GET** /process-groups/{id}/process-groups | Gets all process groups
[**get_processors**](ProcessGroupsApi.md#get_processors) | **GET** /process-groups/{id}/processors | Gets all processors
[**get_remote_process_groups**](ProcessGroupsApi.md#get_remote_process_groups) | **GET** /process-groups/{id}/remote-process-groups | Gets all remote process groups
[**get_replace_process_group_request**](ProcessGroupsApi.md#get_replace_process_group_request) | **GET** /process-groups/replace-requests/{id} | Returns the Replace Request with the given ID
[**get_variable_registry**](ProcessGroupsApi.md#get_variable_registry) | **GET** /process-groups/{id}/variable-registry | Gets a process group's variable registry
[**get_variable_registry_update_request**](ProcessGroupsApi.md#get_variable_registry_update_request) | **GET** /process-groups/{groupId}/variable-registry/update-requests/{updateId} | Gets a process group's variable registry
[**import_process_group**](ProcessGroupsApi.md#import_process_group) | **POST** /process-groups/{id}/process-groups/import | Imports a specified process group
[**import_template**](ProcessGroupsApi.md#import_template) | **POST** /process-groups/{id}/templates/import | Imports a template
[**initiate_replace_process_group**](ProcessGroupsApi.md#initiate_replace_process_group) | **POST** /process-groups/{id}/replace-requests | Initiate the Replace Request of a Process Group with the given ID
[**instantiate_template**](ProcessGroupsApi.md#instantiate_template) | **POST** /process-groups/{id}/template-instance | Instantiates a template
[**remove_drop_request**](ProcessGroupsApi.md#remove_drop_request) | **DELETE** /process-groups/{id}/empty-all-connections-requests/{drop-request-id} | Cancels and/or removes a request to drop all flowfiles.
[**remove_process_group**](ProcessGroupsApi.md#remove_process_group) | **DELETE** /process-groups/{id} | Deletes a process group
[**replace_process_group**](ProcessGroupsApi.md#replace_process_group) | **PUT** /process-groups/{id}/flow-contents | Replace Process Group contents with the given ID with the specified Process Group contents
[**submit_update_variable_registry_request**](ProcessGroupsApi.md#submit_update_variable_registry_request) | **POST** /process-groups/{id}/variable-registry/update-requests | Submits a request to update a process group's variable registry
[**update_process_group**](ProcessGroupsApi.md#update_process_group) | **PUT** /process-groups/{id} | Updates a process group
[**update_variable_registry**](ProcessGroupsApi.md#update_variable_registry) | **PUT** /process-groups/{id}/variable-registry | Updates the contents of a Process Group's variable Registry
[**upload_process_group**](ProcessGroupsApi.md#upload_process_group) | **POST** /process-groups/{id}/process-groups/upload | Uploads a versioned flow definition and creates a process group
[**upload_template**](ProcessGroupsApi.md#upload_template) | **POST** /process-groups/{id}/templates/upload | Uploads a template



## copy_snippet

> crate::models::FlowEntity copy_snippet(id, body)
Copies a snippet and discards it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**CopySnippetRequestEntity**](CopySnippetRequestEntity.md) | The copy snippet request. | [required] |

### Return type

[**crate::models::FlowEntity**](FlowEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_connection

> crate::models::ConnectionEntity create_connection(id, body)
Creates a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**ConnectionEntity**](ConnectionEntity.md) | The connection configuration details. | [required] |

### Return type

[**crate::models::ConnectionEntity**](ConnectionEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_controller_service

> crate::models::ControllerServiceEntity create_controller_service(id, body)
Creates a new controller service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**ControllerServiceEntity**](ControllerServiceEntity.md) | The controller service configuration details. | [required] |

### Return type

[**crate::models::ControllerServiceEntity**](ControllerServiceEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_empty_all_connections_request

> crate::models::ProcessGroupEntity create_empty_all_connections_request(id)
Creates a request to drop all flowfiles of all connection queues in this process group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |

### Return type

[**crate::models::ProcessGroupEntity**](ProcessGroupEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_funnel

> crate::models::FunnelEntity create_funnel(id, body)
Creates a funnel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**FunnelEntity**](FunnelEntity.md) | The funnel configuration details. | [required] |

### Return type

[**crate::models::FunnelEntity**](FunnelEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_input_port

> crate::models::PortEntity create_input_port(id, body)
Creates an input port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**PortEntity**](PortEntity.md) | The input port configuration details. | [required] |

### Return type

[**crate::models::PortEntity**](PortEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_label

> crate::models::LabelEntity create_label(id, body)
Creates a label

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**LabelEntity**](LabelEntity.md) | The label configuration details. | [required] |

### Return type

[**crate::models::LabelEntity**](LabelEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_output_port

> crate::models::PortEntity create_output_port(id, body)
Creates an output port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**PortEntity**](PortEntity.md) | The output port configuration. | [required] |

### Return type

[**crate::models::PortEntity**](PortEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_process_group

> crate::models::ProcessGroupEntity create_process_group(id, body)
Creates a process group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**ProcessGroupEntity**](ProcessGroupEntity.md) | The process group configuration details. | [required] |

### Return type

[**crate::models::ProcessGroupEntity**](ProcessGroupEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_processor

> crate::models::ProcessorEntity create_processor(id, body)
Creates a new processor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**ProcessorEntity**](ProcessorEntity.md) | The processor configuration details. | [required] |

### Return type

[**crate::models::ProcessorEntity**](ProcessorEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_remote_process_group

> crate::models::RemoteProcessGroupEntity create_remote_process_group(id, body)
Creates a new process group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**RemoteProcessGroupEntity**](RemoteProcessGroupEntity.md) | The remote process group configuration details. | [required] |

### Return type

[**crate::models::RemoteProcessGroupEntity**](RemoteProcessGroupEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_template

> crate::models::TemplateEntity create_template(id, body)
Creates a template and discards the specified snippet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**CreateTemplateRequestEntity**](CreateTemplateRequestEntity.md) | The create template request. | [required] |

### Return type

[**crate::models::TemplateEntity**](TemplateEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_replace_process_group_request

> crate::models::ProcessGroupReplaceRequestEntity delete_replace_process_group_request(id, disconnected_node_acknowledged)
Deletes the Replace Request with the given ID

Deletes the Replace Request with the given ID. After a request is created via a POST to /process-groups/{id}/replace-requests, it is expected that the client will properly clean up the request by DELETE'ing it, once the Replace process has completed. If the request is deleted before the request completes, then the Replace request will finish the step that it is currently performing and then will cancel any subsequent steps. Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the Update Request | [required] |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::ProcessGroupReplaceRequestEntity**](ProcessGroupReplaceRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_variable_registry_update_request

> crate::models::VariableRegistryUpdateRequestEntity delete_variable_registry_update_request(group_id, update_id, disconnected_node_acknowledged)
Deletes an update request for a process group's variable registry. If the request is not yet complete, it will automatically be cancelled.

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The process group id. | [required] |
**update_id** | **String** | The ID of the Variable Registry Update Request | [required] |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::VariableRegistryUpdateRequestEntity**](VariableRegistryUpdateRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_process_group

> String export_process_group(id)
Gets a process group for download

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


## get_connections

> crate::models::ConnectionsEntity get_connections(id)
Gets all connections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |

### Return type

[**crate::models::ConnectionsEntity**](ConnectionsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_drop_all_flowfiles_request

> crate::models::DropRequestEntity get_drop_all_flowfiles_request(id, drop_request_id)
Gets the current status of a drop all flowfiles request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**drop_request_id** | **String** | The drop request id. | [required] |

### Return type

[**crate::models::DropRequestEntity**](DropRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_funnels

> crate::models::FunnelsEntity get_funnels(id)
Gets all funnels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |

### Return type

[**crate::models::FunnelsEntity**](FunnelsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_input_ports

> crate::models::InputPortsEntity get_input_ports(id)
Gets all input ports

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |

### Return type

[**crate::models::InputPortsEntity**](InputPortsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_labels

> crate::models::LabelsEntity get_labels(id)
Gets all labels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |

### Return type

[**crate::models::LabelsEntity**](LabelsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_local_modifications

> crate::models::FlowComparisonEntity get_local_modifications(id)
Gets a list of local modifications to the Process Group since it was last synchronized with the Flow Registry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |

### Return type

[**crate::models::FlowComparisonEntity**](FlowComparisonEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_output_ports

> crate::models::OutputPortsEntity get_output_ports(id)
Gets all output ports

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |

### Return type

[**crate::models::OutputPortsEntity**](OutputPortsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_process_group

> crate::models::ProcessGroupEntity get_process_group(id)
Gets a process group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |

### Return type

[**crate::models::ProcessGroupEntity**](ProcessGroupEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_process_groups

> crate::models::ProcessGroupsEntity get_process_groups(id)
Gets all process groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |

### Return type

[**crate::models::ProcessGroupsEntity**](ProcessGroupsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_processors

> crate::models::ProcessorsEntity get_processors(id, include_descendant_groups)
Gets all processors

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**include_descendant_groups** | Option<**bool**> | Whether or not to include processors from descendant process groups |  |[default to false]

### Return type

[**crate::models::ProcessorsEntity**](ProcessorsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_remote_process_groups

> crate::models::RemoteProcessGroupsEntity get_remote_process_groups(id)
Gets all remote process groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |

### Return type

[**crate::models::RemoteProcessGroupsEntity**](RemoteProcessGroupsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_replace_process_group_request

> crate::models::ProcessGroupReplaceRequestEntity get_replace_process_group_request(id)
Returns the Replace Request with the given ID

Returns the Replace Request with the given ID. Once a Replace Request has been created by performing a POST to /process-groups/{id}/replace-requests, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures. Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the Replace Request | [required] |

### Return type

[**crate::models::ProcessGroupReplaceRequestEntity**](ProcessGroupReplaceRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_variable_registry

> crate::models::VariableRegistryEntity get_variable_registry(id, include_ancestor_groups)
Gets a process group's variable registry

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**include_ancestor_groups** | Option<**bool**> | Whether or not to include ancestor groups |  |[default to true]

### Return type

[**crate::models::VariableRegistryEntity**](VariableRegistryEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_variable_registry_update_request

> crate::models::VariableRegistryUpdateRequestEntity get_variable_registry_update_request(group_id, update_id)
Gets a process group's variable registry

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The process group id. | [required] |
**update_id** | **String** | The ID of the Variable Registry Update Request | [required] |

### Return type

[**crate::models::VariableRegistryUpdateRequestEntity**](VariableRegistryUpdateRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_process_group

> crate::models::ProcessGroupEntity import_process_group(id)
Imports a specified process group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |

### Return type

[**crate::models::ProcessGroupEntity**](ProcessGroupEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_template

> crate::models::TemplateEntity import_template(id)
Imports a template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |

### Return type

[**crate::models::TemplateEntity**](TemplateEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## initiate_replace_process_group

> crate::models::ProcessGroupReplaceRequestEntity initiate_replace_process_group(id, body)
Initiate the Replace Request of a Process Group with the given ID

This will initiate the action of replacing a process group with the given process group. This can be a lengthy process, as it will stop any Processors and disable any Controller Services necessary to perform the action and then restart them. As a result, the endpoint will immediately return a ProcessGroupReplaceRequestEntity, and the process of replacing the flow will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /process-groups/replace-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /process-groups/replace-requests/{requestId}. Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**ProcessGroupImportEntity**](ProcessGroupImportEntity.md) | The process group replace request entity | [required] |

### Return type

[**crate::models::ProcessGroupReplaceRequestEntity**](ProcessGroupReplaceRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instantiate_template

> crate::models::FlowEntity instantiate_template(id, body)
Instantiates a template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**InstantiateTemplateRequestEntity**](InstantiateTemplateRequestEntity.md) | The instantiate template request. | [required] |

### Return type

[**crate::models::FlowEntity**](FlowEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_drop_request

> crate::models::DropRequestEntity remove_drop_request(id, drop_request_id)
Cancels and/or removes a request to drop all flowfiles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**drop_request_id** | **String** | The drop request id. | [required] |

### Return type

[**crate::models::DropRequestEntity**](DropRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_process_group

> crate::models::ProcessGroupEntity remove_process_group(id, version, client_id, disconnected_node_acknowledged)
Deletes a process group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**version** | Option<**String**> | The revision is used to verify the client is working with the latest version of the flow. |  |
**client_id** | Option<**String**> | If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response. |  |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::ProcessGroupEntity**](ProcessGroupEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_process_group

> crate::models::ProcessGroupImportEntity replace_process_group(id, body)
Replace Process Group contents with the given ID with the specified Process Group contents

This endpoint is used for replication within a cluster, when replacing a flow with a new flow. It expects that the flow beingreplaced is not under version control and that the given snapshot will not modify any Processor that is currently running or any Controller Service that is enabled. Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**ProcessGroupImportEntity**](ProcessGroupImportEntity.md) | The process group replace request entity. | [required] |

### Return type

[**crate::models::ProcessGroupImportEntity**](ProcessGroupImportEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_update_variable_registry_request

> crate::models::VariableRegistryUpdateRequestEntity submit_update_variable_registry_request(id, body)
Submits a request to update a process group's variable registry

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**VariableRegistryEntity**](VariableRegistryEntity.md) | The variable registry configuration details. | [required] |

### Return type

[**crate::models::VariableRegistryUpdateRequestEntity**](VariableRegistryUpdateRequestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_process_group

> crate::models::ProcessGroupEntity update_process_group(id, body)
Updates a process group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**ProcessGroupEntity**](ProcessGroupEntity.md) | The process group configuration details. | [required] |

### Return type

[**crate::models::ProcessGroupEntity**](ProcessGroupEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_variable_registry

> crate::models::VariableRegistryEntity update_variable_registry(id, body)
Updates the contents of a Process Group's variable Registry

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**VariableRegistryEntity**](VariableRegistryEntity.md) | The variable registry configuration details. | [required] |

### Return type

[**crate::models::VariableRegistryEntity**](VariableRegistryEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_process_group

> crate::models::ProcessGroupEntity upload_process_group(id)
Uploads a versioned flow definition and creates a process group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |

### Return type

[**crate::models::ProcessGroupEntity**](ProcessGroupEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_template

> crate::models::TemplateEntity upload_template(id, template)
Uploads a template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**template** | **std::path::PathBuf** | The binary content of the template file being uploaded. | [required] |

### Return type

[**crate::models::TemplateEntity**](TemplateEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

