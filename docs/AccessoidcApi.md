# \AccessoidcApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**oidc_callback**](AccessoidcApi.md#oidc_callback) | **GET** /access/oidc/callback | Redirect/callback URI for processing the result of the OpenId Connect login sequence.
[**oidc_exchange**](AccessoidcApi.md#oidc_exchange) | **POST** /access/oidc/exchange | Retrieves a JWT following a successful login sequence using the configured OpenId Connect provider.
[**oidc_logout**](AccessoidcApi.md#oidc_logout) | **GET** /access/oidc/logout | Performs a logout in the OpenId Provider.
[**oidc_logout_callback**](AccessoidcApi.md#oidc_logout_callback) | **GET** /access/oidc/logoutCallback | Redirect/callback URI for processing the result of the OpenId Connect logout sequence.
[**oidc_request**](AccessoidcApi.md#oidc_request) | **GET** /access/oidc/request | Initiates a request to authenticate through the configured OpenId Connect provider.



## oidc_callback

> oidc_callback()
Redirect/callback URI for processing the result of the OpenId Connect login sequence.

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oidc_exchange

> String oidc_exchange()
Retrieves a JWT following a successful login sequence using the configured OpenId Connect provider.

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

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


## oidc_logout

> oidc_logout()
Performs a logout in the OpenId Provider.

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oidc_logout_callback

> oidc_logout_callback()
Redirect/callback URI for processing the result of the OpenId Connect logout sequence.

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oidc_request

> oidc_request()
Initiates a request to authenticate through the configured OpenId Connect provider.

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

