# \LabelsApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_label**](LabelsApi.md#get_label) | **GET** /labels/{id} | Gets a label
[**remove_label**](LabelsApi.md#remove_label) | **DELETE** /labels/{id} | Deletes a label
[**update_label**](LabelsApi.md#update_label) | **PUT** /labels/{id} | Updates a label



## get_label

> crate::models::LabelEntity get_label(id)
Gets a label

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The label id. | [required] |

### Return type

[**crate::models::LabelEntity**](LabelEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_label

> crate::models::LabelEntity remove_label(id, version, client_id, disconnected_node_acknowledged)
Deletes a label

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The label id. | [required] |
**version** | Option<**String**> | The revision is used to verify the client is working with the latest version of the flow. |  |
**client_id** | Option<**String**> | If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response. |  |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::LabelEntity**](LabelEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_label

> crate::models::LabelEntity update_label(id, body)
Updates a label

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The label id. | [required] |
**body** | [**LabelEntity**](LabelEntity.md) | The label configuration details. | [required] |

### Return type

[**crate::models::LabelEntity**](LabelEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

