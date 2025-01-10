# SiteForLegalEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**site_nr** | Option<**String**> | Lokalitetsnummer | [optional]
**site_name** | Option<**String**> | Navnet på lokaliteten. | [optional]
**site_capacity** | Option<**f64**> | Tillatt kapasitet for den gitte lokaliteten. | [optional]
**site_temp_capacity** | Option<**f64**> | Den midlertidige kapasiteten til en lokalitet. | [optional]
**site_capacity_unit_type** | Option<**String**> | Kapasitetsenheten som blir brukt for å beskrive kapasitetsmengden på lokaliteten. Oppgitt i f.eks. Tonn. | [optional]
**site_placement** | Option<[**models::AreaPlacement**](AreaPlacement.md)> |  | [optional]
**connections** | Option<[**Vec<models::LicenseConnectionForSite>**](LicenseConnectionForSite.md)> |  | [optional]
**valid_from** | Option<**String**> | Tidsstempel for når lokaliteten begynte å bli benyttet av den juridiske enheten. | [optional]
**valid_until** | Option<**String**> | Tidsstempel for når lokaliteten blir benyttet til av den juridiske enheten. | [optional]
**temporary_until** | Option<**String**> | Tidsstempel for når den gjeldende lokaliteten er midlertidig gyldig til for den juridiske enheten. | [optional]
**status** | Option<**String**> | Status | [optional]
**status_value** | Option<**String**> | Statusverdi | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


