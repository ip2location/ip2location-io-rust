# Quickstart

## Dependencies

This module requires API key to function. You may sign up for a free API key at <https://www.ip2location.io/pricing>.

## Installation

### PyPI Installation

Just add `ip2locationio = "1.0.0"` into your *Cargo.toml*.


## Sample Codes

### Lookup IP Address Geolocation Data

You can make a geolocation data lookup for an IP address as below:

``` Rust
use ip2locationio::{Client, errors::Ip2LocationError};

#[tokio::main]
async fn main() -> Result<(), Ip2LocationError> {
    // Create a single client instance
    let client = Client::new(Some("YOUR_API_KEY".to_string()));

    // Use the IP Geolocation API
    let ip_info = client.ip_geolocation().lookup("8.8.8.8", Some("es")).await?; // Or specify None if you don't need to use the lang parameter.
    println!("Lookup Result:\n{:#?}", ip_info);

    Ok(())
}
```

### Lookup Domain Information

You can lookup domain information as below:

``` Rust
use ip2locationio::{Client, errors::Ip2LocationError};

#[tokio::main]
async fn main() -> Result<(), Ip2LocationError> {
    // Create a single client instance
    let client = Client::new(Some("YOUR_API_KEY".to_string()));

    // Use the WHOIS API
    let whois_info = client.whois().lookup("google.com").await?;
    println!("WHOIS Lookup Result:\n{:#?}", whois_info);

    Ok(())
}
```

### Convert Normal Text to Punycode

You can convert an international domain name to Punycode as below:

``` Rust
use ip2locationio::{Client, errors::Ip2LocationError};

#[tokio::main]
async fn main() -> Result<(), Ip2LocationError> {
    let converted_domain = ip2locationio::whois::get_punycode("täst.de").unwrap();
    println!("Converted Domain: {}", converted_domain);

    Ok(())
}
```

### Convert Punycode to Normal Text

You can convert a Punycode to international domain name as below:

```Rust
use ip2locationio::{Client, errors::Ip2LocationError};

#[tokio::main]
async fn main() -> Result<(), Ip2LocationError> {
    let converted_domain = ip2locationio::whois::get_normaltext("xn--tst-qla.de").unwrap();
    println!("Converted Domain: {}", converted_domain);

    Ok(())
}
```

### Get Domain Name

You can extract the domain name from an url as below:

```Rust
use ip2locationio::{Client, errors::Ip2LocationError};

#[tokio::main]
async fn main() -> Result<(), Ip2LocationError> {
    let domain = ip2locationio::whois::get_domain_name("https://www.example.com/exe").unwrap();
    println!("Extracted Domain: {}", domain);

    Ok(())
}
```

### Get Domain Extension

You can extract the domain extension from a domain name or url as below:

```Rust
use ip2locationio::{Client, errors::Ip2LocationError};

#[tokio::main]
async fn main() -> Result<(), Ip2LocationError> {
    let tld = ip2locationio::whois::get_domain_extension("example.com").unwrap();
    println!("Extracted tld: {}", tld);

    Ok(())
}
```

### Get Hosted Domain List

You can get the domains hosted within the IP using following codes:

``` Rust
use ip2locationio::{Client, errors::Ip2LocationError};

#[tokio::main]
async fn main() -> Result<(), Ip2LocationError> {
    // Create a single client instance
    let client = Client::new(Some("YOUR_API_KEY".to_string()));

    // Use the Hosted Domains API	
    let hosted_domains_info = client.hosted_domains().lookup("8.8.8.8", None).await?;
    println!("Hosted Domains Result:\n{:#?}", hosted_domains_info);

    Ok(())
}
```

## Testing

For testing, you can use `pytest` with the following steps:

1. Setup your IP2Location.io API Key by export to the environment variable, or set it as a command-line argument when running the `pytest` later. To export the API Key to the environment variable, run `export IP2LOCATION_API_KEY=YOUR_API_KEY`. Be sure to change the 'YOUR_API_KEY' to your corresponding API Key.
2. Run the `pytest` after you had clone our repository to your local. If you prefer to set your API key as the argument to the `pytest` command, you can run it as `pytest --apikey YOUR_API_KEY`, where you need to substitute the YOUR_API_KEY to your corresponding API Key.