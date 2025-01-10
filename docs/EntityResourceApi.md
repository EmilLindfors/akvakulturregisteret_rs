# \EntityResourceApi

All URIs are relative to *https://api.fiskeridir.no/pub-aqua*

Method | HTTP request | Description
------------- | ------------- | -------------
[**entities**](EntityResourceApi.md#entities) | **GET** /api/v1/entities | Hent en liste av juridiske enheter
[**get_entity**](EntityResourceApi.md#get_entity) | **GET** /api/v1/entities/{entity-nr-id} | Hent angitt juridisk enhet
[**get_sites_for_entity**](EntityResourceApi.md#get_sites_for_entity) | **GET** /api/v1/entities/{entity-nr-id}/sites | Hent en liste av lokaliteter hvor en juridisk enhet driver med akvakultur
[**get_sites_for_entity_by_legal_entity_nr**](EntityResourceApi.md#get_sites_for_entity_by_legal_entity_nr) | **GET** /api/v1/entities/sites-by-entity-nr/{entity-nr} | Henter en liste av lokaliteter benyttet av juridisk enhet identifisert ved fødselsnummer eller organisasjonsnummer



## entities

> Vec<models::Entity> entities(entity_nr_id, entity_nr, name, zip_code, zip_name, county_code, county_name, country_code, country_name, range)
Hent en liste av juridiske enheter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_nr_id** | Option<**String**> | Filtrerer juridiske enheter etter juridisk-enhets-identifikator. |  |
**entity_nr** | Option<**String**> | Kun den juridiske enheten med angitt organisasjonsnummer eller fødselsnummer blir returnert.  Må være 9 eller 11 siffer langt. |  |
**name** | Option<**String**> | Filtrerer juridiske enheter registrert med angitt enhetsnavn. |  |
**zip_code** | Option<**String**> | Filtrerer juridiske enheter registrert til angitt postnummer. Må være 4 siffer langt. |  |
**zip_name** | Option<**String**> | Filtrerer juridiske enheter som er registrert til angitt poststed. |  |
**county_code** | Option<**String**> | Filtrerer juridiske enheter registrert til angitt fylkesnummer. Må være 2 siffer langt. |  |
**county_name** | Option<**String**> | Filtrerer juridiske enheter med registrert adresse i angitt fylkesnavn. |  |
**country_code** | Option<**String**> | Filtrerer juridiske enheter registrert til angitt landskode. Eksempel: NOR |  |
**country_name** | Option<**String**> | Filtrerer juridiske enheter registrert til angitt landsnavn. |  |
**range** | Option<**String**> | Henter ut angitt utvalg av juridiske enheter. Må være i følgende format: 0-4 |  |

### Return type

[**Vec<models::Entity>**](Entity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_entity

> models::Entity get_entity(entity_nr_id)
Hent angitt juridisk enhet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_nr_id** | **String** | Kun den juridiske enheten med gitt juridisk-enhets-identifikator blir returnert. | [required] |

### Return type

[**models::Entity**](Entity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sites_for_entity

> Vec<models::SiteForLegalEntity> get_sites_for_entity(entity_nr_id, range, include_all_connections, activity, operation)
Hent en liste av lokaliteter hvor en juridisk enhet driver med akvakultur

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_nr_id** | **String** | Filtrerer lokaliteter benyttet av angitt juridisk enhet. | [required] |
**range** | Option<**String**> | Henter ut angitt utvalg av lokaliteter. Må være i følgende format: 0-4 |  |
**include_all_connections** | Option<**String**> | Tar med tilknytningene til lokaliteter som benyttes av en juridisk enhet. |  |
**activity** | Option<**String**> | Filtrerer lokaliteter som driver med angitt virksomhet. |  |
**operation** | Option<**String**> | Filtrerer lokaliteter som har denne typen drift. |  |

### Return type

[**Vec<models::SiteForLegalEntity>**](SiteForLegalEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sites_for_entity_by_legal_entity_nr

> Vec<models::SiteForLegalEntity> get_sites_for_entity_by_legal_entity_nr(entity_nr, range, include_all_connections, activity, operation)
Henter en liste av lokaliteter benyttet av juridisk enhet identifisert ved fødselsnummer eller organisasjonsnummer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_nr** | **String** | Filtrerer lokaliteter på oppslag via fødselsnummer eller organisasjonsnummer. Må være 9 eller 11 siffer langt. | [required] |
**range** | Option<**String**> | Henter ut angitt utvalg av lokaliteter. Må være i følgende format: 0-4 |  |
**include_all_connections** | Option<**String**> | Tar med tilknytninger til lokaliteter som benyttes av den angitte juridisk enheten. |  |
**activity** | Option<**String**> | Filtrerer lokaliteter som driver med angitt virksomhet. |  |
**operation** | Option<**String**> | Filtrerer lokaliteter som har denne typen drift. |  |

### Return type

[**Vec<models::SiteForLegalEntity>**](SiteForLegalEntity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

