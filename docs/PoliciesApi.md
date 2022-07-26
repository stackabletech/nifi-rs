# \PoliciesApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_access_policy**](PoliciesApi.md#create_access_policy) | **POST** /policies | Creates an access policy
[**get_access_policy**](PoliciesApi.md#get_access_policy) | **GET** /policies/{id} | Gets an access policy
[**get_access_policy_for_resource**](PoliciesApi.md#get_access_policy_for_resource) | **GET** /policies/{action}/{resource} | Gets an access policy for the specified action and resource
[**remove_access_policy**](PoliciesApi.md#remove_access_policy) | **DELETE** /policies/{id} | Deletes an access policy
[**update_access_policy**](PoliciesApi.md#update_access_policy) | **PUT** /policies/{id} | Updates a access policy



## create_access_policy

> crate::models::AccessPolicyEntity create_access_policy(body)
Creates an access policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AccessPolicyEntity**](AccessPolicyEntity.md) | The access policy configuration details. | [required] |

### Return type

[**crate::models::AccessPolicyEntity**](AccessPolicyEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_access_policy

> crate::models::AccessPolicyEntity get_access_policy(id)
Gets an access policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The access policy id. | [required] |

### Return type

[**crate::models::AccessPolicyEntity**](AccessPolicyEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_access_policy_for_resource

> crate::models::AccessPolicyEntity get_access_policy_for_resource(action, resource)
Gets an access policy for the specified action and resource

Will return the effective policy if no component specific policy exists for the specified action and resource. Must have Read permissions to the policy with the desired action and resource. Permissions for the policy that is returned will be indicated in the response. This means the client could be authorized to get the policy for a given component but the effective policy may be inherited from an ancestor Process Group. If the client does not have permissions to that policy, the response will not include the policy and the permissions in the response will be marked accordingly. If the client does not have permissions to the policy of the desired action and resource a 403 response will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action** | **String** | The request action. | [required] |
**resource** | **String** | The resource of the policy. | [required] |

### Return type

[**crate::models::AccessPolicyEntity**](AccessPolicyEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_access_policy

> crate::models::AccessPolicyEntity remove_access_policy(id, version, client_id, disconnected_node_acknowledged)
Deletes an access policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The access policy id. | [required] |
**version** | Option<**String**> | The revision is used to verify the client is working with the latest version of the flow. |  |
**client_id** | Option<**String**> | If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response. |  |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::AccessPolicyEntity**](AccessPolicyEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_access_policy

> crate::models::AccessPolicyEntity update_access_policy(id, body)
Updates a access policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The access policy id. | [required] |
**body** | [**AccessPolicyEntity**](AccessPolicyEntity.md) | The access policy configuration details. | [required] |

### Return type

[**crate::models::AccessPolicyEntity**](AccessPolicyEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

