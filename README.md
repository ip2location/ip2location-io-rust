# IP2Location.io Rust SDK — IP Geolocation, Domain WHOIS & Hosted Domain Lookup

[![Crates.io](https://img.shields.io/crates/v/ip2locationio.svg)](https://crates.io/crates/ip2locationio)
[![Crates.io downloads](https://img.shields.io/crates/d/ip2locationio.svg)](https://crates.io/crates/ip2locationio)

**IP2Location.io Rust SDK** (`ip2locationio`) is a Rust crate for retrieving **IPv4 and IPv6 geolocation, network, and IP intelligence data** through the [IP2Location.io API](https://www.ip2location.io/). It also supports **domain WHOIS lookup** and **hosted-domain lookup**. Its asynchronous API can be used in Rust applications with Tokio without writing the HTTP requests yourself.

**Quick links:** [Get an API key](https://www.ip2location.io/pricing) · [Developer documentation](https://ip2location-io-rust.readthedocs.io/en/latest/index.html) · [Rust API reference](https://ip2location-io-rust.readthedocs.io/en/latest/code.html) · [Crates.io package](https://crates.io/crates/ip2locationio)

## Features

- **IPv4 and IPv6 geolocation:** Retrieve available country, region, district, city, ZIP/postal code, latitude, and longitude.
- **Network and ASN information:** Look up ASN, AS name, ISP, associated domain, network speed, and connection/usage information where available.
- **Time zone and localization data:** Retrieve time zone information and request translated continent, country, region, and city names on eligible plans.
- **Proxy and security signals:** Access available proxy, VPN, Tor, data center, crawler, and other threat-related indicators, including fraud scores where supported.
- **Additional IP intelligence:** Retrieve eligible mobile network (MCC/MNC), country metadata, weather/elevation, and advertising category fields.
- **Domain WHOIS lookup:** Obtain available registrar, registration dates, domain status, nameservers, and registrant information.
- **Internationalized domain and URL utilities:** Convert Unicode domains to/from Punycode and extract domain names or domain extensions.
- **Hosted-domain lookup:** Retrieve domains hosted on an IP address, with optional pagination.
- **Asynchronous Rust interface:** Use `async`/`await` with the crate's `Client` and Tokio runtime.

> **Plan availability:** Response fields, translations, and security indicators depend on your IP2Location.io plan and on the queried IP address. Refer to the [pricing page](https://www.ip2location.io/pricing) and [Rust API field reference](https://ip2location-io-rust.readthedocs.io/en/latest/code.html).

## SDK components

The [Rust SDK API reference](https://ip2location-io-rust.readthedocs.io/en/latest/code.html) describes the following interfaces:

| Component | Description |
| --- | --- |
| [`Client::new(api_key)`](https://ip2location-io-rust.readthedocs.io/en/latest/code.html) | Creates a client configured with an API key. |
| [`client.ip_geolocation().lookup(ip, language)`](https://ip2location-io-rust.readthedocs.io/en/latest/code.html) | Retrieves IP geolocation, network, and available security information. The language argument is `None` or `Some("es")`, for example. |
| [`client.whois().lookup(domain)`](https://ip2location-io-rust.readthedocs.io/en/latest/code.html) | Retrieves domain WHOIS information. |
| [`ip2locationio::whois` utilities](https://ip2location-io-rust.readthedocs.io/en/latest/code.html) | Provides `get_punycode()`, `get_normaltext()`, `get_domain_name()`, and `get_domain_extension()` in the official quickstart. |
| [`client.hosted_domains().lookup(ip, page)`](https://ip2location-io-rust.readthedocs.io/en/latest/code.html) | Retrieves hosted domains with optional page selection. |

## API access and query limits

IP2Location.io offers keyless REST API access for limited IP geolocation queries and a Free plan with an API key:

| API service | Keyless (no API key) | Free plan (with API key) |
| --- | --- | --- |
| IP Geolocation API | **1,000 queries/day** | **50,000 queries/month** |
| Domain WHOIS API | — | **500 queries/month** |
| Hosted Domain API | — | **50 queries/month** |

- **Keyless access:** Limited keyless IP geolocation requests are available through the [REST API](https://www.ip2location.io/ip2location-documentation), with a daily reset at **00:00 UTC**. The Rust SDK examples below use an API key; keyless SDK configuration is not demonstrated in the Rust quickstart.
- **Free plan:** Obtain an API key to use the published monthly allowances above.
- **Attribution:** Keyless and Free-plan access require attribution. See the [pricing page](https://www.ip2location.io/pricing) for the applicable conditions.
- **Paid plans:** See the [pricing page](https://www.ip2location.io/pricing) for current quotas, available attributes, and prices.

For authentication and rate-limit details, see the [IP Geolocation API documentation](https://www.ip2location.io/ip2location-documentation).

## Requirements

- **Rust 1.86.0 or later**, with support for Rust edition **2024**, as declared in the repository's current [`Cargo.toml`](https://github.com/ip2location/ip2location-io-rust/blob/main/Cargo.toml).
- [Cargo](https://doc.rust-lang.org/cargo/) to manage dependencies and run Rust applications.
- A **Tokio runtime** for the asynchronous example programs (`#[tokio::main]`).
- An [IP2Location.io API key](https://www.ip2location.io/pricing) for SDK requests.
- Internet access to the API endpoints.

## Installation

### 1. Create a Rust project

```bash
cargo new ip2location-example
cd ip2location-example
```

### 2. Add the SDK and Tokio

Add the following to the `[dependencies]` section of `Cargo.toml`:

```toml
[dependencies]
ip2locationio = "1.1.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The SDK is also listed on [crates.io](https://crates.io/crates/ip2locationio). Cargo downloads the required dependencies when you build or run the project. If your project already has a `[dependencies]` section, add the two entries to that existing section rather than creating a duplicate.

### 3. Set your API key

Set an environment variable rather than hard-coding your credential:

**Linux / macOS terminal:**

```bash
export IP2LOCATION_API_KEY='YOUR_API_KEY'
```

**Windows Command Prompt:**

```bat
set IP2LOCATION_API_KEY=YOUR_API_KEY
```

**Windows PowerShell:**

```powershell
$env:IP2LOCATION_API_KEY = 'YOUR_API_KEY'
```

The examples below obtain the key from `IP2LOCATION_API_KEY` and pass it to `Client::new()`. Using an environment variable is a convenient application-level adaptation of the official quickstart, which passes `"YOUR_API_KEY"` directly.

## Quickstart: IP geolocation lookup

Replace the contents of `src/main.rs` with:

```rust
use ip2locationio::{Client, errors::Ip2LocationError};

#[tokio::main]
async fn main() -> Result<(), Ip2LocationError> {
    let api_key = std::env::var("IP2LOCATION_API_KEY")
        .expect("Set the IP2LOCATION_API_KEY environment variable");

    let client = Client::new(Some(api_key));
    let ip_info = client.ip_geolocation().lookup("8.8.8.8", None).await?;

    println!("Lookup Result:\n{:#?}", ip_info);
    Ok(())
}
```

Run it from the project directory:

```bash
cargo run
```

The SDK's `lookup()` method is asynchronous and returns a structured result. The `?` operator propagates SDK errors, and `{:#?}` prints the returned value for inspection. This follows the official [Rust quickstart](https://ip2location-io-rust.readthedocs.io/en/latest/quickstart.html), with an environment-variable API key in place of a literal credential.

The fields returned depend on your plan and queried IP. See the [Rust geolocation field reference](https://ip2location-io-rust.readthedocs.io/en/latest/code.html).

## More usage examples

The following snippets show the API calls documented by the Rust SDK. **Each network lookup snippet belongs inside an `async` function** where you have created a `client` as in the quickstart. Use the same `Ip2LocationError` return type and `await?` pattern shown above.

### Request translated location names

The optional language argument accepts `Some("es")`, for example, to request Spanish location translations. Translation support is available on **Plus and Security** plans and applies to eligible continent, country, region, and city names.

```rust
let ip_info = client.ip_geolocation().lookup("8.8.8.8", Some("es")).await?;
println!("Translated lookup:\n{:#?}", ip_info);
```

Pass `None` instead when translations are not needed.

### Inspect available network and ASN information

The geolocation response can include ISP, ASN, AS name, and related network attributes, depending on your plan. The documented response fields include `asn`, `as_name`, `isp`, and nested `as_info` details.

```rust
let ip_info = client.ip_geolocation().lookup("8.8.8.8", None).await?;
println!("IP and network information:\n{:#?}", ip_info);
```

This additional example prints the complete structured response rather than assuming optional network attributes are always present.

### Inspect available proxy indicators

Available security-related fields can include `is_proxy`, `fraud_score`, and nested `proxy` details such as VPN and Tor indicators. These values are plan-dependent and may be absent.

```rust
let ip_info = client.ip_geolocation().lookup("8.8.8.8", None).await?;
println!("Available geolocation and proxy data:\n{:#?}", ip_info);
```

Inspect the returned structure before using optional indicators in production. Treat IP-based security signals as supporting information, not sole proof of malicious activity.

### Look up domain WHOIS information

The official Rust quickstart demonstrates WHOIS lookup through `client.whois().lookup()`:

```rust
let whois_info = client.whois().lookup("google.com").await?;
println!("WHOIS Lookup Result:\n{:#?}", whois_info);
```

The returned WHOIS record can include registrar details, creation and expiration dates, nameservers, and available contact information. Some registrant data may be missing or redacted.

### Use domain and Punycode utilities

The official quickstart provides these synchronous functions under `ip2locationio::whois`. They do not require an API request.

**Convert a Unicode domain name to Punycode:**

```rust
let domain = ip2locationio::whois::get_punycode("täst.de").unwrap();
println!("Punycode: {}", domain);
```

**Convert Punycode back to Unicode:**

```rust
let domain = ip2locationio::whois::get_normaltext("xn--tst-qla.de").unwrap();
println!("Unicode domain: {}", domain);
```

**Extract a domain name from a URL:**

```rust
let domain = ip2locationio::whois::get_domain_name("https://www.example.com/exe").unwrap();
println!("Domain: {}", domain);
```

**Get a domain extension:**

```rust
let extension = ip2locationio::whois::get_domain_extension("example.com").unwrap();
println!("Extension: {}", extension);
```

These names follow the **official Rust quickstart** (with underscores in `get_domain_name` and `get_domain_extension`). The `unwrap()` calls follow the published utility examples; for untrusted input in production, handle possible errors as appropriate for your application.

### Find domains hosted on an IP address

The Rust SDK's `hosted_domains()` service supports an optional page argument. Pass `None` for the first page:

```rust
let hosted_domains_info = client.hosted_domains().lookup("8.8.8.8", None).await?;
println!("Hosted Domains Result:\n{:#?}", hosted_domains_info);
```

To request the second page, provide a page number:

```rust
let hosted_domains_info = client.hosted_domains().lookup("8.8.8.8", Some(2)).await?;
println!("Hosted Domains Page 2:\n{:#?}", hosted_domains_info);
```

The API reference documents pagination fields including `total_domains`, `page`, `per_page`, `total_pages`, and `domains`. Check the returned pagination values before requesting additional pages.

## Rust-specific notes and troubleshooting

- **Async runtime:** The published examples use `#[tokio::main]`. Ensure Tokio is a direct dependency with the `macros` and `rt-multi-thread` features enabled.
- **Compiler version:** If the build reports an unsupported edition or Rust version, check `rustc --version` and update the toolchain using [rustup](https://rustup.rs/). The current repository declares Rust 1.86.0 and edition 2024.
- **API key:** If the program reports that `IP2LOCATION_API_KEY` is missing, set it in the same terminal session used for `cargo run` or configure it in your deployment environment.
- **Plan-dependent fields:** Some geolocation attributes and proxy indicators are not available on all plans. Do not assume all optional fields have values.
- **Documentation labels:** The current Rust Read the Docs page has outdated Python-oriented headings (for example, "PyPI Installation" and `pytest`). **Use Cargo** for Rust installation/builds, rather than Python packaging tools.

## Common use cases

- **Website and service localization:** Use approximate IP geolocation to choose regional content or suggest a language.
- **Traffic analytics:** Enrich application logs or events with available country, location, ASN, and ISP data.
- **Security review:** Supplement other risk controls using available proxy, VPN, Tor, crawler, and network indicators.
- **Domain research:** Inspect WHOIS information, domain registration dates, registrar details, and nameservers.
- **Network and hosting research:** Explore autonomous system information and lists of domains hosted on a given IP address.

## Documentation and support

- [Rust SDK quickstart and sample codes](https://ip2location-io-rust.readthedocs.io/en/latest/quickstart.html)
- [Rust API reference and response fields](https://ip2location-io-rust.readthedocs.io/en/latest/code.html)
- [Official GitHub repository](https://github.com/ip2location/ip2location-io-rust)
- [Rust crate on crates.io](https://crates.io/crates/ip2locationio)
- [Current Cargo manifest](https://github.com/ip2location/ip2location-io-rust/blob/main/Cargo.toml)
- [IP2Location.io API plans and query limits](https://www.ip2location.io/pricing)
- [IP Geolocation API authentication and rate limits](https://www.ip2location.io/ip2location-documentation)
- [Report an SDK issue or request a feature](https://github.com/ip2location/ip2location-io-rust/issues)
- [API and account support](https://www.ip2location.io/contact-us)

## License

Licensed under the **MIT License**. See [LICENSE.TXT](./LICENSE.TXT).
