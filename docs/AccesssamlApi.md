# \AccesssamlApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**saml_local_logout**](AccesssamlApi.md#saml_local_logout) | **GET** /access/saml/local-logout | Local logout when SAML is enabled, does not communicate with the IDP.
[**saml_login_exchange**](AccesssamlApi.md#saml_login_exchange) | **POST** /access/saml/login/exchange | Retrieves a JWT following a successful login sequence using the configured SAML identity provider.
[**saml_login_http_post_consumer**](AccesssamlApi.md#saml_login_http_post_consumer) | **POST** /access/saml/login/consumer | Processes the SSO response from the SAML identity provider for HTTP-POST binding.
[**saml_login_http_redirect_consumer**](AccesssamlApi.md#saml_login_http_redirect_consumer) | **GET** /access/saml/login/consumer | Processes the SSO response from the SAML identity provider for HTTP-REDIRECT binding.
[**saml_login_request**](AccesssamlApi.md#saml_login_request) | **GET** /access/saml/login/request | Initiates an SSO request to the configured SAML identity provider.
[**saml_metadata**](AccesssamlApi.md#saml_metadata) | **GET** /access/saml/metadata | Retrieves the service provider metadata.
[**saml_single_logout_http_post_consumer**](AccesssamlApi.md#saml_single_logout_http_post_consumer) | **POST** /access/saml/single-logout/consumer | Processes a SingleLogout message from the configured SAML identity provider using the HTTP-POST binding.
[**saml_single_logout_http_redirect_consumer**](AccesssamlApi.md#saml_single_logout_http_redirect_consumer) | **GET** /access/saml/single-logout/consumer | Processes a SingleLogout message from the configured SAML identity provider using the HTTP-REDIRECT binding.
[**saml_single_logout_request**](AccesssamlApi.md#saml_single_logout_request) | **GET** /access/saml/single-logout/request | Initiates a logout request using the SingleLogout service of the configured SAML identity provider.



## saml_local_logout

> saml_local_logout()
Local logout when SAML is enabled, does not communicate with the IDP.

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


## saml_login_exchange

> String saml_login_exchange()
Retrieves a JWT following a successful login sequence using the configured SAML identity provider.

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


## saml_login_http_post_consumer

> saml_login_http_post_consumer()
Processes the SSO response from the SAML identity provider for HTTP-POST binding.

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


## saml_login_http_redirect_consumer

> saml_login_http_redirect_consumer()
Processes the SSO response from the SAML identity provider for HTTP-REDIRECT binding.

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


## saml_login_request

> saml_login_request()
Initiates an SSO request to the configured SAML identity provider.

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


## saml_metadata

> saml_metadata()
Retrieves the service provider metadata.

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


## saml_single_logout_http_post_consumer

> saml_single_logout_http_post_consumer()
Processes a SingleLogout message from the configured SAML identity provider using the HTTP-POST binding.

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


## saml_single_logout_http_redirect_consumer

> saml_single_logout_http_redirect_consumer()
Processes a SingleLogout message from the configured SAML identity provider using the HTTP-REDIRECT binding.

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


## saml_single_logout_request

> saml_single_logout_request()
Initiates a logout request using the SingleLogout service of the configured SAML identity provider.

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

