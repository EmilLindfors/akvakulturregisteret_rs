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

/// LicenseConnectionForSite : Tillatelsesforbindelsene til en gitt lokalitet.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LicenseConnectionForSite {
    #[serde(rename = "connectionDetail", skip_serializing_if = "Option::is_none")]
    pub connection_detail: Option<Box<models::LatestLicenseSiteConnectionDetail>>,
    /// Arvet tillatelsesnummer.
    #[serde(rename = "legacyLicenseNr", skip_serializing_if = "Option::is_none")]
    pub legacy_license_nr: Option<String>,
    /// Juridisk-enhetsnummer-identifikator
    #[serde(rename = "legalEntityNrId", skip_serializing_if = "Option::is_none")]
    pub legal_entity_nr_id: Option<String>,
    /// Åpent juridisk enhetsnummer
    #[serde(rename = "openLegalEntityNr", skip_serializing_if = "Option::is_none")]
    pub open_legal_entity_nr: Option<String>,
    /// Juridisk enhetsnavn
    #[serde(rename = "legalEntityName", skip_serializing_if = "Option::is_none")]
    pub legal_entity_name: Option<String>,
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<Box<models::CapacityInfo>>,
    /// Formålet med benyttelsen av lokaliteten.
    #[serde(rename = "intention", skip_serializing_if = "Option::is_none")]
    pub intention: Option<String>,
    /// Formålsverdi
    #[serde(rename = "intentionValue", skip_serializing_if = "Option::is_none")]
    pub intention_value: Option<String>,
    /// Produksjonsstadiet lokaliteten og dens forbindelser er på.
    #[serde(rename = "productionStage", skip_serializing_if = "Option::is_none")]
    pub production_stage: Option<String>,
    /// Produksjonsstadie-verdi
    #[serde(rename = "productionStageValue", skip_serializing_if = "Option::is_none")]
    pub production_stage_value: Option<String>,
    /// Etikett.
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Tidsstempel for når tillatelsesforbindelsen er gyldig fra.
    #[serde(rename = "validFrom", skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    /// Tidsstempel for når tillatelsesforbindelsen er gyldig til.
    #[serde(rename = "validUntil", skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<String>,
    /// Tidsstempel for når tillatelsesforbindelsen er midlertidig gyldig til.
    #[serde(rename = "temporaryUntil", skip_serializing_if = "Option::is_none")]
    pub temporary_until: Option<String>,
    /// Status på tillatelsesforbindelsene til lokaliteten.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Statusverdi
    #[serde(rename = "statusValue", skip_serializing_if = "Option::is_none")]
    pub status_value: Option<String>,
}

impl LicenseConnectionForSite {
    /// Tillatelsesforbindelsene til en gitt lokalitet.
    pub fn new() -> LicenseConnectionForSite {
        LicenseConnectionForSite {
            connection_detail: None,
            legacy_license_nr: None,
            legal_entity_nr_id: None,
            open_legal_entity_nr: None,
            legal_entity_name: None,
            capacity: None,
            intention: None,
            intention_value: None,
            production_stage: None,
            production_stage_value: None,
            tag: None,
            valid_from: None,
            valid_until: None,
            temporary_until: None,
            status: None,
            status_value: None,
        }
    }
}

