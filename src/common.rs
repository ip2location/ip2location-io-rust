use reqwest::Client as HttpClient;
use serde::de::DeserializeOwned;
use url::Url;
use crate::errors::Ip2LocationError;

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

/// A generic function to execute API queries, handling URL building, request sending,
/// and response parsing.
pub(crate) async fn execute_query<T: DeserializeOwned>(
    base_url: &str,
    config: &Config,
    params: &[(&str, &str)],
) -> Result<T, Ip2LocationError> {
    let mut url = Url::parse(base_url)?;

    {
        let mut qp = url.query_pairs_mut();
        for (key, value) in params {
            qp.append_pair(key, value);
        }

        if let Some(k) = &config.api_key {
            if !k.is_empty() {
                qp.append_pair("key", k);
            }
        }
    }

    let resp = config.http.get(url).send().await?;
    let status = resp.status();
    let body = resp.text().await?;

    if !status.is_success() {
        return Err(Ip2LocationError::ApiError {
            status: status.as_u16(),
            message: body,
        });
    }

    serde_json::from_str(&body).map_err(Ip2LocationError::Json)
}