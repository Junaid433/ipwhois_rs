# ipwhois_rs

![License](https://img.shields.io/crates/l/ipwhois_rs)
![Crates.io](https://img.shields.io/crates/v/ipwhois_rs)
![Build](https://img.shields.io/github/actions/workflow/status/Junaid433/ipwhois_rs/ci.yml?branch=master)

A Rust wrapper library for IP whois and geolocation lookup using ipwhois.io API.

---

## Features

- Lookup IP address information including location, ISP, timezone, and flags.
- Async API using Tokio runtime.
- Detailed response struct with nested data.

---

## Response Data Structure

The `IpWhoIsResponse` struct contains detailed information returned by the API.

### IpWhoIsResponse Fields

- **ip** (`String`): Queried IP address.
- **success** (`bool`): Whether the lookup was successful.
- **ip_type** (`String`): IP type, e.g., "IPv4" or "IPv6".
- **continent** (`String`): Continent name (e.g., "North America").
- **continent_code** (`String`): Continent code (e.g., "NA").
- **country** (`String`): Country name (e.g., "United States").
- **country_code** (`String`): Country code (e.g., "US").
- **region** (`String`): Region or state name (e.g., "California").
- **region_code** (`String`): Region code (e.g., "CA").
- **city** (`String`): City name (e.g., "Mountain View").
- **latitude** (`f64`): Latitude coordinate.
- **longitude** (`f64`): Longitude coordinate.
- **is_eu** (`bool`): Whether the IP belongs to the European Union.
- **postal** (`String`): Postal or ZIP code.
- **calling_code** (`String`): International telephone calling code.
- **capital** (`String`): Capital city of the country.
- **borders** (`String`): Bordering countries or regions.
- **flag** (`Flag`): Country flag information.
- **connection** (`Connection`): ISP and ASN info.
- **timezone** (`TimeZone`): Timezone info.

---

### Nested Structs

#### Flag

- **img** (`String`): URL to the country flag image.
- **emoji** (`String`): Country flag emoji.
- **emoji_unicode** (`String`): Unicode points for the emoji flag.

#### Connection

- **asn** (`i32`): Autonomous System Number.
- **org** (`String`): Organization (ISP or company).
- **isp** (`String`): Internet Service Provider name.
- **domain** (`String`): Domain associated with the connection.

#### TimeZone

- **id** (`String`): Timezone identifier (e.g., "America/Los_Angeles").
- **abbr** (`String`): Timezone abbreviation (e.g., "PDT").
- **is_dst** (`bool`): Whether daylight saving time is active.
- **offset** (`i32`): Offset in seconds from UTC (e.g., -25200).
- **utc** (`String`): UTC offset as a string (e.g., "-07:00").
- **current_time** (`String`): Current local time in the timezone.

---

## Example Usage

```rust
use ipwhois_rs::IpWhoIs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = IpWhoIs::new();
    let info = client.lookup("8.8.4.4").await?;

    println!("IP: {}", info.ip);
    println!("Country: {} ({})", info.country, info.country_code);
    println!("City: {}", info.city);
    println!("Latitude: {}", info.latitude);
    println!("Longitude: {}", info.longitude);
    println!("Flag Emoji: {}", info.flag.emoji);
    println!("ISP: {}", info.connection.isp);
    println!("Timezone: {}", info.timezone.id);

    Ok(())
}

````

---

## License

MIT © 2025 Junaid Rahman (https://github.com/junaid433)

```

