use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

pub mod area_resource_api;
pub mod csv_dump_resource_api;
pub mod entity_resource_api;
pub mod license_resource_api;
pub mod license_type_resource_api;
pub mod site_resource_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn area_resource_api(&self) -> &dyn area_resource_api::AreaResourceApi;
    fn csv_dump_resource_api(&self) -> &dyn csv_dump_resource_api::CsvDumpResourceApi;
    fn entity_resource_api(&self) -> &dyn entity_resource_api::EntityResourceApi;
    fn license_resource_api(&self) -> &dyn license_resource_api::LicenseResourceApi;
    fn license_type_resource_api(&self) -> &dyn license_type_resource_api::LicenseTypeResourceApi;
    fn site_resource_api(&self) -> &dyn site_resource_api::SiteResourceApi;
}

pub struct ApiClient {
    area_resource_api: Box<dyn area_resource_api::AreaResourceApi>,
    csv_dump_resource_api: Box<dyn csv_dump_resource_api::CsvDumpResourceApi>,
    entity_resource_api: Box<dyn entity_resource_api::EntityResourceApi>,
    license_resource_api: Box<dyn license_resource_api::LicenseResourceApi>,
    license_type_resource_api: Box<dyn license_type_resource_api::LicenseTypeResourceApi>,
    site_resource_api: Box<dyn site_resource_api::SiteResourceApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            area_resource_api: Box::new(area_resource_api::AreaResourceApiClient::new(
                configuration.clone(),
            )),
            csv_dump_resource_api: Box::new(csv_dump_resource_api::CsvDumpResourceApiClient::new(
                configuration.clone(),
            )),
            entity_resource_api: Box::new(entity_resource_api::EntityResourceApiClient::new(
                configuration.clone(),
            )),
            license_resource_api: Box::new(license_resource_api::LicenseResourceApiClient::new(
                configuration.clone(),
            )),
            license_type_resource_api: Box::new(
                license_type_resource_api::LicenseTypeResourceApiClient::new(configuration.clone()),
            ),
            site_resource_api: Box::new(site_resource_api::SiteResourceApiClient::new(
                configuration.clone(),
            )),
        }
    }
}

impl Api for ApiClient {
    fn area_resource_api(&self) -> &dyn area_resource_api::AreaResourceApi {
        self.area_resource_api.as_ref()
    }
    fn csv_dump_resource_api(&self) -> &dyn csv_dump_resource_api::CsvDumpResourceApi {
        self.csv_dump_resource_api.as_ref()
    }
    fn entity_resource_api(&self) -> &dyn entity_resource_api::EntityResourceApi {
        self.entity_resource_api.as_ref()
    }
    fn license_resource_api(&self) -> &dyn license_resource_api::LicenseResourceApi {
        self.license_resource_api.as_ref()
    }
    fn license_type_resource_api(&self) -> &dyn license_type_resource_api::LicenseTypeResourceApi {
        self.license_type_resource_api.as_ref()
    }
    fn site_resource_api(&self) -> &dyn site_resource_api::SiteResourceApi {
        self.site_resource_api.as_ref()
    }
}

#[cfg(feature = "mockall")]
pub struct MockApiClient {
    pub area_resource_api_mock: area_resource_api::MockAreaResourceApi,
    pub csv_dump_resource_api_mock: csv_dump_resource_api::MockCsvDumpResourceApi,
    pub entity_resource_api_mock: entity_resource_api::MockEntityResourceApi,
    pub license_resource_api_mock: license_resource_api::MockLicenseResourceApi,
    pub license_type_resource_api_mock: license_type_resource_api::MockLicenseTypeResourceApi,
    pub site_resource_api_mock: site_resource_api::MockSiteResourceApi,
}

#[cfg(feature = "mockall")]
impl MockApiClient {
    pub fn new() -> Self {
        Self {
            area_resource_api_mock: area_resource_api::MockAreaResourceApi::new(),
            csv_dump_resource_api_mock: csv_dump_resource_api::MockCsvDumpResourceApi::new(),
            entity_resource_api_mock: entity_resource_api::MockEntityResourceApi::new(),
            license_resource_api_mock: license_resource_api::MockLicenseResourceApi::new(),
            license_type_resource_api_mock:
                license_type_resource_api::MockLicenseTypeResourceApi::new(),
            site_resource_api_mock: site_resource_api::MockSiteResourceApi::new(),
        }
    }
}

#[cfg(feature = "mockall")]
impl Api for MockApiClient {
    fn area_resource_api(&self) -> &dyn area_resource_api::AreaResourceApi {
        &self.area_resource_api_mock
    }
    fn csv_dump_resource_api(&self) -> &dyn csv_dump_resource_api::CsvDumpResourceApi {
        &self.csv_dump_resource_api_mock
    }
    fn entity_resource_api(&self) -> &dyn entity_resource_api::EntityResourceApi {
        &self.entity_resource_api_mock
    }
    fn license_resource_api(&self) -> &dyn license_resource_api::LicenseResourceApi {
        &self.license_resource_api_mock
    }
    fn license_type_resource_api(&self) -> &dyn license_type_resource_api::LicenseTypeResourceApi {
        &self.license_type_resource_api_mock
    }
    fn site_resource_api(&self) -> &dyn site_resource_api::SiteResourceApi {
        &self.site_resource_api_mock
    }
}
