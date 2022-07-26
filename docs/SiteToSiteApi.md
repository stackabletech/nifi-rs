# \SiteToSiteApi

All URIs are relative to *http://localhost/nifi-api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_peers**](SiteToSiteApi.md#get_peers) | **GET** /site-to-site/peers | Returns the available Peers and its status of this NiFi
[**get_site_to_site_details**](SiteToSiteApi.md#get_site_to_site_details) | **GET** /site-to-site | Returns the details about this NiFi necessary to communicate via site to site



## get_peers

> crate::models::PeersEntity get_peers()
Returns the available Peers and its status of this NiFi

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PeersEntity**](PeersEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_site_to_site_details

> crate::models::ControllerEntity get_site_to_site_details()
Returns the details about this NiFi necessary to communicate via site to site

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ControllerEntity**](ControllerEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

