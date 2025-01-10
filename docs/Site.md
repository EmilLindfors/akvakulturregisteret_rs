# Site

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**site_id** | Option<**i64**> | Unik lokalitets identifikator | [optional]
**version_id** | Option<**i64**> | Versjons ID | [optional]
**site_nr** | Option<**i32**> | Lokalitetsnummer | [optional]
**name** | Option<**String**> | Navnet på lokaliteten | [optional]
**placement_type** | Option<**String**> | Plasseringstypen til lokaliteten | [optional]
**placement_type_value** | Option<**String**> | Plasseringstypeverdien til lokaliteten | [optional]
**water_type** | Option<**String**> | Vanntype som benyttes på lokaliteten | [optional]
**water_type_value** | Option<**String**> | Vanntypeverdien på lokaliteten | [optional]
**first_clearance_time** | Option<**String**> | Første klareringstiden til lokaliteten | [optional]
**first_clearance_type** | Option<**String**> | Første klareringstypen til lokaliteten | [optional]
**first_clearance_type_value** | Option<**String**> | Første klareringstypeverdien til lokaliteten | [optional]
**latitude** | Option<**f64**> | Breddegrad | [optional]
**longitude** | Option<**f64**> | Lengdegrad | [optional]
**capacity** | Option<**f64**> | Kapasiteten til lokaliteten | [optional]
**temp_capacity** | Option<**f64**> | Den midlertidige kapasiteten til lokaliteten | [optional]
**capacity_unit_type** | Option<**String**> | Kapasitets enhetstype | [optional]
**placement** | Option<[**models::AreaPlacement**](AreaPlacement.md)> |  | [optional]
**species_type** | Option<**String**> | Artstype | [optional]
**species_type_value** | Option<**String**> | Artstypeverdi | [optional]
**species_limitations** | Option<[**Vec<models::FishCode>**](FishCode.md)> | Artsbegrensninger | [optional]
**connections** | Option<[**Vec<models::LatestLicenseSiteConnectionDetail>**](LatestLicenseSiteConnectionDetail.md)> | Forbindelsene til lokaliteten | [optional]
**obsolete_connections** | Option<[**Vec<models::LatestLicenseSiteConnectionDetail>**](LatestLicenseSiteConnectionDetail.md)> | Utdaterte forbindelser til lokaliteten | [optional]
**version** | Option<[**models::VersionDetail**](VersionDetail.md)> |  | [optional]
**is_slaughtery** | Option<**bool**> | Om lokalitet er slakteri eller ikke | [optional]
**has_commercial_activity** | Option<**bool**> | Om lokaliteten har kommersiell aktivitet eller ikke | [optional]
**has_colocation** | Option<**bool**> | Om lokaliteten er en del av samlokalitet | [optional]
**has_joint_operation** | Option<**bool**> | Om lokaliteten har felles drift med annen lokalitet | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


