# \SnippetsApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_snippet**](SnippetsApi.md#create_snippet) | **POST** /snippets | Creates a snippet. The snippet will be automatically discarded if not used in a subsequent request after 1 minute.
[**delete_snippet**](SnippetsApi.md#delete_snippet) | **DELETE** /snippets/{id} | Deletes the components in a snippet and discards the snippet
[**update_snippet**](SnippetsApi.md#update_snippet) | **PUT** /snippets/{id} | Move's the components in this Snippet into a new Process Group and discards the snippet



## create_snippet

> crate::models::SnippetEntity create_snippet(body)
Creates a snippet. The snippet will be automatically discarded if not used in a subsequent request after 1 minute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SnippetEntity**](SnippetEntity.md) | The snippet configuration details. | [required] |

### Return type

[**crate::models::SnippetEntity**](SnippetEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_snippet

> crate::models::SnippetEntity delete_snippet(id, disconnected_node_acknowledged)
Deletes the components in a snippet and discards the snippet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The snippet id. | [required] |
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. |  |[default to false]

### Return type

[**crate::models::SnippetEntity**](SnippetEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_snippet

> crate::models::SnippetEntity update_snippet(id, body)
Move's the components in this Snippet into a new Process Group and discards the snippet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The snippet id. | [required] |
**body** | [**SnippetEntity**](SnippetEntity.md) | The snippet configuration details. | [required] |

### Return type

[**crate::models::SnippetEntity**](SnippetEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

