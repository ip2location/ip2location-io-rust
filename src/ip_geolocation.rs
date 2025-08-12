use crate::common::{BASE_IP_URL, Config, execute_query};
use crate::errors::Ip2LocationError;
use crate::models::IpGeolocationResult;

/// A handler for the IP Geolocation API.
pub struct IpGeolocation<'a> {
    pub(crate) config: &'a Config,
}

impl<'a> IpGeolocation<'a> {
    /// Looks up geolocation data for a given IP address.
    pub async fn lookup(&self, ip: &str, lang: Option<&str>) -> Result<IpGeolocationResult, Ip2LocationError> {
        if ip.trim().is_empty() {
            return Err(Ip2LocationError::InvalidInput("IP address cannot be empty.".into()));
        }

        let mut params = vec![("ip", ip)];
        if let Some(l) = &lang {
            if !l.is_empty() {
            params.push(("lang", l));
            }
        }

        execute_query(BASE_IP_URL, self.config, &params).await
    }
}