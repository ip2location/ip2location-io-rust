pub mod errors;
pub mod common;
pub mod models;
pub mod ip_geolocation;
pub mod whois;
pub mod hosteddomains;

use crate::common::Config;
pub use ip_geolocation::IpGeolocation;
pub use whois::Whois;
pub use hosteddomains::HostedDomains;

/// The main, unified client for accessing all IP2Location.io APIs.
pub struct Client {
    config: Config,
}

impl Client {
    /// Creates a new client with an optional API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - An optional string slice that holds the IP2Location.io API key.
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            config: Config::new(api_key),
        }
    }

    /// Returns a handler for the IP Geolocation API.
    pub fn ip_geolocation(&self) -> IpGeolocation<'_> {
        IpGeolocation { config: &self.config }
    }

    /// Returns a handler for the WHOIS Lookup API.
    pub fn whois(&self) -> Whois<'_> {
        Whois { config: &self.config }
    }

    /// Returns a handler for the Hosted Domains API.
    pub fn hosted_domains(&self) -> HostedDomains<'_> {
        HostedDomains { config: &self.config }
    }
}