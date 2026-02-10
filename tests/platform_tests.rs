use syspeek::utils::platform;

#[test]
fn test_format_bytes_zero() {
    assert_eq!(platform::format_bytes(0), "0.0 B");
}

#[test]
fn test_format_bytes_bytes() {
    assert_eq!(platform::format_bytes(500), "500.0 B");
}

#[test]
fn test_format_bytes_kilobytes() {
    assert_eq!(platform::format_bytes(1024), "1.0 KB");
}

#[test]
fn test_format_bytes_megabytes() {
    assert_eq!(platform::format_bytes(1024 * 1024), "1.0 MB");
}

#[test]
fn test_format_bytes_gigabytes() {
    assert_eq!(platform::format_bytes(1024 * 1024 * 1024), "1.0 GB");
}

#[test]
fn test_format_bytes_terabytes() {
    assert_eq!(platform::format_bytes(1024 * 1024 * 1024 * 1024), "1.0 TB");
}

#[test]
fn test_format_uptime_minutes_only() {
    assert_eq!(platform::format_uptime(300), "5 minutes");
}

#[test]
fn test_format_uptime_hours_and_minutes() {
    assert_eq!(platform::format_uptime(3660), "1 hours, 1 minutes");
}

#[test]
fn test_format_uptime_days_hours_minutes() {
    assert_eq!(platform::format_uptime(90060), "1 days, 1 hours, 1 minutes");
}

#[test]
fn test_format_uptime_zero() {
    assert_eq!(platform::format_uptime(0), "0 minutes");
}
