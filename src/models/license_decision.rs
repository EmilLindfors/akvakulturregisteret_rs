/*
 * Offentlig Akvakulturregister
 *
 * API for det nye offentlige Akvakulturregisteret
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: akva-hjelp@fiskeridir.no
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// LicenseDecision : Informasjon om vedtak for denne tillatelsen.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LicenseDecision {
    /// Unik identifikator for vedtaket.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Tidsstempel for den gitte fristen til saken.
    #[serde(rename = "caseGrantedTime", skip_serializing_if = "Option::is_none")]
    pub case_granted_time: Option<String>,
    /// Sakstype.
    #[serde(rename = "caseType", skip_serializing_if = "Option::is_none")]
    pub case_type: Option<String>,
    /// Sakstype verdi.
    #[serde(rename = "caseTypeValue", skip_serializing_if = "Option::is_none")]
    pub case_type_value: Option<String>,
    /// Tidsstempel for når vedtaket er gyldig fra.
    #[serde(rename = "validFrom", skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    /// Tidsstempel for når vedtaket er gyldig til.
    #[serde(rename = "validUntil", skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<String>,
    /// Type vedtak.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Verdien til denne typen vedtak.
    #[serde(rename = "typeValue", skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Tidsstempel for når vedtaket ble registrert.
    #[serde(rename = "registeredTime", skip_serializing_if = "Option::is_none")]
    pub registered_time: Option<String>,
    /// Status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Tillatelsesnummer for tillatelsen som vedtaket gjelder for.
    #[serde(rename = "licenseNr", skip_serializing_if = "Option::is_none")]
    pub license_nr: Option<String>,
    /// Vedtak i referanse til tillatelsesnummer.
    #[serde(
        rename = "decisionInRefToLicenseNr",
        skip_serializing_if = "Option::is_none"
    )]
    pub decision_in_ref_to_license_nr: Option<String>,
    /// Unik tillatelsesversjons identifikator.
    #[serde(rename = "licenseVersionId", skip_serializing_if = "Option::is_none")]
    pub license_version_id: Option<i64>,
    /// Unik tillatelses identifikator.
    #[serde(rename = "licenseId", skip_serializing_if = "Option::is_none")]
    pub license_id: Option<i64>,
    /// Statusverdi.
    #[serde(rename = "statusValue", skip_serializing_if = "Option::is_none")]
    pub status_value: Option<String>,
}

impl LicenseDecision {
    /// Informasjon om vedtak for denne tillatelsen.
    pub fn new() -> LicenseDecision {
        LicenseDecision {
            id: None,
            case_granted_time: None,
            case_type: None,
            case_type_value: None,
            valid_from: None,
            valid_until: None,
            r#type: None,
            type_value: None,
            registered_time: None,
            status: None,
            license_nr: None,
            decision_in_ref_to_license_nr: None,
            license_version_id: None,
            license_id: None,
            status_value: None,
        }
    }
}
