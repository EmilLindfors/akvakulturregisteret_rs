# \SiteResourceApi

All URIs are relative to *https://api.fiskeridir.no/pub-aqua*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_borders_by_site_nr**](SiteResourceApi.md#get_borders_by_site_nr) | **GET** /api/v1/sites/{site-nr}/borders | Hent grense rundt en lokalitet etter lokalitetsnummer
[**get_by_site_nr**](SiteResourceApi.md#get_by_site_nr) | **GET** /api/v1/sites/{site-nr} | Hent lokalitet etter lokalitetsnummer
[**get_decisions**](SiteResourceApi.md#get_decisions) | **GET** /api/v1/sites/{site-nr}/decisions | Hent vedtak for lokalitet etter lokalitetsnummer
[**get_license_connection_for_site_by_site_nr**](SiteResourceApi.md#get_license_connection_for_site_by_site_nr) | **GET** /api/v1/sites/{site-nr}/license-connections | Hent tillatelsesforbindelser for lokalitet etter lokalitetsnummer
[**sites**](SiteResourceApi.md#sites) | **GET** /api/v1/sites | Hent en liste av lokaliteter



## get_borders_by_site_nr

> Vec<models::SiteBorder> get_borders_by_site_nr(site_nr)
Hent grense rundt en lokalitet etter lokalitetsnummer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_nr** | **String** | Filtrerer lokalitetens grensepunkter på lokalitetsnummer. | [required] |

### Return type

[**Vec<models::SiteBorder>**](SiteBorder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_by_site_nr

> models::Site get_by_site_nr(site_nr)
Hent lokalitet etter lokalitetsnummer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_nr** | **String** | Filtrerer lokalitet på lokalitetsnummer. | [required] |

### Return type

[**models::Site**](Site.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_decisions

> Vec<models::SiteDecision> get_decisions(site_nr)
Hent vedtak for lokalitet etter lokalitetsnummer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_nr** | **String** | Hent vedtak en lokalitet har på lokalitetsnummer. | [required] |

### Return type

[**Vec<models::SiteDecision>**](SiteDecision.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_connection_for_site_by_site_nr

> Vec<models::LicenseConnectionForSite> get_license_connection_for_site_by_site_nr(site_nr)
Hent tillatelsesforbindelser for lokalitet etter lokalitetsnummer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_nr** | **String** | Filtrerer tillatelsesforbindelser på lokalitetsnummer. | [required] |

### Return type

[**Vec<models::LicenseConnectionForSite>**](LicenseConnectionForSite.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sites

> Vec<models::Site> sites(nr, legal_entity_nr_id, legal_entity_nr, license_nr, name, placement, water, species_type, municipality_code, county_code, production_area_code, valid_from, registered_from, temporary_until, range, activity, operation)
Hent en liste av lokaliteter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nr** | Option<**String**> | Filtrerer lokaliteter på lokalitetsnummer. |  |
**legal_entity_nr_id** | Option<**String**> | Filtrerer lokaliteter på registrert juridisk-enhets-identifikator. |  |
**legal_entity_nr** | Option<**String**> | Filtrerer lokaliteter på angitt organisasjonsnummer eller fødselsnummer.  Må være 9 eller 11 siffer langt. |  |
**license_nr** | Option<**String**> | Filtrerer lokaliteter på tillatelsesnummer. |  |
**name** | Option<**String**> | Filtrer lokalitet på angitt navn. |  |
**placement** | Option<**String**> | Filtrerer lokaliteter på plasseringstype. Eksempel: Offshore |  |
**water** | Option<**String**> | Filtrerer lokaliteter på vanntype. Eksempel: Salt |  |
**species_type** | Option<**String**> | Filtrerer lokaliteter som blir klassifisert til å drive med gitt artstype. Eksempel laksefisk (Salmon). |  |
**municipality_code** | Option<**String**> | Filtrer lokaliteter registrert til dette kommunenummeret. Må være fire siffer langt. |  |
**county_code** | Option<**String**> | Filtrerer lokaliteter registrert til angitt fylkesnummer. Må være to siffer langt. |  |
**production_area_code** | Option<**String**> | Filtrerer lokaliteter registrert til angitt produksjonsområdekode. Må være maks 2 siffer langt. |  |
**valid_from** | Option<**String**> | Filtrerer lokaliteter som inneholder minst en tillatelse som er blitt gyldig etter gitt dato. Må være oppgitt i følgende format: 2020-01-01T22:00:00Z |  |
**registered_from** | Option<**String**> | Filtrerer lokaliteter som er registrert fra angitt dato. Må være oppgitt i følgende format: 2020-01-01T22:00:00Z |  |
**temporary_until** | Option<**String**> |  |  |[default to ]
**range** | Option<**String**> | Henter ut angitt utvalg av lokaliteter. Må være oppgitt i følgende format: 0-4 |  |
**activity** | Option<**String**> | Filtrerer lokaliteter som driver med angitt virksomhet. |  |
**operation** | Option<**String**> | Filtrerer lokaliteter som har angitt type drift. |  |

### Return type

[**Vec<models::Site>**](Site.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

