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

/// GrantInfo : Informasjon om tillatelser.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GrantInfo {
    /// Tillatt kapasitet.
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<f64>,
    /// Enheten på kapasitetsmengden, f.eks. Tonn.
    #[serde(rename = "capacityUnit", skip_serializing_if = "Option::is_none")]
    pub capacity_unit: Option<String>,
    /// Kapasitetstypen.
    #[serde(rename = "capacityType", skip_serializing_if = "Option::is_none")]
    pub capacity_type: Option<String>,
    /// Tidsstempel på når tillatelsen ble gyldig.
    #[serde(rename = "grantedTime", skip_serializing_if = "Option::is_none")]
    pub granted_time: Option<String>,
    /// Juridisk-enhetsnummer-identifikator.
    #[serde(rename = "legalEntityNrId", skip_serializing_if = "Option::is_none")]
    pub legal_entity_nr_id: Option<String>,
    /// Åpent juridisk enhetsnummer
    #[serde(rename = "openLegalEntityNr", skip_serializing_if = "Option::is_none")]
    pub open_legal_entity_nr: Option<String>,
    /// Navnet på den juridiske enheten
    #[serde(rename = "legalEntityName", skip_serializing_if = "Option::is_none")]
    pub legal_entity_name: Option<String>,
}

impl GrantInfo {
    /// Informasjon om tillatelser.
    pub fn new() -> GrantInfo {
        GrantInfo {
            capacity: None,
            capacity_unit: None,
            capacity_type: None,
            granted_time: None,
            legal_entity_nr_id: None,
            open_legal_entity_nr: None,
            legal_entity_name: None,
        }
    }
}
