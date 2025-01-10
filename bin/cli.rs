use akvakulturregisteret_rs::{
    apis::{
        configuration::Configuration,
        csv_dump_resource_api::{GetNewLegacyCsvDumpError, GetNewLegacyCsvDumpSuccess},
        entity_resource_api::{
            EntityResourceApi, EntityResourceApiParams, GetSitesForEntityApiParams,
        },
        license_resource_api::{GetLicensesParams, LicenseResourceApi},
        site_resource_api::SitesApiParams,
        Api, ApiClient, ResponseContent,
    },
    models::entity,
};
use clap::{Parser, Subcommand};
use csv::{Reader, ReaderBuilder, StringRecord, Writer};
use reqwest::StatusCode;
use serde_json::{json, Value};
use std::io::Write;
use std::io::{self, Cursor};
use std::sync::Arc;
use std::{fs::File, path::PathBuf};

#[derive(clap::ValueEnum, Clone, Debug)]
enum OutputFormat {
    Json,
    Csv,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Output file path. If not specified, prints JSON to stdout
    #[arg(long, short)]
    output: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List entities with filters
    Entities {
        #[arg(long, short = 'i')]
        entity_nr_id: Option<String>,
        #[arg(long)]
        entity_nr: Option<String>,
        #[arg(long, short)]
        name: Option<String>,
        #[arg(long)]
        zip_code: Option<String>,
        #[arg(long)]
        zip_name: Option<String>,
        #[arg(long)]
        county_code: Option<String>,
        #[arg(long)]
        county_name: Option<String>,
        #[arg(long)]
        country_code: Option<String>,
        #[arg(long)]
        country_name: Option<String>,
        #[arg(long)]
        range: Option<String>,
    },

    /// Get a single entity by ID
    Entity {
        /// Entity number ID
        entity_nr_id: String,
    },

    /// Get sites for an entity
    EntitySites {
        /// Entity number ID
        entity_nr_id: String,
        #[arg(long)]
        range: Option<String>,
        #[arg(long)]
        include_all_connections: Option<String>,
        #[arg(long)]
        activity: Option<String>,
        #[arg(long)]
        operation: Option<String>,
    },

    /// Get sites for an entity by legal entity number
    EntitySitesByNr {
        /// Entity number
        entity_nr: String,
        #[arg(long)]
        range: Option<String>,
        #[arg(long)]
        include_all_connections: Option<String>,
        #[arg(long)]
        activity: Option<String>,
        #[arg(long)]
        operation: Option<String>,
    },

    /// List all sites with filters
    Sites {
        #[arg(long)]
        nr: Option<String>,
        #[arg(long)]
        legal_entity_nr_id: Option<String>,
        #[arg(long)]
        legal_entity_nr: Option<String>,
        #[arg(long)]
        license_nr: Option<String>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        placement: Option<String>,
        #[arg(long)]
        water: Option<String>,
        #[arg(long)]
        species_type: Option<String>,
        #[arg(long)]
        municipality_code: Option<String>,
        #[arg(long)]
        county_code: Option<String>,
        #[arg(long)]
        production_area_code: Option<String>,
        #[arg(long)]
        valid_from: Option<String>,
        #[arg(long)]
        registered_from: Option<String>,
        #[arg(long)]
        temporary_until: Option<String>,
        #[arg(long)]
        range: Option<String>,
        #[arg(long)]
        activity: Option<String>,
        #[arg(long)]
        operation: Option<String>,
    },

    /// Get a single site by number
    Site {
        /// Site number
        site_nr: String,
    },

    /// Get site borders
    SiteBorders {
        /// Site number
        site_nr: String,
    },

    /// Get site decisions
    SiteDecisions {
        /// Site number
        site_nr: String,
    },

    /// Get license connections for a site
    SiteLicenses {
        /// Site number
        site_nr: String,
    },

    /// Get licenses
    License {
        /// License number
        #[arg(long, short = 'n')]
        license_nr: Option<String>,

        /// Legal entity number
        #[arg(long)]
        legal_entity_nr: Option<String>,

        /// Legal entity name
        #[arg(long)]
        legal_entity_name: Option<String>,
    },

    /// Get area by version ID
    Area {
        /// Version ID
        version_id: String,
    },

    /// Get area by code and type
    AreaByCode {
        /// Area type
        #[arg(long)]
        r#type: String,
        /// Area code
        #[arg(long)]
        code: String,
        /// Time
        #[arg(long)]
        time: Option<String>,
    },

    /// List areas with filters
    Areas {
        /// Area type
        #[arg(long)]
        r#type: Option<String>,
        /// Area code
        #[arg(long)]
        code: Option<String>,
        /// Time
        #[arg(long)]
        time: Option<String>,
        /// Range
        #[arg(long)]
        range: Option<String>,
    },

    /// Get license details and its transfers
    LicenseTransfers {
        /// License number
        license_nr: String,
    },

    /// Get license by number
    LicenseByNr {
        /// License number
        license_nr: String,
    },

    /// Get license decisions
    LicenseDecisions {
        /// License number
        license_nr: String,
    },

    /// Get license capacity history
    LicenseCapacityHistory {
        /// License number
        license_nr: String,
    },

    /// Get license site connections
    LicenseSiteConnections {
        /// License number
        license_nr: String,
    },

    /// Get license overview
    LicensesOverview {
        #[arg(long)]
        license_nr: Option<String>,
        #[arg(long)]
        legal_entity_nr: Option<String>,
        #[arg(long)]
        legal_entity_name: Option<String>,
        #[arg(long)]
        portfolio_type: Option<String>,
        #[arg(long)]
        tag: Option<String>,
        #[arg(long)]
        not_tag: Option<String>,
        #[arg(long)]
        intention: Option<String>,
        #[arg(long)]
        produces: Option<String>,
        #[arg(long)]
        range: Option<String>,
    },

    /// Get license liens
    LicenseLiens {
        /// License number
        license_nr: String,
    },

    /// Get CSV/JSON dump of the database
    Dump {
        /// Output file path. If not provided, prints to stdout
        #[arg(long, short)]
        output: Option<PathBuf>,
    },
}

impl Commands {
    fn handle_csv_to_json(csv_data: &str) -> Result<Value, Box<dyn std::error::Error>> {
        // Skip the first line and remove any empty lines
        let content_lines: Vec<&str> = csv_data
            .lines()
            .skip(1) // Skip the title row
            .filter(|line| !line.trim().is_empty())
            .collect();

        // Create a new string with the filtered content
        let filtered_content = content_lines.join("\n");

        // Configure CSV reader with appropriate settings
        let mut csv_reader = ReaderBuilder::new()
            .delimiter(b';')
            .flexible(true) // Handle variable number of fields
            .trim(csv::Trim::All) // Trim whitespace
            .from_reader(filtered_content.as_bytes());

        let headers: Vec<String> = csv_reader
            .headers()
            .unwrap_or(&StringRecord::new())
            .iter()
            .map(|header| header.to_string())
            .collect();

        let records: Result<Vec<Value>, _> = csv_reader
            .records()
            .map(|record| {
                let record = record?;

                let obj: Value = json!(headers
                    .iter()
                    .zip(record.iter())
                    .map(|(header, value)| (header.clone(), Value::String(value.to_string())))
                    .collect::<serde_json::Map<String, Value>>());
                Ok::<Value, csv::Error>(obj)
            })
            .collect();

        let json_array = Value::Array(records.map_err(|e| {
            akvakulturregisteret_rs::apis::Error::ResponseError(ResponseContent {
                status: StatusCode::OK,
                content: "csv parsing error".to_string(),
                entity: Some(GetNewLegacyCsvDumpError::UnknownValue(
                    json!({"error": e.to_string()}),
                )),
            })
        })?);

        Ok(json_array)
    }
    fn handle_csv_output(
        csv_data: &str,
        output: &Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create a CSV reader from the string data
        let mut rdr = Reader::from_reader(Cursor::new(csv_data));

        match output {
            Some(path) => {
                // Write to file
                let mut wtr = Writer::from_path(path)?;

                // Copy headers
                let headers = rdr.headers()?.clone();
                wtr.write_record(&headers)?;

                // Copy all records
                for result in rdr.records() {
                    let record = result?;
                    wtr.write_record(&record)?;
                }

                wtr.flush()?;
                println!("CSV file written to: {}", path);
            }
            None => {
                // Pretty print to terminal
                let headers = rdr.headers()?;
                println!("{}", headers.iter().collect::<Vec<_>>().join("\t"));

                for result in rdr.records() {
                    let record = result?;
                    println!("{}", record.iter().collect::<Vec<_>>().join("\t"));
                }
            }
        }

        Ok(())
    }

    fn handle_output<T: serde::Serialize>(
        data: &T,
        output: &Option<PathBuf>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match output {
            Some(path) => {
                match path.extension().and_then(|ext| ext.to_str()) {
                    Some("csv") => {
                        let mut wtr = csv::Writer::from_path(path)?;
                        wtr.serialize(data)?;
                        wtr.flush()?;
                        println!("CSV file written to: {}", path.display());
                    }
                    _ => {
                        // Default to JSON for unknown extensions
                        let json = serde_json::to_string_pretty(&data)?;
                        std::fs::write(path, json)?;
                        println!("JSON file written to: {}", path.display());
                    }
                }
            }
            None => {
                // Print JSON to stdout
                println!("{}", serde_json::to_string_pretty(&data)?);
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config = Configuration::default();
    let client = ApiClient::new(Arc::new(config));

    match &cli.command {
        Commands::Entities {
            entity_nr_id,
            entity_nr,
            name,
            zip_code,
            zip_name,
            county_code,
            county_name,
            country_code,
            country_name,
            range,
        } => {
            let params = EntityResourceApiParams::builder()
                .maybe_entity_nr(entity_nr.as_deref())
                .maybe_entity_nr_id(entity_nr_id.as_deref())
                .maybe_name(name.as_deref())
                .maybe_zip_code(zip_code.as_deref())
                .maybe_zip_name(zip_name.as_deref())
                .maybe_county_code(county_code.as_deref())
                .maybe_county_name(county_name.as_deref())
                .maybe_country_code(country_code.as_deref())
                .maybe_country_name(country_name.as_deref())
                .maybe_range(range.as_deref())
                .build();

            let result = client.entity_resource_api().entities(params).await?;
            if let Some(entities) = result.entity {
                Commands::handle_output(&entities, &cli.output)?;
            }
        }

        Commands::Entity { entity_nr_id } => {
            let result = client
                .entity_resource_api()
                .get_entity(entity_nr_id)
                .await?;
            if let Some(entity) = result.entity {
                Commands::handle_output(&entity, &cli.output)?;
            }
        }

        Commands::EntitySites {
            entity_nr_id,
            range,
            include_all_connections,
            activity,
            operation,
        } => {
            let params = GetSitesForEntityApiParams::builder()
                .entity_nr_id(entity_nr_id)
                .maybe_range(range.as_deref())
                .maybe_activity(activity.as_deref())
                .maybe_include_all_connections(include_all_connections.as_deref())
                .maybe_operation(operation.as_deref())
                .build();

            let result = client
                .entity_resource_api()
                .get_sites_for_entity(params)
                .await?;
            if let Some(sites) = result.entity {
                Commands::handle_output(&sites, &cli.output)?;
            }
        }

        Commands::EntitySitesByNr {
            entity_nr,
            range,
            include_all_connections,
            activity,
            operation,
        } => {
            let params = GetSitesForEntityApiParams::builder()
                .entity_nr_id(entity_nr)
                .maybe_range(range.as_deref())
                .maybe_activity(activity.as_deref())
                .maybe_include_all_connections(include_all_connections.as_deref())
                .maybe_operation(operation.as_deref())
                .build();

            let result = client
                .entity_resource_api()
                .get_sites_for_entity_by_legal_entity_nr(params)
                .await?;
            if let Some(sites) = result.entity {
                Commands::handle_output(&sites, &cli.output)?;
            }
        }

        Commands::Site { site_nr } => {
            let result = client.site_resource_api().get_by_site_nr(site_nr).await?;
            if let Some(site) = result.entity {
                Commands::handle_output(&site, &cli.output)?;
            }
        }

        Commands::SiteBorders { site_nr } => {
            let result = client
                .site_resource_api()
                .get_borders_by_site_nr(site_nr)
                .await?;
            if let Some(borders) = result.entity {
                Commands::handle_output(&borders, &cli.output)?;
            }
        }

        Commands::SiteDecisions { site_nr } => {
            let result = client.site_resource_api().get_decisions(site_nr).await?;
            if let Some(decisions) = result.entity {
                Commands::handle_output(&decisions, &cli.output)?;
            }
        }

        Commands::SiteLicenses { site_nr } => {
            let result = client
                .site_resource_api()
                .get_license_connection_for_site_by_site_nr(site_nr)
                .await?;
            if let Some(licenses) = result.entity {
                Commands::handle_output(&licenses, &cli.output)?;
            }
        }

        Commands::Sites {
            nr,
            legal_entity_nr_id,
            legal_entity_nr,
            license_nr,
            name,
            placement,
            water,
            species_type,
            municipality_code,
            county_code,
            production_area_code,
            valid_from,
            registered_from,
            temporary_until,
            range,
            activity,
            operation,
        } => {
            let params = SitesApiParams::builder()
                .maybe_nr(nr.as_deref())
                .maybe_legal_entity_nr_id(legal_entity_nr_id.as_deref())
                .maybe_legal_entity_nr(legal_entity_nr.as_deref())
                .maybe_license_nr(license_nr.as_deref())
                .maybe_name(name.as_deref())
                .maybe_placement(placement.as_deref())
                .maybe_water(water.as_deref())
                .maybe_species_type(species_type.as_deref())
                .maybe_municipality_code(municipality_code.as_deref())
                .maybe_county_code(county_code.as_deref())
                .maybe_production_area_code(production_area_code.as_deref())
                .maybe_valid_from(valid_from.as_deref())
                .maybe_registered_from(registered_from.as_deref())
                .maybe_temporary_until(temporary_until.as_deref())
                .maybe_range(range.as_deref())
                .maybe_activity(activity.as_deref())
                .maybe_operation(operation.as_deref())
                .build();

            let result = client.site_resource_api().sites(params).await?;
            if let Some(sites) = result.entity {
                Commands::handle_output(&sites, &cli.output)?;
            }
        }

        Commands::License {
            license_nr,
            legal_entity_nr,
            legal_entity_name,
        } => {
            let params = GetLicensesParams::builder()
                .maybe_license_nr(license_nr.as_deref())
                .maybe_legal_entity_nr(legal_entity_nr.as_deref())
                .maybe_legal_entity_name(legal_entity_name.as_deref())
                .build();

            let result = client.license_resource_api().get_licenses(params).await?;
            if let Some(licenses) = result.entity {
                Commands::handle_output(&licenses, &cli.output)?;
            }
        }

        Commands::Area { version_id } => {
            let result = client.area_resource_api().area(version_id).await?;
            if let Some(area) = result.entity {
                Commands::handle_output(&area, &cli.output)?;
            }
        }

        Commands::AreaByCode { r#type, code, time } => {
            let result = client
                .area_resource_api()
                .area_by_code_and_type(r#type, code, time.as_deref())
                .await?;
            if let Some(area) = result.entity {
                Commands::handle_output(&area, &cli.output)?;
            }
        }

        Commands::Areas {
            r#type,
            code,
            time,
            range,
        } => {
            let result = client
                .area_resource_api()
                .areas(
                    r#type.as_deref(),
                    code.as_deref(),
                    time.as_deref(),
                    range.as_deref(),
                )
                .await?;
            if let Some(areas) = result.entity {
                Commands::handle_output(&areas, &cli.output)?;
            }
        }

        Commands::LicenseTransfers { license_nr } => {
            let result = client
                .license_resource_api()
                .get_ajour_transfers(license_nr)
                .await?;
            if let Some(transfers) = result.entity {
                Commands::handle_output(&transfers, &cli.output)?;
            }
        }

        Commands::LicenseByNr { license_nr } => {
            let result = client
                .license_resource_api()
                .get_by_license_nr(license_nr)
                .await?;
            if let Some(license) = result.entity {
                Commands::handle_output(&license, &cli.output)?;
            }
        }

        Commands::LicenseDecisions { license_nr } => {
            let result = client
                .license_resource_api()
                .get_decisions1(license_nr)
                .await?;
            if let Some(decisions) = result.entity {
                Commands::handle_output(&decisions, &cli.output)?;
            }
        }

        Commands::LicenseCapacityHistory { license_nr } => {
            let result = client
                .license_resource_api()
                .get_license_capacity_history_by_license_nr(license_nr)
                .await?;
            if let Some(history) = result.entity {
                Commands::handle_output(&history, &cli.output)?;
            }
        }

        Commands::LicenseSiteConnections { license_nr } => {
            let result = client
                .license_resource_api()
                .get_license_connection_for_site_by_site_nr1(license_nr)
                .await?;
            if let Some(connections) = result.entity {
                Commands::handle_output(&connections, &cli.output)?;
            }
        }

        Commands::LicensesOverview {
            license_nr,
            legal_entity_nr,
            legal_entity_name,
            portfolio_type,
            tag,
            not_tag,
            intention,
            produces,
            range,
        } => {
            let params = GetLicensesParams::builder()
                .maybe_license_nr(license_nr.as_deref())
                .maybe_legal_entity_nr(legal_entity_nr.as_deref())
                .maybe_legal_entity_name(legal_entity_name.as_deref())
                .maybe_portfolio_type(portfolio_type.as_deref())
                .maybe_tag(tag.as_deref())
                .maybe_not_tag(not_tag.as_deref())
                .maybe_intention(intention.as_deref())
                .maybe_produces(produces.as_deref())
                .maybe_range(range.as_deref())
                .build();

            let result = client
                .license_resource_api()
                .get_licenses_overview(params)
                .await?;
            if let Some(overview) = result.entity {
                Commands::handle_output(&overview, &cli.output)?;
            }
        }

        Commands::LicenseLiens { license_nr } => {
            let result = client.license_resource_api().get_liens(license_nr).await?;
            if let Some(liens) = result.entity {
                Commands::handle_output(&liens, &cli.output)?;
            }
        }

        Commands::Dump { output } => {
            if let Some(path) = output {
                if path.exists() {
                    eprintln!(
                        "Wardning: Output file already exists, overwriting: {}",
                        path.display()
                    );
                }

                let format = match path.extension().and_then(|ext| ext.to_str()) {
                    Some("json") => OutputFormat::Json,
                    Some("csv") => OutputFormat::Csv,
                    _ => {
                        eprintln!(
                            "Warning: Output file extension not recognized, defaulting to JSON"
                        );
                        OutputFormat::Json
                    }
                };
                match format {
                    OutputFormat::Json => {
                        // The JSON endpoint actually returns CSV text
                        let data = client
                            .csv_dump_resource_api()
                            .get_new_legacy_csv_file_dump()
                            .await?;

                        match output {
                            Some(path) => {
                                if let Ok(json) = String::from_utf8(data) {
                                    let json = Commands::handle_csv_to_json(&json)?;
                                    std::fs::write(path, serde_json::to_string_pretty(&json)?)?;
                                    println!("JSON file written to: {:?}", path.to_str());
                                } else {
                                    println!("Warning: CSV data contains invalid UTF-8");
                                }
                            }
                            None => {
                                if let Ok(json) = String::from_utf8(data) {
                                    let json = Commands::handle_csv_to_json(&json)?;
                                    println!("{}", serde_json::to_string_pretty(&json)?);
                                } else {
                                    println!("Warning: CSV data contains invalid UTF-8");
                                }
                            }
                        }
                    }
                    OutputFormat::Csv => {
                        let data = client
                            .csv_dump_resource_api()
                            .get_new_legacy_csv_file_dump()
                            .await?;
                        match output {
                            Some(path) => {
                                std::fs::write(path, &data)?;
                                println!("CSV file written to: {:?}", path.to_str());
                            }
                            None => {
                                if let Ok(csv_str) = String::from_utf8(data) {
                                    Commands::handle_csv_output(&csv_str, &None)?;
                                } else {
                                    println!("Warning: CSV data contains invalid UTF-8");
                                }
                            }
                        }
                    }
                }
            } else {
                let data = client
                    .csv_dump_resource_api()
                    .get_new_legacy_csv_file_dump()
                    .await?;
                if let Ok(csv_str) = String::from_utf8(data) {
                    let json = Commands::handle_csv_to_json(&csv_str)?;
                    println!("{}", serde_json::to_string_pretty(&json)?);
                } else {
                    println!("Warning: CSV data contains invalid UTF-8");
                }
            }
        }
    }

    Ok(())
}
