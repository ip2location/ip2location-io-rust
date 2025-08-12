use idna;
use url::Url;

use crate::common::{BASE_WHOIS_URL, Config};
use crate::errors::Ip2LocationError;
use crate::models::WhoisResult;

pub struct Whois {
    config: Config,
}

impl Whois {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            config: Config::new(api_key),
        }
    }

    pub async fn lookup_domain(&self, domain: &str) -> Result<WhoisResult, Ip2LocationError> {
        if domain.trim().is_empty() {
            return Err(Ip2LocationError::InvalidInput("Domain is empty".into()));
        }

        let mut url = Url::parse(BASE_WHOIS_URL)?;
        {
            let mut qp = url.query_pairs_mut();
            qp.append_pair("domain", domain);
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

	/// Convert Unicode domain -> Punycode (IDNA ASCII)
	pub fn get_punycode(domain: &str) -> Result<String, String> {
		idna::domain_to_ascii(domain).map_err(|e| format!("IDNA encode error: {:?}", e))
	}

	/// Convert Punycode (IDNA ASCII) -> Unicode
	/// Returns Err(...) if IDNA reported errors (to mirror Python raising on decode problems).
	pub fn get_normaltext(domain: &str) -> Result<String, String> {
		let (unicode, res) = idna::domain_to_unicode(domain);
		match res {
			Ok(()) => Ok(unicode),
			Err(e) => Err(format!("IDNA decode error: {:?}", e)),
		}
	}

	/// Extract the hostname (domain) from a URL string.
	/// - If input is empty -> None
	/// - If scheme missing, we prepend "https://"
	/// - Strips leading "www." (same behavior as your Python slice)
	pub fn get_domainname(url: &str) -> Option<String> {
		if url.is_empty() {
			return None;
		}

		let candidate = if url.starts_with("http") {
			url.to_owned()
		} else {
			format!("https://{}", url)
		};

		match Url::parse(&candidate) {
			Ok(parsed) => {
				if let Some(host) = parsed.host_str() {
					// remove leading "www." if present
					if host.starts_with("www.") {
						Some(host[4..].to_string())
					} else {
						Some(host.to_string())
					}
				} else {
					None
				}
			}
			Err(_) => None,
		}
	}

	/// Return the domain extension (including the leading '.') for a URL,
	/// e.g. "https://www.example.com" -> Some(".com")
	pub fn get_domain_extension(url: &str) -> Option<String> {
		Self::get_domainname(url).and_then(|domain| domain.find('.').map(|i| domain[i..].to_string()))
	}
}
