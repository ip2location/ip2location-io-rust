use serde::{Deserialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct IpGeolocationResult {
    pub ip: Option<String>,
    pub country_code: Option<String>,
    pub country_name: Option<String>,
    pub region_name: Option<String>,
    pub district: Option<String>,
    pub city_name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub zip_code: Option<String>,
    pub time_zone: Option<String>,
    pub asn: Option<String>,
    #[serde(rename = "as")]
    pub as_name: Option<String>,
    pub isp: Option<String>,
    pub domain: Option<String>,
    pub net_speed: Option<String>,
    pub idd_code: Option<String>,
    pub area_code: Option<String>,
    pub weather_station_code: Option<String>,
    pub weather_station_name: Option<String>,
    pub mcc: Option<String>,
    pub mnc: Option<String>,
    pub mobile_brand: Option<String>,
    pub elevation: Option<u8>,
    pub usage_type: Option<String>,
    pub address_type: Option<String>,
    pub ads_category: Option<String>,
    pub ads_category_name: Option<String>,
    pub continent: Continent,
    pub country: Country,
    pub region: Region,
    pub city: City,
    pub time_zone_info: TimeZoneInfo,
    pub geotargeting: Geotargeting,
    pub is_proxy: Option<bool>,
    pub fraud_score: Option<u8>,
    pub proxy: Proxy,
    // #[serde(flatten)]
    // pub extra: Value, // Any additional fields depending on plan
}

#[derive(Debug, Deserialize)]
pub struct Continent {
    pub name: Option<String>,
    pub code: Option<String>,
    pub hemisphere: Vec<String>,
    pub translation: Translation,
}

#[derive(Debug, Deserialize)]
pub struct Translation {
    pub lang: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Country {
    pub name: Option<String>,
    pub alpha3_code: Option<String>,
    pub numeric_code: Option<i32>,
    pub demonym: Option<String>,
    pub flag: Option<String>,
    pub capital: Option<String>,
    pub total_area: Option<i32>,
    pub population: Option<i32>,
    pub currency: Currency,
    pub language: Language,
    pub tld: Option<String>,
    pub translation: Translation,
}

#[derive(Debug, Deserialize)]
pub struct Currency {
    pub code: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Language {
    pub code: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Region {
    pub name: Option<String>,
    pub code: Option<String>,
    pub translation: Translation,
}

#[derive(Debug, Deserialize)]
pub struct City {
    pub name: Option<String>,
    pub translation: Translation,
}

#[derive(Debug, Deserialize)]
pub struct Geotargeting {
    pub metro: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TimeZoneInfo {
    pub olson: Option<String>,
    pub current_time: Option<String>,
    pub gmt_offset: Option<i32>,
    pub is_dst: Option<bool>,
    pub abbreviation: Option<String>,
    pub dst_start_date: Option<String>,
    pub dst_end_date: Option<String>,
    pub sunrise: Option<String>,
    pub sunset: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Proxy {
    pub last_seen: Option<i32>,
    pub proxy_type: Option<String>,
    pub threat: Option<String>,
    pub provider: Option<String>,
    pub is_vpn: Option<bool>,
    pub is_tor: Option<bool>,
    pub is_data_center: Option<bool>,
    pub is_public_proxy: Option<bool>,
    pub is_web_proxy: Option<bool>,
    pub is_web_crawler: Option<bool>,
    pub is_residential_proxy: Option<bool>,
    pub is_spammer: Option<bool>,
    pub is_scanner: Option<bool>,
    pub is_botnet: Option<bool>,
    pub is_bogon: Option<bool>,
    pub is_consumer_privacy_network: Option<bool>,
    pub is_enterprise_private_network: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct WhoisResult {
    pub domain: Option<String>,
    pub domain_id: Option<String>,
    pub status: Option<String>,
    pub create_date: Option<String>,
    pub update_date: Option<String>,
    pub expire_date: Option<String>,
    pub whois_server: Option<String>,
    pub domain_age: Option<u64>,

    // Complex nested fields as JSON values
    pub registrar: Option<Value>,
    pub registrant: Option<Value>,
    pub admin: Option<Value>,
    pub tech: Option<Value>,
    pub billing: Option<Value>,
    pub nameservers: Option<Value>,

    // #[serde(flatten)]
    // pub extra: Value,
}

#[derive(Debug, Deserialize)]
pub struct HostedDomainsResult {
    pub ip: Option<String>,
    pub total_domains: Option<i32>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub total_pages: Option<i32>,
    pub domains: Vec<String>,
}