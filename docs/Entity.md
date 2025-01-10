# Entity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unik enhets identifikator. | [optional]
**version_id** | Option<**i64**> | Unik versjons identifikator. | [optional]
**open_nr** | Option<**String**> | Åpent nummer. | [optional]
**type_name** | Option<**String**> | Navnetypen | [optional]
**type_value** | Option<**String**> | Typeverdien. | [optional]
**name** | Option<**String**> | Navnet til enheten. | [optional]
**business_types** | Option<[**Vec<models::BusinessType>**](BusinessType.md)> |  | [optional]
**official_source_type** | Option<**String**> | Offisiell kildetype til enheten. | [optional]
**addresses** | Option<[**Vec<models::Address>**](Address.md)> |  | [optional]
**industry_codes** | Option<**Vec<String>**> | Liste av industrikoder som enheten har. | [optional]
**brreg_statuses** | Option<**Vec<String>**> | Liste av enhetens statuser i brønnøysundsregisteret. | [optional]
**version_valid_from** | Option<**String**> | Tidsstempel for når enhetsversjonen er gyldig fra. | [optional]
**version_valid_until** | Option<**String**> | Tidsstempel for når enhetsversjonen er gyldig til. | [optional]
**version_registered_time** | Option<**String**> | Tidsstempel for når enhetsversjonen er registrert. | [optional]
**version_registered_by** | Option<**String**> | Hvem som registrerte enhetsversjonen. | [optional]
**status** | Option<**String**> | Status. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


