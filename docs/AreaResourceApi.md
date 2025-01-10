# \AreaResourceApi

All URIs are relative to *https://api.fiskeridir.no/pub-aqua*

Method | HTTP request | Description
------------- | ------------- | -------------
[**area**](AreaResourceApi.md#area) | **GET** /api/v1/areas/{version-id} | Hent et spesifikt område
[**area_by_code_and_type**](AreaResourceApi.md#area_by_code_and_type) | **GET** /api/v1/areas/{type}/{code} | Hent et spesifikt område etter type og kode
[**areas**](AreaResourceApi.md#areas) | **GET** /api/v1/areas | Henter en liste av områder



## area

> models::AreaListItem area(version_id)
Hent et spesifikt område

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version_id** | **String** | Hent ut området som er knytt til gitt versjons-ID. | [required] |

### Return type

[**models::AreaListItem**](AreaListItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## area_by_code_and_type

> models::AreaListItem area_by_code_and_type(r#type, code, time)
Hent et spesifikt område etter type og kode

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | Filtrerer områder på angitt områdetype. | [required] |
**code** | **String** | Filtrerer områder registrert til angitt landskode. Eksempel: NOR | [required] |
**time** | Option<**String**> | Filtrerer områder på tid. Må være i følgende format: 2011-01-01T01:00:00Z |  |

### Return type

[**models::AreaListItem**](AreaListItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## areas

> Vec<models::AreaListItem> areas(r#type, code, time, range)
Henter en liste av områder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Filtrerer områder på angitt områdetype. |  |
**code** | Option<**String**> | Filtrerer områder registrert til angitt landskode. Eksempel: NOR |  |
**time** | Option<**String**> | Filtrerer områder på tid. Må være i følgende format: 2011-01-01T01:00:00Z |  |
**range** | Option<**String**> | Henter ut et angitt utvalg av områder. Må være i følgende format: 0-4 |  |

### Return type

[**Vec<models::AreaListItem>**](AreaListItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

