use idna;
use url::Url;

use crate::common::{BASE_WHOIS_URL, Config, execute_query};
use crate::errors::Ip2LocationError;
use crate::models::WhoisResult;

/// A handler for the WHOIS Lookup API.
pub struct Whois<'a> {
    pub(crate) config: &'a Config,
}

impl<'a> Whois<'a> {
    /// Looks up WHOIS information for a given domain.
    pub async fn lookup(&self, domain: &str) -> Result<WhoisResult, Ip2LocationError> {
        if domain.trim().is_empty() {
            return Err(Ip2LocationError::InvalidInput("Domain cannot be empty.".into()));
        }

        let params = vec![("domain", domain)];
        execute_query(BASE_WHOIS_URL, self.config, &params).await
    }
}

//
// Standalone Domain Utility Functions
//

/// Converts a Unicode domain to its Punycode representation (IDNA ASCII).
///
/// Example: `München.de` -> `xn--mnchen-3ya.de`
pub fn get_punycode(domain: &str) -> Result<String, String> {
    idna::domain_to_ascii(domain).map_err(|e| format!("IDNA encoding error: {:?}", e))
}

/// Converts a Punycode domain back to its Unicode representation.
///
/// Example: `xn--mnchen-3ya.de` -> `München.de`
pub fn get_normaltext(domain: &str) -> Result<String, String> {
    let (unicode, res) = idna::domain_to_unicode(domain);
    match res {
        Ok(()) => Ok(unicode),
        Err(e) => Err(format!("IDNA decoding error: {:?}", e)),
    }
}

/// Extracts the hostname (domain) from a full URL string.
///
/// This function automatically adds a scheme if missing and strips a leading "www.".
pub fn get_domain_name(url_str: &str) -> Option<String> {
    if url_str.is_empty() {
        return None;
    }

    let candidate = if url_str.starts_with("http") {
        url_str.to_owned()
    } else {
        format!("https://{}", url_str)
    };

    if let Ok(parsed_url) = Url::parse(&candidate) {
        parsed_url.host_str().map(|host| {
            host.strip_prefix("www.").unwrap_or(host).to_string()
        })
    } else {
        None
    }
}

/// Extracts the domain extension (TLD) from a URL or domain string.
///
/// Example: "https://www.example.co.uk" -> Some(".co.uk")
pub fn get_domain_extension(url_str: &str) -> Option<String> {
    get_domain_name(url_str).and_then(|domain| domain.find('.').map(|i| domain[i..].to_string()))
}