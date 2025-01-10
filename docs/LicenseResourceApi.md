# \LicenseResourceApi

All URIs are relative to *https://api.fiskeridir.no/pub-aqua*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_ajour_transfers**](LicenseResourceApi.md#get_ajour_transfers) | **GET** /api/v1/licenses/{license-nr}/transfers | Hent ajour-dato på overføringer mellom juridiske enheter av tillatelse på tillatelsesnummer
[**get_by_license_nr**](LicenseResourceApi.md#get_by_license_nr) | **GET** /api/v1/licenses/{license-nr} | Hent en tillatelse etter tillatelsesnummer
[**get_decisions1**](LicenseResourceApi.md#get_decisions1) | **GET** /api/v1/licenses/{license-nr}/decisions | Hent vedtak for tillatelse etter tillatelsesnummer
[**get_license_capacity_history_by_license_nr**](LicenseResourceApi.md#get_license_capacity_history_by_license_nr) | **GET** /api/v1/licenses/{license-nr}/capacity-history | Hent kapasistetshistorien for tillatelse etter tillatelsesnummer
[**get_license_connection_for_site_by_site_nr1**](LicenseResourceApi.md#get_license_connection_for_site_by_site_nr1) | **GET** /api/v1/licenses/{license-nr}/site-connections | Hent lokalitetstilknytninger for tillatelse etter tillatelsesnummer
[**get_licenses**](LicenseResourceApi.md#get_licenses) | **GET** /api/v1/licenses | Hent alle tillatelser som tilfredsstiller kriteriene for søkeparametere
[**get_licenses_overview**](LicenseResourceApi.md#get_licenses_overview) | **GET** /api/v1/licenses-overview | Hent alle tillatelser som tilfredsstiller kriteriene for søkeparametere
[**get_liens**](LicenseResourceApi.md#get_liens) | **GET** /api/v1/licenses/{license-nr}/liens | Hent panterettholdere av tillatelse etter tillatelsesnummer



## get_ajour_transfers

> models::LicenseTransfers get_ajour_transfers(license_nr)
Hent ajour-dato på overføringer mellom juridiske enheter av tillatelse på tillatelsesnummer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_nr** | **String** | Filtrerer overføringer mellom juridiske enheter på tillatelsesnummer. | [required] |

### Return type

[**models::LicenseTransfers**](LicenseTransfers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_by_license_nr

> models::LicenseDetail get_by_license_nr(license_nr)
Hent en tillatelse etter tillatelsesnummer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_nr** | **String** | Hent tillatelsen med angitt tillatelsesnummer. | [required] |

### Return type

[**models::LicenseDetail**](LicenseDetail.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_decisions1

> Vec<models::LicenseDecision> get_decisions1(license_nr)
Hent vedtak for tillatelse etter tillatelsesnummer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_nr** | **String** | Filtrerer vedtak på tillatelsesnummer. | [required] |

### Return type

[**Vec<models::LicenseDecision>**](LicenseDecision.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_capacity_history_by_license_nr

> Vec<models::LicenseCapacityHistory> get_license_capacity_history_by_license_nr(license_nr)
Hent kapasistetshistorien for tillatelse etter tillatelsesnummer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_nr** | **String** | Filtrer kapasitetshistorikken til tillatelsen på tillatelsesnummer. | [required] |

### Return type

[**Vec<models::LicenseCapacityHistory>**](LicenseCapacityHistory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_connection_for_site_by_site_nr1

> Vec<models::SiteConnectionForLicense> get_license_connection_for_site_by_site_nr1(license_nr)
Hent lokalitetstilknytninger for tillatelse etter tillatelsesnummer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_nr** | **String** | Hent lokalitetstilknytninger registrert til tillatelsesnummeret. | [required] |

### Return type

[**Vec<models::SiteConnectionForLicense>**](SiteConnectionForLicense.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_licenses

> Vec<models::LicenseDetail> get_licenses(license_nr, original_license_nr, legal_entity_nr_id, legal_entity_nr, legal_entity_name, portfolio_type, tag, not_tag, intention, produces, species_code, species_type, species_comp_id, municipality_code, county_code, production_area_code, valid_from, registered_from, temporary_until, temporary_from, range)
Hent alle tillatelser som tilfredsstiller kriteriene for søkeparametere

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_nr** | Option<**String**> | Hent tillatelsen med angitt tillatelsesnummer. |  |
**original_license_nr** | Option<**String**> | Filtrerer tillatelser på angitt originalt tillatelsesnummer. |  |
**legal_entity_nr_id** | Option<**String**> | Filtrerer tillatelser som benyttes av angitt juridisk-enhets-identifikator. |  |
**legal_entity_nr** | Option<**String**> | Filtrerer tillatelser som benyttes av angitt organisasjonsnummer eller fødselsnummer. Må være 9 eller 11 siffer langt. |  |
**legal_entity_name** | Option<**String**> | Filtrerer tillatelser som benyttes av navngitt juridisk enhet. |  |
**portfolio_type** | Option<**String**> | Filtrerer tillatelser med denne porteføljetypen. |  |
**tag** | Option<**String**> | Filtrerer tillatelser som har den angitte etiketten. Eksempel: KOMM-MATF |  |
**not_tag** | Option<**String**> | Filtrerer tillatelser som ikke inkluderer denne etiketten. Eksempel: KOMM-MATF |  |
**intention** | Option<**String**> | Filtrerer tillatelser som har angitt formål. Eksempel: Commercial |  |
**produces** | Option<**String**> | Filtrerer tillatelser som blir klassifisert til å drive med angitt type produksjon. Eksempel: Consumption |  |
**species_code** | Option<**String**> | Filtrerer tillatelser som blir klassifisert til å drive med angitt artskode. Må være 6 siffer langt. |  |
**species_type** | Option<**String**> | Filtrerer tillatelser som blir klassifisert til å drive med angitt artstype. Eksempel laksefisk (Salmon). |  |
**species_comp_id** | Option<**String**> | Filtrerer tillatelser på angitt artskomposisjons-ID. |  |
**municipality_code** | Option<**String**> | Filtrerer tillatelser registrert til angitt kommunenummer. Må være 4 siffer langt. |  |
**county_code** | Option<**String**> | Filtrerer tillatelser registrert til angitt fylkesnummer. Må være to 2 siffer langt. |  |
**production_area_code** | Option<**String**> | Filtrerer tillatelser registrert til angitt produksjonsområde. Må være maks 2 siffer langt. |  |
**valid_from** | Option<**String**> | Filtrerer tillatelser som har blitt gyldig etter angitt dato. Må være gitt i følgende format: 2020-01-01T17:00:00Z |  |
**registered_from** | Option<**String**> | Filtrerer tillatelser som har blitt registrert etter gitt dato. Må være gitt i følgende format: 2020-01-01T17:00:00Z |  |
**temporary_until** | Option<**String**> | Filtrerer tillatelser som er midlertidig til gitt dato. Må være gitt i følgende format: 2020-01-01T17:00:00Z |  |
**temporary_from** | Option<**String**> | Filtrerer tillatelser som er midlertidig fra gitt dato. Må være gitt i følgende format: 2019-01-01T17:00:00Z |  |
**range** | Option<**String**> | Henter ut angitt utvalg av tillatelser. Må være oppgitt i følgende format: 0-4 |  |

### Return type

[**Vec<models::LicenseDetail>**](LicenseDetail.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_licenses_overview

> Vec<models::LicenseOverview> get_licenses_overview(license_nr, original_license_nr, legal_entity_nr_id, legal_entity_nr, legal_entity_name, portfolio_type, tag, not_tag, intention, produces, species_code, species_type, species_comp_id, municipality_code, county_code, production_area_code, valid_from, registered_from, temporary_until, temporary_from, range)
Hent alle tillatelser som tilfredsstiller kriteriene for søkeparametere

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_nr** | Option<**String**> | Hent tillatelsen med angitt tillatelsesnummer. |  |
**original_license_nr** | Option<**String**> | Filtrerer tillatelser på angitt originalt tillatelsesnummer. |  |
**legal_entity_nr_id** | Option<**String**> | Filtrerer tillatelser som benyttes av angitt juridisk-enhets-identifikator. |  |
**legal_entity_nr** | Option<**String**> | Filtrerer tillatelser som benyttes av angitt organisasjonsnummer eller fødselsnummer. Må være 9 eller 11 siffer langt. |  |
**legal_entity_name** | Option<**String**> | Filtrerer tillatelser som benyttes av navngitt juridisk enhet. |  |
**portfolio_type** | Option<**String**> | Filtrerer tillatelser med denne porteføljetypen. |  |
**tag** | Option<**String**> | Filtrerer tillatelser som har den angitte etiketten. Eksempel: KOMM-MATF |  |
**not_tag** | Option<**String**> | Filtrerer tillatelser som ikke inkluderer denne etiketten. Eksempel: KOMM-MATF |  |
**intention** | Option<**String**> | Filtrerer tillatelser som har angitt formål. Eksempel: Commercial |  |
**produces** | Option<**String**> | Filtrerer tillatelser som blir klassifisert til å drive med angitt type produksjon. Eksempel: Consumption |  |
**species_code** | Option<**String**> | Filtrerer tillatelser som blir klassifisert til å drive med angitt artskode. Må være 6 siffer langt. |  |
**species_type** | Option<**String**> | Filtrerer tillatelser som blir klassifisert til å drive med angitt artstype. Eksempel laksefisk (Salmon). |  |
**species_comp_id** | Option<**String**> | Filtrerer tillatelser på angitt artskomposisjons-ID. |  |
**municipality_code** | Option<**String**> | Filtrerer tillatelser registrert til angitt kommunenummer. Må være 4 siffer langt. |  |
**county_code** | Option<**String**> | Filtrerer tillatelser registrert til angitt fylkesnummer. Må være to 2 siffer langt. |  |
**production_area_code** | Option<**String**> | Filtrerer tillatelser registrert til angitt produksjonsområde. Må være maks 2 siffer langt. |  |
**valid_from** | Option<**String**> | Filtrerer tillatelser som har blitt gyldig etter angitt dato. Må være gitt i følgende format: 2020-01-01T17:00:00Z |  |
**registered_from** | Option<**String**> | Filtrerer tillatelser som har blitt registrert etter gitt dato. Må være gitt i følgende format: 2020-01-01T17:00:00Z |  |
**temporary_until** | Option<**String**> | Filtrerer tillatelser som er midlertidig til gitt dato. Må være gitt i følgende format: 2020-01-01T17:00:00Z |  |
**temporary_from** | Option<**String**> | Filtrerer tillatelser som er midlertidig fra gitt dato. Må være gitt i følgende format: 2019-01-01T17:00:00Z |  |
**range** | Option<**String**> | Henter ut angitt utvalg av tillatelser. Må være oppgitt i følgende format: 0-4 |  |

### Return type

[**Vec<models::LicenseOverview>**](LicenseOverview.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_liens

> models::LienHolders get_liens(license_nr)
Hent panterettholdere av tillatelse etter tillatelsesnummer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_nr** | **String** | Filtrerer panterettholdere med ajour-dato på tillatelsesnummer. | [required] |

### Return type

[**models::LienHolders**](LienHolders.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json; charset=UTF-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

