# SiteBorder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | Unik identifikator | [optional]
**site_version_id** | Option<**i64**> | Unik lokalitetsversjons-identifikator | [optional]
**site_nr** | Option<**i32**> | Lokalitetsnummer | [optional]
**name** | Option<**String**> | Navnet på lokaliteten. | [optional]
**r#type** | Option<[**models::BorderType**](BorderType.md)> |  | [optional]
**points** | Option<[**Vec<models::BorderPoint>**](BorderPoint.md)> | Liste over grensepunkter for en gitt lokalitet. | [optional]
**area_m2** | Option<**f64**> | Størrelsen på lokaliteten målt i kvadratmeter. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


