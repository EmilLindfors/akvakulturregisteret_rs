# LicenseOverview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version_id** | Option<**i64**> | Unik identifikator for tillatelsesversjonen. | [optional]
**license_nr** | Option<**String**> | Tillatelsesnummer | [optional]
**original_license_nr** | Option<**String**> | Tillatelsesnummer slik det var da innehaveren fikk tillatelsen. | [optional]
**legacy_license_nr** | Option<**String**> | Arvet tillatelsesnummer. | [optional]
**portfolio_type** | Option<[**models::LicensePortfolioType**](LicensePortfolioType.md)> |  | [optional]
**portfolio_master_license_id** | Option<**i64**> | Unik identifikator for hovedtillatelsen i innehaverens portefølje over tillatelser. | [optional]
**legal_entity_nr_id** | Option<**String**> | Juridisk-enhetsnummer-identifikator | [optional]
**open_legal_entity_nr** | Option<**String**> | Åpent juridisk enhetsnummer. | [optional]
**legal_entity_name** | Option<**String**> | Juridisk enhetsnavn. | [optional]
**production_regime** | Option<**String**> | Regimet som produksjonsprosessen for den juridiske enheten opererer under. | [optional]
**production_model** | Option<**String**> | Modellen for hvordan produksjonsprosessen skal være. | [optional]
**capacity** | Option<[**models::CapacityInfo**](CapacityInfo.md)> |  | [optional]
**r#type** | Option<[**models::LicenseTypeOverview**](LicenseTypeOverview.md)> |  | [optional]
**species** | Option<[**models::SpeciesOverview**](SpeciesOverview.md)> |  | [optional]
**connections** | Option<[**Vec<models::LatestLicenseSiteConnectionOverview>**](LatestLicenseSiteConnectionOverview.md)> |  | [optional]
**placement** | Option<[**models::AreaPlacement**](AreaPlacement.md)> |  | [optional]
**version** | Option<[**models::VersionOverview**](VersionOverview.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


