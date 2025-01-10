# \LicenseTypeResourceApi

All URIs are relative to *https://api.fiskeridir.no/pub-aqua*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_license_intention**](LicenseTypeResourceApi.md#get_license_intention) | **GET** /api/v1/license-types/intentions/{id} | Hent formålet med en tillatelsestype basert på ID
[**get_license_intentions**](LicenseTypeResourceApi.md#get_license_intentions) | **GET** /api/v1/license-types/intentions | Hent en liste av formål for hver tillatelsestype



## get_license_intention

> models::IntentionType get_license_intention(id)
Hent formålet med en tillatelsestype basert på ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::IntentionType**](IntentionType.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_intentions

> Vec<models::IntentionType> get_license_intentions()
Hent en liste av formål for hver tillatelsestype

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::IntentionType>**](IntentionType.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

