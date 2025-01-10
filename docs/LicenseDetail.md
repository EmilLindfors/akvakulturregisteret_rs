# LicenseDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**license_id** | Option<**i64**> | Unik tillatelses-identifikator | [optional]
**version_id** | Option<**i64**> | Versjons-ID | [optional]
**license_nr** | Option<**String**> | Tillatelsesnummer | [optional]
**original_license_nr** | Option<**String**> | Tillatelsesnummer som ble brukt når tillatelsen ble registrert. | [optional]
**legacy_license_nr** | Option<**String**> | Arvet tillatelsesnummer | [optional]
**portfolio_type** | Option<[**models::LicensePortfolioType**](LicensePortfolioType.md)> |  | [optional]
**portfolio_master_license_id** | Option<**i64**> | Identifikatoren for hovedtillatelsen i tillatelsesporteføljen. | [optional]
**legal_entity_nr_id** | Option<**String**> | Juridisk-enhetsnummer-identifikator. | [optional]
**open_legal_entity_nr** | Option<**String**> | Åpent juridisk enhetsnummer. | [optional]
**legal_entity_name** | Option<**String**> | Juridisk enhetsnavn. | [optional]
**production_regime** | Option<**String**> | Informasjon om produksjonsregimet til tillatelsen. | [optional]
**production_model** | Option<**String**> | Informasjon om produksjonsmodellen til tillatelsen. | [optional]
**capacity** | Option<[**models::CapacityInfo**](CapacityInfo.md)> |  | [optional]
**r#type** | Option<[**models::LicenseTypeDetail**](LicenseTypeDetail.md)> |  | [optional]
**species** | Option<[**models::SpeciesDetail**](SpeciesDetail.md)> |  | [optional]
**connections** | Option<[**Vec<models::LatestLicenseSiteConnectionDetail>**](LatestLicenseSiteConnectionDetail.md)> |  | [optional]
**placement** | Option<[**models::AreaPlacement**](AreaPlacement.md)> |  | [optional]
**grant_information** | Option<[**models::GrantInfo**](GrantInfo.md)> |  | [optional]
**version** | Option<[**models::VersionDetail**](VersionDetail.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


