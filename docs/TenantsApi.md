# \TenantsApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user**](TenantsApi.md#create_user) | **POST** /tenants/users | Creates a user
[**create_user_group**](TenantsApi.md#create_user_group) | **POST** /tenants/user-groups | Creates a user group
[**get_user**](TenantsApi.md#get_user) | **GET** /tenants/users/{id} | Gets a user
[**get_user_group**](TenantsApi.md#get_user_group) | **GET** /tenants/user-groups/{id} | Gets a user group
[**get_user_groups**](TenantsApi.md#get_user_groups) | **GET** /tenants/user-groups | Gets all user groups
[**get_users**](TenantsApi.md#get_users) | **GET** /tenants/users | Gets all users
[**remove_user**](TenantsApi.md#remove_user) | **DELETE** /tenants/users/{id} | Deletes a user
[**remove_user_group**](TenantsApi.md#remove_user_group) | **DELETE** /tenants/user-groups/{id} | Deletes a user group
[**search_tenants**](TenantsApi.md#search_tenants) | **GET** /tenants/search-results | Searches for a tenant with the specified identity
[**update_user**](TenantsApi.md#update_user) | **PUT** /tenants/users/{id} | Updates a user
[**update_user_group**](TenantsApi.md#update_user_group) | **PUT** /tenants/user-groups/{id} | Updates a user group



## create_user

> crate::models::UserEntity create_user(body)
Creates a user

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UserEntity**](UserEntity.md) | The user configuration details. | [required] |

### Return type

[**crate::models::UserEntity**](UserEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_group

> crate::models::UserGroupEntity create_user_group(body)
Creates a user group

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UserGroupEntity**](UserGroupEntity.md) | The user group configuration details. | [required] |

### Return type

[**crate::models::UserGroupEntity**](UserGroupEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> crate::models::UserEntity get_user(id)
Gets a user

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The user id. | [required] |

### Return type

[**crate::models::UserEntity**](UserEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_group

> crate::models::UserGroupEntity get_user_group(id)
Gets a user group

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The user group id. | [required] |

### Return type

[**crate::models::UserGroupEntity**](UserGroupEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_groups

> crate::models::UserGroupsEntity get_user_groups()
Gets all user groups

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserGroupsEntity**](UserGroupsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users

> crate::models::UsersEntity get_users()
Gets all users

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UsersEntity**](UsersEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user

> crate::models::UserEntity remove_user(id, version, client_id, disconnected_node_acknowledged)
Deletes a user

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The user id. | [required] |
**version** | Option<**String**> | The revision is used to verify the client is working with the latest version of the flow. |  |
**client_id** | Option<**String**> | If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response. |  |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::UserEntity**](UserEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_group

> crate::models::UserGroupEntity remove_user_group(id, version, client_id, disconnected_node_acknowledged)
Deletes a user group

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The user group id. | [required] |
**version** | Option<**String**> | The revision is used to verify the client is working with the latest version of the flow. |  |
**client_id** | Option<**String**> | If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response. |  |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::UserGroupEntity**](UserGroupEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_tenants

> crate::models::TenantsEntity search_tenants(q)
Searches for a tenant with the specified identity

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Identity to search for. | [required] |

### Return type

[**crate::models::TenantsEntity**](TenantsEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::UserEntity update_user(id, body)
Updates a user

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The user id. | [required] |
**body** | [**UserEntity**](UserEntity.md) | The user configuration details. | [required] |

### Return type

[**crate::models::UserEntity**](UserEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_group

> crate::models::UserGroupEntity update_user_group(id, body)
Updates a user group

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The user group id. | [required] |
**body** | [**UserGroupEntity**](UserGroupEntity.md) | The user group configuration details. | [required] |

### Return type

[**crate::models::UserGroupEntity**](UserGroupEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

