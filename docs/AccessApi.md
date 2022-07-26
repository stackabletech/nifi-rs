# \AccessApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_access_token**](AccessApi.md#create_access_token) | **POST** /access/token | Creates a token for accessing the REST API via username/password
[**create_access_token_from_ticket**](AccessApi.md#create_access_token_from_ticket) | **POST** /access/kerberos | Creates a token for accessing the REST API via Kerberos ticket exchange / SPNEGO negotiation
[**get_access_status**](AccessApi.md#get_access_status) | **GET** /access | Gets the status the client's access
[**get_login_config**](AccessApi.md#get_login_config) | **GET** /access/config | Retrieves the access configuration for this NiFi
[**knox_callback**](AccessApi.md#knox_callback) | **GET** /access/knox/callback | Redirect/callback URI for processing the result of the Apache Knox login sequence.
[**knox_logout**](AccessApi.md#knox_logout) | **GET** /access/knox/logout | Performs a logout in the Apache Knox.
[**knox_request**](AccessApi.md#knox_request) | **GET** /access/knox/request | Initiates a request to authenticate through Apache Knox.
[**log_out**](AccessApi.md#log_out) | **DELETE** /access/logout | Performs a logout for other providers that have been issued a JWT.
[**log_out_complete**](AccessApi.md#log_out_complete) | **GET** /access/logout/complete | Completes the logout sequence by removing the cached Logout Request and Cookie if they existed and redirects to /nifi/login.



## create_access_token

> String create_access_token(username, password)
Creates a token for accessing the REST API via username/password

The token returned is formatted as a JSON Web Token (JWT). The token is base64 encoded and comprised of three parts. The header, the body, and the signature. The expiration of the token is a contained within the body. It is stored in the browser as a cookie, but also returned inthe response body to be stored/used by third party client scripts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_access_token_from_ticket

> String create_access_token_from_ticket()
Creates a token for accessing the REST API via Kerberos ticket exchange / SPNEGO negotiation

The token returned is formatted as a JSON Web Token (JWT). The token is base64 encoded and comprised of three parts. The header, the body, and the signature. The expiration of the token is a contained within the body. The token can be used in the Authorization header in the format 'Authorization: Bearer <token>'. It is also stored in the browser as a cookie.

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


## get_access_status

> crate::models::AccessStatusEntity get_access_status()
Gets the status the client's access

Note: This endpoint is subject to change as NiFi and it's REST API evolve.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AccessStatusEntity**](AccessStatusEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_login_config

> crate::models::AccessConfigurationEntity get_login_config()
Retrieves the access configuration for this NiFi

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AccessConfigurationEntity**](AccessConfigurationEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## knox_callback

> knox_callback()
Redirect/callback URI for processing the result of the Apache Knox login sequence.

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


## knox_logout

> knox_logout()
Performs a logout in the Apache Knox.

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


## knox_request

> knox_request()
Initiates a request to authenticate through Apache Knox.

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


## log_out

> log_out()
Performs a logout for other providers that have been issued a JWT.

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


## log_out_complete

> log_out_complete()
Completes the logout sequence by removing the cached Logout Request and Cookie if they existed and redirects to /nifi/login.

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

