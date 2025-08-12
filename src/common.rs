use reqwest::Client as HttpClient;

pub const BASE_IP_URL: &str = "https://api.ip2location.io/";
pub const BASE_WHOIS_URL: &str = "https://api.ip2whois.com/v2";
pub const BASE_HOSTED_DOMAINS_URL: &str = "https://domains.ip2whois.com/domains";

#[derive(Clone)]
pub struct Config {
    pub api_key: Option<String>,
    pub http: HttpClient,
}

impl Config {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            api_key,
            http: HttpClient::new(),
        }
    }
}
