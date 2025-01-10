# Rust API client for akvakulturregisteret_rs

API client for the norwegian aquaculture registry.

## Install

### Install prebuilt binaries via shell script

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/EmilLindfors/akvakulturregisteret_rs/releases/download/v0.3.0/akvakulturregisteret_rs-installer.sh | sh
```

### Install prebuilt binaries via powershell script

```sh
powershell -ExecutionPolicy ByPass -c "irm https://github.com/EmilLindfors/akvakulturregisteret_rs/releases/download/v0.3.0/akvakulturregisteret_rs-installer.ps1 | iex"
```

### Install prebuilt binaries into your npm project

```sh
npm install akvakulturregisteret_rs@0.3.0
```

## Download akvakulturregisteret_rs

| File                                                                                                                                                                                                | Platform            | Checksum                                                                                                                                                    |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [akvakulturregisteret_rs-aarch64-apple-darwin.tar.xz](https://github.com/EmilLindfors/akvakulturregisteret_rs/releases/download/v0.3.0/akvakulturregisteret_rs-aarch64-apple-darwin.tar.xz)         | Apple Silicon macOS | [checksum](https://github.com/EmilLindfors/akvakulturregisteret_rs/releases/download/v0.3.0/akvakulturregisteret_rs-aarch64-apple-darwin.tar.xz.sha256)     |
| [akvakulturregisteret_rs-x86_64-apple-darwin.tar.xz](https://github.com/EmilLindfors/akvakulturregisteret_rs/releases/download/v0.3.0/akvakulturregisteret_rs-x86_64-apple-darwin.tar.xz)           | Intel macOS         | [checksum](https://github.com/EmilLindfors/akvakulturregisteret_rs/releases/download/v0.3.0/akvakulturregisteret_rs-x86_64-apple-darwin.tar.xz.sha256)      |
| [akvakulturregisteret_rs-x86_64-pc-windows-msvc.zip](https://github.com/EmilLindfors/akvakulturregisteret_rs/releases/download/v0.3.0/akvakulturregisteret_rs-x86_64-pc-windows-msvc.zip)           | x64 Windows         | [checksum](https://github.com/EmilLindfors/akvakulturregisteret_rs/releases/download/v0.3.0/akvakulturregisteret_rs-x86_64-pc-windows-msvc.zip.sha256)      |
| [akvakulturregisteret_rs-x86_64-pc-windows-msvc.msi](https://github.com/EmilLindfors/akvakulturregisteret_rs/releases/download/v0.3.0/akvakulturregisteret_rs-x86_64-pc-windows-msvc.msi)           | x64 Windows         | [checksum](https://github.com/EmilLindfors/akvakulturregisteret_rs/releases/download/v0.3.0/akvakulturregisteret_rs-x86_64-pc-windows-msvc.msi.sha256)      |
| [akvakulturregisteret_rs-x86_64-unknown-linux-gnu.tar.xz](https://github.com/EmilLindfors/akvakulturregisteret_rs/releases/download/v0.3.0/akvakulturregisteret_rs-x86_64-unknown-linux-gnu.tar.xz) | x64 Linux           | [checksum](https://github.com/EmilLindfors/akvakulturregisteret_rs/releases/download/v0.3.0/akvakulturregisteret_rs-x86_64-unknown-linux-gnu.tar.xz.sha256) |

## CLI Usage

# Get JSON CSV dump

areg csv-dump

# Download CSV file

areg csv-file-dump -o output.csv

# Area commands

```
areg area <version-id>
areg area-by-code --type <type> --code <code> [--time <time>]
areg areas [--type <type>] [--code <code>] [--time <time>] [--range <range>]
```

# License commands

```
areg license-transfers <license-nr>
areg license-by-nr <license-nr>
areg license-decisions <license-nr>
areg license-capacity-history <license-nr>
areg license-site-connections <license-nr>
areg licenses-overview [--license-nr <nr>] [--legal-entity-nr <nr>] [--legal-entity-name <name>] ...
areg license-liens <license-nr>
```

etc.

## Library usage

All URIs are relative to _https://api.fiskeridir.no/pub-aqua_

| Class                    | Method                                                                                                                    | HTTP request                                            | Description                                                                                                        |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| _AreaResourceApi_        | [**area**](docs/AreaResourceApi.md#area)                                                                                  | **GET** /api/v1/areas/{version-id}                      | Hent et spesifikt område                                                                                           |
| _AreaResourceApi_        | [**area_by_code_and_type**](docs/AreaResourceApi.md#area_by_code_and_type)                                                | **GET** /api/v1/areas/{type}/{code}                     | Hent et spesifikt område etter type og kode                                                                        |
| _AreaResourceApi_        | [**areas**](docs/AreaResourceApi.md#areas)                                                                                | **GET** /api/v1/areas                                   | Henter en liste av områder                                                                                         |
| _CsvDumpResourceApi_     | [**get_new_legacy_csv_dump**](docs/CsvDumpResourceApi.md#get_new_legacy_csv_dump)                                         | **GET** /api/v1/dump/new-legacy-csv                     | Oppretter en CSV-dump av akvakulturregisteret                                                                      |
| _CsvDumpResourceApi_     | [**get_new_legacy_csv_file_dump**](docs/CsvDumpResourceApi.md#get_new_legacy_csv_file_dump)                               | **GET** /api/v1/dump/new-legacy-csv-file                | Oppretter en CSV-fildump av akvakulturregisteret                                                                   |
| _EntityResourceApi_      | [**entities**](docs/EntityResourceApi.md#entities)                                                                        | **GET** /api/v1/entities                                | Hent en liste av juridiske enheter                                                                                 |
| _EntityResourceApi_      | [**get_entity**](docs/EntityResourceApi.md#get_entity)                                                                    | **GET** /api/v1/entities/{entity-nr-id}                 | Hent angitt juridisk enhet                                                                                         |
| _EntityResourceApi_      | [**get_sites_for_entity**](docs/EntityResourceApi.md#get_sites_for_entity)                                                | **GET** /api/v1/entities/{entity-nr-id}/sites           | Hent en liste av lokaliteter hvor en juridisk enhet driver med akvakultur                                          |
| _EntityResourceApi_      | [**get_sites_for_entity_by_legal_entity_nr**](docs/EntityResourceApi.md#get_sites_for_entity_by_legal_entity_nr)          | **GET** /api/v1/entities/sites-by-entity-nr/{entity-nr} | Henter en liste av lokaliteter benyttet av juridisk enhet identifisert ved fødselsnummer eller organisasjonsnummer |
| _LicenseResourceApi_     | [**get_ajour_transfers**](docs/LicenseResourceApi.md#get_ajour_transfers)                                                 | **GET** /api/v1/licenses/{license-nr}/transfers         | Hent ajour-dato på overføringer mellom juridiske enheter av tillatelse på tillatelsesnummer                        |
| _LicenseResourceApi_     | [**get_by_license_nr**](docs/LicenseResourceApi.md#get_by_license_nr)                                                     | **GET** /api/v1/licenses/{license-nr}                   | Hent en tillatelse etter tillatelsesnummer                                                                         |
| _LicenseResourceApi_     | [**get_decisions1**](docs/LicenseResourceApi.md#get_decisions1)                                                           | **GET** /api/v1/licenses/{license-nr}/decisions         | Hent vedtak for tillatelse etter tillatelsesnummer                                                                 |
| _LicenseResourceApi_     | [**get_license_capacity_history_by_license_nr**](docs/LicenseResourceApi.md#get_license_capacity_history_by_license_nr)   | **GET** /api/v1/licenses/{license-nr}/capacity-history  | Hent kapasistetshistorien for tillatelse etter tillatelsesnummer                                                   |
| _LicenseResourceApi_     | [**get_license_connection_for_site_by_site_nr1**](docs/LicenseResourceApi.md#get_license_connection_for_site_by_site_nr1) | **GET** /api/v1/licenses/{license-nr}/site-connections  | Hent lokalitetstilknytninger for tillatelse etter tillatelsesnummer                                                |
| _LicenseResourceApi_     | [**get_licenses**](docs/LicenseResourceApi.md#get_licenses)                                                               | **GET** /api/v1/licenses                                | Hent alle tillatelser som tilfredsstiller kriteriene for søkeparametere                                            |
| _LicenseResourceApi_     | [**get_licenses_overview**](docs/LicenseResourceApi.md#get_licenses_overview)                                             | **GET** /api/v1/licenses-overview                       | Hent alle tillatelser som tilfredsstiller kriteriene for søkeparametere                                            |
| _LicenseResourceApi_     | [**get_liens**](docs/LicenseResourceApi.md#get_liens)                                                                     | **GET** /api/v1/licenses/{license-nr}/liens             | Hent panterettholdere av tillatelse etter tillatelsesnummer                                                        |
| _LicenseTypeResourceApi_ | [**get_license_intention**](docs/LicenseTypeResourceApi.md#get_license_intention)                                         | **GET** /api/v1/license-types/intentions/{id}           | Hent formålet med en tillatelsestype basert på ID                                                                  |
| _LicenseTypeResourceApi_ | [**get_license_intentions**](docs/LicenseTypeResourceApi.md#get_license_intentions)                                       | **GET** /api/v1/license-types/intentions                | Hent en liste av formål for hver tillatelsestype                                                                   |
| _SiteResourceApi_        | [**get_borders_by_site_nr**](docs/SiteResourceApi.md#get_borders_by_site_nr)                                              | **GET** /api/v1/sites/{site-nr}/borders                 | Hent grense rundt en lokalitet etter lokalitetsnummer                                                              |
| _SiteResourceApi_        | [**get_by_site_nr**](docs/SiteResourceApi.md#get_by_site_nr)                                                              | **GET** /api/v1/sites/{site-nr}                         | Hent lokalitet etter lokalitetsnummer                                                                              |
| _SiteResourceApi_        | [**get_decisions**](docs/SiteResourceApi.md#get_decisions)                                                                | **GET** /api/v1/sites/{site-nr}/decisions               | Hent vedtak for lokalitet etter lokalitetsnummer                                                                   |
| _SiteResourceApi_        | [**get_license_connection_for_site_by_site_nr**](docs/SiteResourceApi.md#get_license_connection_for_site_by_site_nr)      | **GET** /api/v1/sites/{site-nr}/license-connections     | Hent tillatelsesforbindelser for lokalitet etter lokalitetsnummer                                                  |
| _SiteResourceApi_        | [**sites**](docs/SiteResourceApi.md#sites)                                                                                | **GET** /api/v1/sites                                   | Hent en liste av lokaliteter                                                                                       |

## Documentation For Models

- [Address](docs/Address.md)
- [AreaListItem](docs/AreaListItem.md)
- [AreaPlacement](docs/AreaPlacement.md)
- [BorderPoint](docs/BorderPoint.md)
- [BorderType](docs/BorderType.md)
- [BusinessType](docs/BusinessType.md)
- [CapacityInfo](docs/CapacityInfo.md)
- [Entity](docs/Entity.md)
- [ErrorResponse](docs/ErrorResponse.md)
- [FishCode](docs/FishCode.md)
- [GrantInfo](docs/GrantInfo.md)
- [IntentionType](docs/IntentionType.md)
- [LatestLicenseSiteConnectionDetail](docs/LatestLicenseSiteConnectionDetail.md)
- [LatestLicenseSiteConnectionOverview](docs/LatestLicenseSiteConnectionOverview.md)
- [LicenseCapacityHistory](docs/LicenseCapacityHistory.md)
- [LicenseConnectionForSite](docs/LicenseConnectionForSite.md)
- [LicenseDecision](docs/LicenseDecision.md)
- [LicenseDetail](docs/LicenseDetail.md)
- [LicenseOverview](docs/LicenseOverview.md)
- [LicensePortfolioType](docs/LicensePortfolioType.md)
- [LicenseTransfers](docs/LicenseTransfers.md)
- [LicenseTypeDetail](docs/LicenseTypeDetail.md)
- [LicenseTypeOverview](docs/LicenseTypeOverview.md)
- [Lien](docs/Lien.md)
- [LienHolders](docs/LienHolders.md)
- [Lienholder](docs/Lienholder.md)
- [RegisteredLicenseOwner](docs/RegisteredLicenseOwner.md)
- [Site](docs/Site.md)
- [SiteBorder](docs/SiteBorder.md)
- [SiteConnectionForLicense](docs/SiteConnectionForLicense.md)
- [SiteDecision](docs/SiteDecision.md)
- [SiteForLegalEntity](docs/SiteForLegalEntity.md)
- [SpeciesDetail](docs/SpeciesDetail.md)
- [SpeciesOverview](docs/SpeciesOverview.md)
- [Transfer](docs/Transfer.md)
- [VersionDetail](docs/VersionDetail.md)
- [VersionOverview](docs/VersionOverview.md)

To get access to the crate's generated documentation, use:

```
cargo doc --open
```
