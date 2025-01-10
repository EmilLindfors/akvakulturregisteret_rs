/*
 * Offentlig Akvakulturregister
 *
 * API for det nye offentlige Akvakulturregisteret
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: akva-hjelp@fiskeridir.no
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use async_trait::async_trait;
#[cfg(feature = "mockall")]
use mockall::automock;
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(bon::Builder)]
pub struct SitesApiParams<
    'nr,
    'legal_entity_nr_id,
    'legal_entity_nr,
    'license_nr,
    'name,
    'placement,
    'water,
    'species_type,
    'municipality_code,
    'county_code,
    'production_area_code,
    'valid_from,
    'registered_from,
    'temporary_until,
    'range,
    'activity,
    'operation,
> {
    nr: Option<&'nr str>,
    legal_entity_nr_id: Option<&'legal_entity_nr_id str>,
    legal_entity_nr: Option<&'legal_entity_nr str>,
    license_nr: Option<&'license_nr str>,
    name: Option<&'name str>,
    placement: Option<&'placement str>,
    water: Option<&'water str>,
    species_type: Option<&'species_type str>,
    municipality_code: Option<&'municipality_code str>,
    county_code: Option<&'county_code str>,
    production_area_code: Option<&'production_area_code str>,
    valid_from: Option<&'valid_from str>,
    registered_from: Option<&'registered_from str>,
    temporary_until: Option<&'temporary_until str>,
    range: Option<&'range str>,
    activity: Option<&'activity str>,
    operation: Option<&'operation str>,
}

#[cfg_attr(feature = "mockall", automock)]
#[async_trait]
pub trait SiteResourceApi: Send + Sync {
    async fn get_borders_by_site_nr<'site_nr>(
        &self,
        site_nr: &'site_nr str,
    ) -> Result<ResponseContent<GetBordersBySiteNrSuccess>, Error<GetBordersBySiteNrError>>;
    async fn get_by_site_nr<'site_nr>(
        &self,
        site_nr: &'site_nr str,
    ) -> Result<ResponseContent<GetBySiteNrSuccess>, Error<GetBySiteNrError>>;
    async fn get_decisions<'site_nr>(
        &self,
        site_nr: &'site_nr str,
    ) -> Result<ResponseContent<GetDecisionsSuccess>, Error<GetDecisionsError>>;
    async fn get_license_connection_for_site_by_site_nr<'site_nr>(
        &self,
        site_nr: &'site_nr str,
    ) -> Result<
        ResponseContent<GetLicenseConnectionForSiteBySiteNrSuccess>,
        Error<GetLicenseConnectionForSiteBySiteNrError>,
    >;
    async fn sites<
        'nr,
        'legal_entity_nr_id,
        'legal_entity_nr,
        'license_nr,
        'name,
        'placement,
        'water,
        'species_type,
        'municipality_code,
        'county_code,
        'production_area_code,
        'valid_from,
        'registered_from,
        'temporary_until,
        'range,
        'activity,
        'operation,
    >(
        &self,
       params: SitesApiParams<
            'nr,
            'legal_entity_nr_id,
            'legal_entity_nr,
            'license_nr,
            'name,
            'placement,
            'water,
            'species_type,
            'municipality_code,
            'county_code,
            'production_area_code,
            'valid_from,
            'registered_from,
            'temporary_until,
            'range,
            'activity,
            'operation,
        >,
    ) -> Result<ResponseContent<SitesSuccess>, Error<SitesError>>;
}

pub struct SiteResourceApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl SiteResourceApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

#[async_trait]
impl SiteResourceApi for SiteResourceApiClient {
    async fn get_borders_by_site_nr<'site_nr>(
        &self,
        site_nr: &'site_nr str,
    ) -> Result<ResponseContent<GetBordersBySiteNrSuccess>, Error<GetBordersBySiteNrError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/sites/{site_nr}/borders",
            local_var_configuration.base_path,
            site_nr = crate::apis::urlencode(site_nr)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetBordersBySiteNrSuccess> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetBordersBySiteNrError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_by_site_nr<'site_nr>(
        &self,
        site_nr: &'site_nr str,
    ) -> Result<ResponseContent<GetBySiteNrSuccess>, Error<GetBySiteNrError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/sites/{site_nr}",
            local_var_configuration.base_path,
            site_nr = crate::apis::urlencode(site_nr)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetBySiteNrSuccess> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetBySiteNrError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_decisions<'site_nr>(
        &self,
        site_nr: &'site_nr str,
    ) -> Result<ResponseContent<GetDecisionsSuccess>, Error<GetDecisionsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/sites/{site_nr}/decisions",
            local_var_configuration.base_path,
            site_nr = crate::apis::urlencode(site_nr)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetDecisionsSuccess> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetDecisionsError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_license_connection_for_site_by_site_nr<'site_nr>(
        &self,
        site_nr: &'site_nr str,
    ) -> Result<
        ResponseContent<GetLicenseConnectionForSiteBySiteNrSuccess>,
        Error<GetLicenseConnectionForSiteBySiteNrError>,
    > {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/sites/{site_nr}/license-connections",
            local_var_configuration.base_path,
            site_nr = crate::apis::urlencode(site_nr)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetLicenseConnectionForSiteBySiteNrSuccess> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetLicenseConnectionForSiteBySiteNrError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn sites<
        'nr,
        'legal_entity_nr_id,
        'legal_entity_nr,
        'license_nr,
        'name,
        'placement,
        'water,
        'species_type,
        'municipality_code,
        'county_code,
        'production_area_code,
        'valid_from,
        'registered_from,
        'temporary_until,
        'range,
        'activity,
        'operation,
    >(
        &self,
        params: SitesApiParams<
        'nr,
        'legal_entity_nr_id,
        'legal_entity_nr,
        'license_nr,
        'name,
        'placement,
        'water,
        'species_type,
        'municipality_code,
        'county_code,
        'production_area_code,
        'valid_from,
        'registered_from,
        'temporary_until,
        'range,
        'activity,
        'operation,
    >,
    ) -> Result<ResponseContent<SitesSuccess>, Error<SitesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/api/v1/sites", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = params.nr {
            local_var_req_builder =
                local_var_req_builder.query(&[("nr", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = params.legal_entity_nr_id {
            local_var_req_builder =
                local_var_req_builder.query(&[("legal-entity-nr-id", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = params.legal_entity_nr {
            local_var_req_builder =
                local_var_req_builder.query(&[("legal-entity-nr", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = params.license_nr {
            local_var_req_builder =
                local_var_req_builder.query(&[("license-nr", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = params.name {
            local_var_req_builder =
                local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = params.placement {
            local_var_req_builder =
                local_var_req_builder.query(&[("placement", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = params.water {
            local_var_req_builder =
                local_var_req_builder.query(&[("water", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = params.species_type {
            local_var_req_builder =
                local_var_req_builder.query(&[("species-type", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = params.municipality_code {
            local_var_req_builder =
                local_var_req_builder.query(&[("municipality-code", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = params.county_code {
            local_var_req_builder =
                local_var_req_builder.query(&[("county-code", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = params.production_area_code {
            local_var_req_builder = local_var_req_builder
                .query(&[("production-area-code", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = params.valid_from {
            local_var_req_builder =
                local_var_req_builder.query(&[("valid-from", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = params.registered_from {
            local_var_req_builder =
                local_var_req_builder.query(&[("registered-from", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = params.temporary_until {
            local_var_req_builder =
                local_var_req_builder.query(&[("temporary-until", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = params.range {
            local_var_req_builder =
                local_var_req_builder.query(&[("range", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = params.activity {
            local_var_req_builder =
                local_var_req_builder.query(&[("activity", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = params.operation {
            local_var_req_builder =
                local_var_req_builder.query(&[("operation", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<SitesSuccess> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SitesError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }
}

/// struct for typed successes of method [`get_borders_by_site_nr`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBordersBySiteNrSuccess {
    Status200(Vec<models::SiteBorder>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_by_site_nr`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBySiteNrSuccess {
    Status200(models::Site),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_decisions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDecisionsSuccess {
    Status200(Vec<models::SiteDecision>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_license_connection_for_site_by_site_nr`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLicenseConnectionForSiteBySiteNrSuccess {
    Status200(Vec<models::LicenseConnectionForSite>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`sites`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SitesSuccess {
    Status200(Vec<models::Site>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_borders_by_site_nr`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBordersBySiteNrError {
    Status400(std::collections::HashMap<String, Vec<String>>),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_by_site_nr`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBySiteNrError {
    Status400(std::collections::HashMap<String, Vec<String>>),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_decisions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDecisionsError {
    Status400(std::collections::HashMap<String, Vec<String>>),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_license_connection_for_site_by_site_nr`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLicenseConnectionForSiteBySiteNrError {
    Status400(std::collections::HashMap<String, Vec<String>>),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sites`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SitesError {
    Status400(std::collections::HashMap<String, Vec<String>>),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
