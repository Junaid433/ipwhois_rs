use ipwhois_rs::lookup::IpWhoIs;

#[tokio::test]
async fn test_valid_ip_lookup() {
    let mut client = IpWhoIs::new();
    let ip = "8.8.4.4";
    let info = client.lookup(ip).await.expect("Lookup failed");

    assert_eq!(info.ip, ip);
    assert!(info.success);
    assert!(!info.country.is_empty());
    assert!(!info.city.is_empty());
}

#[tokio::test]
async fn test_valid_ipv6_lookup() {
    let mut client = IpWhoIs::new();
    let ip = "2001:4860:4860::8888";
    let info = client.lookup(ip).await.expect("IPv6 lookup failed");

    assert_eq!(info.ip, ip);
    assert!(info.success);
}

#[tokio::test]
async fn test_invalid_ip_returns_error() {
    let mut client = IpWhoIs::new();
    let ip = "999.999.999.999";
    let result = client.lookup(ip).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_flag_fields_not_empty() {
    let mut client = IpWhoIs::new();
    let ip = "8.8.8.8";
    let info = client.lookup(ip).await.expect("Lookup failed");

    assert!(!info.flag.img.is_empty());
    assert!(!info.flag.emoji.is_empty());
    assert!(!info.flag.emoji_unicode.is_empty());
}

#[tokio::test]
async fn test_connection_fields_are_valid() {
    let mut client = IpWhoIs::new();
    let ip = "8.8.8.8";
    let info = client.lookup(ip).await.expect("Lookup failed");

    assert!(info.connection.asn > 0);
    assert!(!info.connection.org.is_empty());
    assert!(!info.connection.isp.is_empty());
    assert!(!info.connection.domain.is_empty());
}

#[tokio::test]
async fn test_timezone_fields_are_valid() {
    let mut client = IpWhoIs::new();
    let ip = "8.8.8.8";
    let info = client.lookup(ip).await.expect("Lookup failed");

    assert!(!info.timezone.id.is_empty());
    assert!(!info.timezone.abbr.is_empty());
    assert!(info.timezone.offset != 0);
    assert!(!info.timezone.utc.is_empty());
    assert!(!info.timezone.current_time.is_empty());
}

#[tokio::test]
async fn test_success_flag_true_for_valid_ip() {
    let mut client = IpWhoIs::new();
    let ip = "1.1.1.1";
    let info = client.lookup(ip).await.expect("Lookup failed");
    assert!(info.success);
}

#[tokio::test]
async fn test_latitude_and_longitude_are_not_zero() {
    let mut client = IpWhoIs::new();
    let ip = "1.1.1.1";
    let info = client.lookup(ip).await.expect("Lookup failed");

    assert!(info.latitude.abs() > 0.0);
    assert!(info.longitude.abs() > 0.0);
}