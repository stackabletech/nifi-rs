# \ControllerApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_bulletin**](ControllerApi.md#create_bulletin) | **POST** /controller/bulletin | Creates a new bulletin
[**create_controller_service**](ControllerApi.md#create_controller_service) | **POST** /controller/controller-services | Creates a new controller service
[**create_registry_client**](ControllerApi.md#create_registry_client) | **POST** /controller/registry-clients | Creates a new registry client
[**create_reporting_task**](ControllerApi.md#create_reporting_task) | **POST** /controller/reporting-tasks | Creates a new reporting task
[**delete_history**](ControllerApi.md#delete_history) | **DELETE** /controller/history | Purges history
[**delete_node**](ControllerApi.md#delete_node) | **DELETE** /controller/cluster/nodes/{id} | Removes a node from the cluster
[**delete_registry_client**](ControllerApi.md#delete_registry_client) | **DELETE** /controller/registry-clients/{id} | Deletes a registry client
[**get_cluster**](ControllerApi.md#get_cluster) | **GET** /controller/cluster | Gets the contents of the cluster
[**get_controller_config**](ControllerApi.md#get_controller_config) | **GET** /controller/config | Retrieves the configuration for this NiFi Controller
[**get_node**](ControllerApi.md#get_node) | **GET** /controller/cluster/nodes/{id} | Gets a node in the cluster
[**get_node_status_history**](ControllerApi.md#get_node_status_history) | **GET** /controller/status/history | Gets status history for the node
[**get_registry_client**](ControllerApi.md#get_registry_client) | **GET** /controller/registry-clients/{id} | Gets a registry client
[**get_registry_clients**](ControllerApi.md#get_registry_clients) | **GET** /controller/registry-clients | Gets the listing of available registry clients
[**update_controller_config**](ControllerApi.md#update_controller_config) | **PUT** /controller/config | Retrieves the configuration for this NiFi
[**update_node**](ControllerApi.md#update_node) | **PUT** /controller/cluster/nodes/{id} | Updates a node in the cluster
[**update_registry_client**](ControllerApi.md#update_registry_client) | **PUT** /controller/registry-clients/{id} | Updates a registry client



## create_bulletin

> crate::models::BulletinEntity create_bulletin(body)
Creates a new bulletin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BulletinEntity**](BulletinEntity.md) | The reporting task configuration details. | [required] |

### Return type

[**crate::models::BulletinEntity**](BulletinEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_controller_service

> crate::models::ControllerServiceEntity create_controller_service(body)
Creates a new controller service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ControllerServiceEntity**](ControllerServiceEntity.md) | The controller service configuration details. | [required] |

### Return type

[**crate::models::ControllerServiceEntity**](ControllerServiceEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_registry_client

> crate::models::RegistryClientEntity create_registry_client(body)
Creates a new registry client

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RegistryClientEntity**](RegistryClientEntity.md) | The registry configuration details. | [required] |

### Return type

[**crate::models::RegistryClientEntity**](RegistryClientEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_reporting_task

> crate::models::ReportingTaskEntity create_reporting_task(body)
Creates a new reporting task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ReportingTaskEntity**](ReportingTaskEntity.md) | The reporting task configuration details. | [required] |

### Return type

[**crate::models::ReportingTaskEntity**](ReportingTaskEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_history

> crate::models::HistoryEntity delete_history(end_date)
Purges history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**end_date** | **String** | Purge actions before this date/time. | [required] |

### Return type

[**crate::models::HistoryEntity**](HistoryEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_node

> crate::models::NodeEntity delete_node(id)
Removes a node from the cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The node id. | [required] |

### Return type

[**crate::models::NodeEntity**](NodeEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_registry_client

> crate::models::RegistryClientEntity delete_registry_client(id, version, client_id, disconnected_node_acknowledged)
Deletes a registry client

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The registry id. | [required] |
**version** | Option<**String**> | The revision is used to verify the client is working with the latest version of the flow. |  |
**client_id** | Option<**String**> | If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response. |  |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::RegistryClientEntity**](RegistryClientEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster

> crate::models::ClusterEntity get_cluster()
Gets the contents of the cluster

Returns the contents of the cluster including all nodes and their status.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ClusterEntity**](ClusterEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_controller_config

> crate::models::ControllerConfigurationEntity get_controller_config()
Retrieves the configuration for this NiFi Controller

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ControllerConfigurationEntity**](ControllerConfigurationEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_node

> crate::models::NodeEntity get_node(id)
Gets a node in the cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The node id. | [required] |

### Return type

[**crate::models::NodeEntity**](NodeEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_node_status_history

> crate::models::ComponentHistoryEntity get_node_status_history()
Gets status history for the node

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ComponentHistoryEntity**](ComponentHistoryEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_registry_client

> crate::models::RegistryClientEntity get_registry_client(id)
Gets a registry client

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The registry id. | [required] |

### Return type

[**crate::models::RegistryClientEntity**](RegistryClientEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_registry_clients

> crate::models::RegistryClientsEntity get_registry_clients()
Gets the listing of available registry clients

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


## update_controller_config

> crate::models::ControllerConfigurationEntity update_controller_config(body)
Retrieves the configuration for this NiFi

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ControllerConfigurationEntity**](ControllerConfigurationEntity.md) | The controller configuration. | [required] |

### Return type

[**crate::models::ControllerConfigurationEntity**](ControllerConfigurationEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_node

> crate::models::NodeEntity update_node(id, body)
Updates a node in the cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The node id. | [required] |
**body** | [**NodeEntity**](NodeEntity.md) | The node configuration. The only configuration that will be honored at this endpoint is the status. | [required] |

### Return type

[**crate::models::NodeEntity**](NodeEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_registry_client

> crate::models::RegistryClientEntity update_registry_client(id, body)
Updates a registry client

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The registry id. | [required] |
**body** | [**RegistryClientEntity**](RegistryClientEntity.md) | The registry configuration details. | [required] |

### Return type

[**crate::models::RegistryClientEntity**](RegistryClientEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

