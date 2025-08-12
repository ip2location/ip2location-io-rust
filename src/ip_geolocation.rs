use url::Url;

use crate::common::{BASE_IP_URL, Config};
use crate::errors::Ip2LocationError;
use crate::models::IpGeolocationResult;

pub struct IpGeolocation {
    config: Config,
}

impl IpGeolocation {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            config: Config::new(api_key),
        }
    }

    pub async fn lookup_ip(&self, ip: &str, lang: Option<&str>) -> Result<IpGeolocationResult, Ip2LocationError> {
        if ip.trim().is_empty() {
            return Err(Ip2LocationError::InvalidInput("IP is empty".into()));
        }

        let mut url = Url::parse(BASE_IP_URL)?;
        {
            let mut qp = url.query_pairs_mut();
            qp.append_pair("ip", ip);
            if let Some(l) = lang {
                qp.append_pair("lang", l);
            }
            // append key only if we have one and it's non-empty
            if let Some(k) = &self.config.api_key {
                if !k.is_empty() {
                    qp.append_pair("key", k);
                }
            }
        } // drop query_pairs_mut borrow

        let resp = self.config.http.get(url).send().await?;
        let status = resp.status();
        let body = resp.text().await?;

        if !status.is_success() {
            return Err(Ip2LocationError::ApiError { status: status.as_u16(), message: body });
        }

        Ok(serde_json::from_str(&body)?)
    }
}
