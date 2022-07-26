# \FunnelApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_funnel**](FunnelApi.md#get_funnel) | **GET** /funnels/{id} | Gets a funnel
[**remove_funnel**](FunnelApi.md#remove_funnel) | **DELETE** /funnels/{id} | Deletes a funnel
[**update_funnel**](FunnelApi.md#update_funnel) | **PUT** /funnels/{id} | Updates a funnel



## get_funnel

> crate::models::FunnelEntity get_funnel(id)
Gets a funnel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The funnel id. | [required] |

### Return type

[**crate::models::FunnelEntity**](FunnelEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_funnel

> crate::models::FunnelEntity remove_funnel(id, version, client_id, disconnected_node_acknowledged)
Deletes a funnel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The funnel id. | [required] |
**version** | Option<**String**> | The revision is used to verify the client is working with the latest version of the flow. |  |
**client_id** | Option<**String**> | If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response. |  |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::FunnelEntity**](FunnelEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_funnel

> crate::models::FunnelEntity update_funnel(id, body)
Updates a funnel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The funnel id. | [required] |
**body** | [**FunnelEntity**](FunnelEntity.md) | The funnel configuration details. | [required] |

### Return type

[**crate::models::FunnelEntity**](FunnelEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

