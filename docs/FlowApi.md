# \FlowApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activate_controller_services**](FlowApi.md#activate_controller_services) | **PUT** /flow/process-groups/{id}/controller-services | Enable or disable Controller Services in the specified Process Group.
[**generate_client_id**](FlowApi.md#generate_client_id) | **GET** /flow/client-id | Generates a client id.
[**get_about_info**](FlowApi.md#get_about_info) | **GET** /flow/about | Retrieves details about this NiFi to put in the About dialog
[**get_action**](FlowApi.md#get_action) | **GET** /flow/history/{id} | Gets an action
[**get_banners**](FlowApi.md#get_banners) | **GET** /flow/banners | Retrieves the banners for this NiFi
[**get_buckets**](FlowApi.md#get_buckets) | **GET** /flow/registries/{id}/buckets | Gets the buckets from the specified registry for the current user
[**get_bulletin_board**](FlowApi.md#get_bulletin_board) | **GET** /flow/bulletin-board | Gets current bulletins
[**get_bulletins**](FlowApi.md#get_bulletins) | **GET** /flow/controller/bulletins | Retrieves Controller level bulletins
[**get_cluster_summary**](FlowApi.md#get_cluster_summary) | **GET** /flow/cluster/summary | The cluster summary for this NiFi
[**get_component_history**](FlowApi.md#get_component_history) | **GET** /flow/history/components/{componentId} | Gets configuration history for a component
[**get_connection_statistics**](FlowApi.md#get_connection_statistics) | **GET** /flow/connections/{id}/statistics | Gets statistics for a connection
[**get_connection_status**](FlowApi.md#get_connection_status) | **GET** /flow/connections/{id}/status | Gets status for a connection
[**get_connection_status_history**](FlowApi.md#get_connection_status_history) | **GET** /flow/connections/{id}/status/history | Gets the status history for a connection
[**get_controller_service_types**](FlowApi.md#get_controller_service_types) | **GET** /flow/controller-service-types | Retrieves the types of controller services that this NiFi supports
[**get_controller_services_from_controller**](FlowApi.md#get_controller_services_from_controller) | **GET** /flow/controller/controller-services | Gets controller services for reporting tasks
[**get_controller_services_from_group**](FlowApi.md#get_controller_services_from_group) | **GET** /flow/process-groups/{id}/controller-services | Gets all controller services
[**get_controller_status**](FlowApi.md#get_controller_status) | **GET** /flow/status | Gets the current status of this NiFi
[**get_current_user**](FlowApi.md#get_current_user) | **GET** /flow/current-user | Retrieves the user identity of the user making the request
[**get_flow**](FlowApi.md#get_flow) | **GET** /flow/process-groups/{id} | Gets a process group
[**get_flow_config**](FlowApi.md#get_flow_config) | **GET** /flow/config | Retrieves the configuration for this NiFi flow
[**get_flow_metrics**](FlowApi.md#get_flow_metrics) | **GET** /flow/metrics/{producer} | Gets all metrics for the flow from a particular node
[**get_flows**](FlowApi.md#get_flows) | **GET** /flow/registries/{registry-id}/buckets/{bucket-id}/flows | Gets the flows from the specified registry and bucket for the current user
[**get_input_port_status**](FlowApi.md#get_input_port_status) | **GET** /flow/input-ports/{id}/status | Gets status for an input port
[**get_output_port_status**](FlowApi.md#get_output_port_status) | **GET** /flow/output-ports/{id}/status | Gets status for an output port
[**get_parameter_contexts**](FlowApi.md#get_parameter_contexts) | **GET** /flow/parameter-contexts | Gets all Parameter Contexts
[**get_prioritizers**](FlowApi.md#get_prioritizers) | **GET** /flow/prioritizers | Retrieves the types of prioritizers that this NiFi supports
[**get_process_group_status**](FlowApi.md#get_process_group_status) | **GET** /flow/process-groups/{id}/status | Gets the status for a process group
[**get_process_group_status_history**](FlowApi.md#get_process_group_status_history) | **GET** /flow/process-groups/{id}/status/history | Gets status history for a remote process group
[**get_processor_status**](FlowApi.md#get_processor_status) | **GET** /flow/processors/{id}/status | Gets status for a processor
[**get_processor_status_history**](FlowApi.md#get_processor_status_history) | **GET** /flow/processors/{id}/status/history | Gets status history for a processor
[**get_processor_types**](FlowApi.md#get_processor_types) | **GET** /flow/processor-types | Retrieves the types of processors that this NiFi supports
[**get_registries**](FlowApi.md#get_registries) | **GET** /flow/registries | Gets the listing of available registries
[**get_remote_process_group_status**](FlowApi.md#get_remote_process_group_status) | **GET** /flow/remote-process-groups/{id}/status | Gets status for a remote process group
[**get_remote_process_group_status_history**](FlowApi.md#get_remote_process_group_status_history) | **GET** /flow/remote-process-groups/{id}/status/history | Gets the status history
[**get_reporting_task_types**](FlowApi.md#get_reporting_task_types) | **GET** /flow/reporting-task-types | Retrieves the types of reporting tasks that this NiFi supports
[**get_reporting_tasks**](FlowApi.md#get_reporting_tasks) | **GET** /flow/reporting-tasks | Gets all reporting tasks
[**get_runtime_manifest**](FlowApi.md#get_runtime_manifest) | **GET** /flow/runtime-manifest | Retrieves the runtime manifest for this NiFi instance.
[**get_templates**](FlowApi.md#get_templates) | **GET** /flow/templates | Gets all templates
[**get_versions**](FlowApi.md#get_versions) | **GET** /flow/registries/{registry-id}/buckets/{bucket-id}/flows/{flow-id}/versions | Gets the flow versions from the specified registry and bucket for the specified flow for the current user
[**query_history**](FlowApi.md#query_history) | **GET** /flow/history | Gets configuration history
[**schedule_components**](FlowApi.md#schedule_components) | **PUT** /flow/process-groups/{id} | Schedule or unschedule components in the specified Process Group.
[**search_cluster**](FlowApi.md#search_cluster) | **GET** /flow/cluster/search-results | Searches the cluster for a node with the specified address
[**search_flow**](FlowApi.md#search_flow) | **GET** /flow/search-results | Performs a search against this NiFi using the specified search term



## activate_controller_services

> crate::models::ActivateControllerServicesEntity activate_controller_services(id, body)
Enable or disable Controller Services in the specified Process Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**ActivateControllerServicesEntity**](ActivateControllerServicesEntity.md) | The request to schedule or unschedule. If the comopnents in the request are not specified, all authorized components will be considered. | [required] |

### Return type

[**crate::models::ActivateControllerServicesEntity**](ActivateControllerServicesEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_client_id

> String generate_client_id()
Generates a client id.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_about_info

> crate::models::AboutEntity get_about_info()
Retrieves details about this NiFi to put in the About dialog

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AboutEntity**](AboutEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_action

> crate::models::ActionEntity get_action(id)
Gets an action

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The action id. | [required] |

### Return type

[**crate::models::ActionEntity**](ActionEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_banners

> crate::models::BannerEntity get_banners()
Retrieves the banners for this NiFi

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BannerEntity**](BannerEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_buckets

> crate::models::BucketsEntity get_buckets(id)
Gets the buckets from the specified registry for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The registry id. | [required] |

### Return type

[**crate::models::BucketsEntity**](BucketsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bulletin_board

> crate::models::BulletinBoardEntity get_bulletin_board(after, source_name, message, source_id, group_id, limit)
Gets current bulletins

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**after** | Option<**String**> | Includes bulletins with an id after this value. |  |
**source_name** | Option<**String**> | Includes bulletins originating from this sources whose name match this regular expression. |  |
**message** | Option<**String**> | Includes bulletins whose message that match this regular expression. |  |
**source_id** | Option<**String**> | Includes bulletins originating from this sources whose id match this regular expression. |  |
**group_id** | Option<**String**> | Includes bulletins originating from this sources whose group id match this regular expression. |  |
**limit** | Option<**String**> | The number of bulletins to limit the response to. |  |

### Return type

[**crate::models::BulletinBoardEntity**](BulletinBoardEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bulletins

> crate::models::ControllerBulletinsEntity get_bulletins()
Retrieves Controller level bulletins

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ControllerBulletinsEntity**](ControllerBulletinsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_summary

> crate::models::ClusteSummaryEntity get_cluster_summary()
The cluster summary for this NiFi

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ClusteSummaryEntity**](ClusteSummaryEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_component_history

> crate::models::ComponentHistoryEntity get_component_history(component_id)
Gets configuration history for a component

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**component_id** | **String** | The component id. | [required] |

### Return type

[**crate::models::ComponentHistoryEntity**](ComponentHistoryEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connection_statistics

> crate::models::ConnectionStatisticsEntity get_connection_statistics(id, nodewise, cluster_node_id)
Gets statistics for a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The connection id. | [required] |
**nodewise** | Option<**bool**> | Whether or not to include the breakdown per node. Optional, defaults to false |  |[default to false]
**cluster_node_id** | Option<**String**> | The id of the node where to get the statistics. |  |

### Return type

[**crate::models::ConnectionStatisticsEntity**](ConnectionStatisticsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connection_status

> crate::models::ConnectionStatusEntity get_connection_status(id, nodewise, cluster_node_id)
Gets status for a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The connection id. | [required] |
**nodewise** | Option<**bool**> | Whether or not to include the breakdown per node. Optional, defaults to false |  |[default to false]
**cluster_node_id** | Option<**String**> | The id of the node where to get the status. |  |

### Return type

[**crate::models::ConnectionStatusEntity**](ConnectionStatusEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connection_status_history

> crate::models::StatusHistoryEntity get_connection_status_history(id)
Gets the status history for a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The connection id. | [required] |

### Return type

[**crate::models::StatusHistoryEntity**](StatusHistoryEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_controller_service_types

> crate::models::ControllerServiceTypesEntity get_controller_service_types(service_type, service_bundle_group, service_bundle_artifact, service_bundle_version, bundle_group_filter, bundle_artifact_filter, type_filter)
Retrieves the types of controller services that this NiFi supports

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_type** | Option<**String**> | If specified, will only return controller services that are compatible with this type of service. |  |
**service_bundle_group** | Option<**String**> | If serviceType specified, is the bundle group of the serviceType. |  |
**service_bundle_artifact** | Option<**String**> | If serviceType specified, is the bundle artifact of the serviceType. |  |
**service_bundle_version** | Option<**String**> | If serviceType specified, is the bundle version of the serviceType. |  |
**bundle_group_filter** | Option<**String**> | If specified, will only return types that are a member of this bundle group. |  |
**bundle_artifact_filter** | Option<**String**> | If specified, will only return types that are a member of this bundle artifact. |  |
**type_filter** | Option<**String**> | If specified, will only return types whose fully qualified classname matches. |  |

### Return type

[**crate::models::ControllerServiceTypesEntity**](ControllerServiceTypesEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_controller_services_from_controller

> crate::models::ControllerServicesEntity get_controller_services_from_controller(ui_only)
Gets controller services for reporting tasks

If the uiOnly query parameter is provided with a value of true, the returned entity may only contain fields that are necessary for rendering the NiFi User Interface. As such, the selected fields may change at any time, even during incremental releases, without warning. As a result, this parameter should not be provided by any client other than the UI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ui_only** | Option<**bool**> |  |  |[default to false]

### Return type

[**crate::models::ControllerServicesEntity**](ControllerServicesEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_controller_services_from_group

> crate::models::ControllerServicesEntity get_controller_services_from_group(id, include_ancestor_groups, include_descendant_groups, ui_only)
Gets all controller services

If the uiOnly query parameter is provided with a value of true, the returned entity may only contain fields that are necessary for rendering the NiFi User Interface. As such, the selected fields may change at any time, even during incremental releases, without warning. As a result, this parameter should not be provided by any client other than the UI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**include_ancestor_groups** | Option<**bool**> | Whether or not to include parent/ancestory process groups |  |[default to true]
**include_descendant_groups** | Option<**bool**> | Whether or not to include descendant process groups |  |[default to false]
**ui_only** | Option<**bool**> |  |  |[default to false]

### Return type

[**crate::models::ControllerServicesEntity**](ControllerServicesEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_controller_status

> crate::models::ControllerStatusEntity get_controller_status()
Gets the current status of this NiFi

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ControllerStatusEntity**](ControllerStatusEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user

> crate::models::CurrentUserEntity get_current_user()
Retrieves the user identity of the user making the request

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CurrentUserEntity**](CurrentUserEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flow

> crate::models::ProcessGroupFlowEntity get_flow(id, ui_only)
Gets a process group

If the uiOnly query parameter is provided with a value of true, the returned entity may only contain fields that are necessary for rendering the NiFi User Interface. As such, the selected fields may change at any time, even during incremental releases, without warning. As a result, this parameter should not be provided by any client other than the UI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**ui_only** | Option<**bool**> |  |  |[default to false]

### Return type

[**crate::models::ProcessGroupFlowEntity**](ProcessGroupFlowEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flow_config

> crate::models::FlowConfigurationEntity get_flow_config()
Retrieves the configuration for this NiFi flow

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::FlowConfigurationEntity**](FlowConfigurationEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flow_metrics

> serde_json::Value get_flow_metrics(producer, included_registries, sample_name, sample_label_value, root_field_name)
Gets all metrics for the flow from a particular node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**producer** | **String** | The producer for flow file metrics. Each producer may have its own output format. | [required] |
**included_registries** | Option<[**Vec<String>**](String.md)> | Set of included metrics registries |  |
**sample_name** | Option<**String**> | Regular Expression Pattern to be applied against the sample name field |  |
**sample_label_value** | Option<**String**> | Regular Expression Pattern to be applied against the sample label value field |  |
**root_field_name** | Option<**String**> | Name of the first field of JSON object. Applicable for JSON producer only. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows

> crate::models::VersionedFlowsEntity get_flows(registry_id, bucket_id)
Gets the flows from the specified registry and bucket for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry_id** | **String** | The registry id. | [required] |
**bucket_id** | **String** | The bucket id. | [required] |

### Return type

[**crate::models::VersionedFlowsEntity**](VersionedFlowsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_input_port_status

> crate::models::PortStatusEntity get_input_port_status(id, nodewise, cluster_node_id)
Gets status for an input port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The input port id. | [required] |
**nodewise** | Option<**bool**> | Whether or not to include the breakdown per node. Optional, defaults to false |  |[default to false]
**cluster_node_id** | Option<**String**> | The id of the node where to get the status. |  |

### Return type

[**crate::models::PortStatusEntity**](PortStatusEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_output_port_status

> crate::models::PortStatusEntity get_output_port_status(id, nodewise, cluster_node_id)
Gets status for an output port

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The output port id. | [required] |
**nodewise** | Option<**bool**> | Whether or not to include the breakdown per node. Optional, defaults to false |  |[default to false]
**cluster_node_id** | Option<**String**> | The id of the node where to get the status. |  |

### Return type

[**crate::models::PortStatusEntity**](PortStatusEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_parameter_contexts

> crate::models::ParameterContextsEntity get_parameter_contexts()
Gets all Parameter Contexts

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ParameterContextsEntity**](ParameterContextsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_prioritizers

> crate::models::PrioritizerTypesEntity get_prioritizers()
Retrieves the types of prioritizers that this NiFi supports

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PrioritizerTypesEntity**](PrioritizerTypesEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_process_group_status

> crate::models::ProcessGroupStatusEntity get_process_group_status(id, recursive, nodewise, cluster_node_id)
Gets the status for a process group

The status for a process group includes status for all descendent components. When invoked on the root group with recursive set to true, it will return the current status of every component in the flow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**recursive** | Option<**bool**> | Whether all descendant groups and the status of their content will be included. Optional, defaults to false |  |[default to false]
**nodewise** | Option<**bool**> | Whether or not to include the breakdown per node. Optional, defaults to false |  |[default to false]
**cluster_node_id** | Option<**String**> | The id of the node where to get the status. |  |

### Return type

[**crate::models::ProcessGroupStatusEntity**](ProcessGroupStatusEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_process_group_status_history

> crate::models::StatusHistoryEntity get_process_group_status_history(id)
Gets status history for a remote process group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |

### Return type

[**crate::models::StatusHistoryEntity**](StatusHistoryEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_processor_status

> crate::models::ProcessorStatusEntity get_processor_status(id, nodewise, cluster_node_id)
Gets status for a processor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The processor id. | [required] |
**nodewise** | Option<**bool**> | Whether or not to include the breakdown per node. Optional, defaults to false |  |[default to false]
**cluster_node_id** | Option<**String**> | The id of the node where to get the status. |  |

### Return type

[**crate::models::ProcessorStatusEntity**](ProcessorStatusEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_processor_status_history

> crate::models::StatusHistoryEntity get_processor_status_history(id)
Gets status history for a processor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The processor id. | [required] |

### Return type

[**crate::models::StatusHistoryEntity**](StatusHistoryEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_processor_types

> crate::models::ProcessorTypesEntity get_processor_types(bundle_group_filter, bundle_artifact_filter, _type)
Retrieves the types of processors that this NiFi supports

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_group_filter** | Option<**String**> | If specified, will only return types that are a member of this bundle group. |  |
**bundle_artifact_filter** | Option<**String**> | If specified, will only return types that are a member of this bundle artifact. |  |
**_type** | Option<**String**> | If specified, will only return types whose fully qualified classname matches. |  |

### Return type

[**crate::models::ProcessorTypesEntity**](ProcessorTypesEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_registries

> crate::models::RegistryClientsEntity get_registries()
Gets the listing of available registries

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RegistryClientsEntity**](RegistryClientsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_remote_process_group_status

> crate::models::RemoteProcessGroupStatusEntity get_remote_process_group_status(id, nodewise, cluster_node_id)
Gets status for a remote process group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The remote process group id. | [required] |
**nodewise** | Option<**bool**> | Whether or not to include the breakdown per node. Optional, defaults to false |  |[default to false]
**cluster_node_id** | Option<**String**> | The id of the node where to get the status. |  |

### Return type

[**crate::models::RemoteProcessGroupStatusEntity**](RemoteProcessGroupStatusEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_remote_process_group_status_history

> crate::models::StatusHistoryEntity get_remote_process_group_status_history(id)
Gets the status history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The remote process group id. | [required] |

### Return type

[**crate::models::StatusHistoryEntity**](StatusHistoryEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reporting_task_types

> crate::models::ReportingTaskTypesEntity get_reporting_task_types(bundle_group_filter, bundle_artifact_filter, _type)
Retrieves the types of reporting tasks that this NiFi supports

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_group_filter** | Option<**String**> | If specified, will only return types that are a member of this bundle group. |  |
**bundle_artifact_filter** | Option<**String**> | If specified, will only return types that are a member of this bundle artifact. |  |
**_type** | Option<**String**> | If specified, will only return types whose fully qualified classname matches. |  |

### Return type

[**crate::models::ReportingTaskTypesEntity**](ReportingTaskTypesEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reporting_tasks

> crate::models::ReportingTasksEntity get_reporting_tasks()
Gets all reporting tasks

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ReportingTasksEntity**](ReportingTasksEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_runtime_manifest

> crate::models::RuntimeManifestEntity get_runtime_manifest()
Retrieves the runtime manifest for this NiFi instance.

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RuntimeManifestEntity**](RuntimeManifestEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_templates

> crate::models::TemplatesEntity get_templates()
Gets all templates

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TemplatesEntity**](TemplatesEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_versions

> crate::models::VersionedFlowSnapshotMetadataSetEntity get_versions(registry_id, bucket_id, flow_id)
Gets the flow versions from the specified registry and bucket for the specified flow for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry_id** | **String** | The registry id. | [required] |
**bucket_id** | **String** | The bucket id. | [required] |
**flow_id** | **String** | The flow id. | [required] |

### Return type

[**crate::models::VersionedFlowSnapshotMetadataSetEntity**](VersionedFlowSnapshotMetadataSetEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_history

> crate::models::HistoryEntity query_history(offset, count, sort_column, sort_order, start_date, end_date, user_identity, source_id)
Gets configuration history

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | **String** | The offset into the result set. | [required] |
**count** | **String** | The number of actions to return. | [required] |
**sort_column** | Option<**String**> | The field to sort on. |  |
**sort_order** | Option<**String**> | The direction to sort. |  |
**start_date** | Option<**String**> | Include actions after this date. |  |
**end_date** | Option<**String**> | Include actions before this date. |  |
**user_identity** | Option<**String**> | Include actions performed by this user. |  |
**source_id** | Option<**String**> | Include actions on this component. |  |

### Return type

[**crate::models::HistoryEntity**](HistoryEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schedule_components

> crate::models::ScheduleComponentsEntity schedule_components(id, body)
Schedule or unschedule components in the specified Process Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The process group id. | [required] |
**body** | [**ScheduleComponentsEntity**](ScheduleComponentsEntity.md) | The request to schedule or unschedule. If the comopnents in the request are not specified, all authorized components will be considered. | [required] |

### Return type

[**crate::models::ScheduleComponentsEntity**](ScheduleComponentsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_cluster

> crate::models::ClusterSearchResultsEntity search_cluster(q)
Searches the cluster for a node with the specified address

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Node address to search for. | [required] |

### Return type

[**crate::models::ClusterSearchResultsEntity**](ClusterSearchResultsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_flow

> crate::models::SearchResultsEntity search_flow(q, a)
Performs a search against this NiFi using the specified search term

Only search results from authorized components will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> |  |  |
**a** | Option<**String**> |  |  |

### Return type

[**crate::models::SearchResultsEntity**](SearchResultsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

