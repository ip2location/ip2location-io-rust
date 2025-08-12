use crate::common::{BASE_HOSTED_DOMAINS_URL, Config, execute_query};
use crate::errors::Ip2LocationError;
use crate::models::HostedDomainsResult;

/// A handler for the Hosted Domains API.
pub struct HostedDomains<'a> {
    pub(crate) config: &'a Config,
}

impl<'a> HostedDomains<'a> {
    /// Looks up domains hosted on the same IP address.
    pub async fn lookup(&self, ip: &str, page: Option<String>) -> Result<HostedDomainsResult, Ip2LocationError> {
        if ip.trim().is_empty() {
            return Err(Ip2LocationError::InvalidInput("IP address cannot be empty.".into()));
        }

        let mut params = vec![("ip", ip)];
        if let Some(p) = &page {
            if !p.is_empty() {
                params.push(("page", p));
            }
        }
        
        execute_query(BASE_HOSTED_DOMAINS_URL, self.config, &params).await
    }
}