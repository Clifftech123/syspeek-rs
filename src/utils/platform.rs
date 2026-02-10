

// Retrieves the current network data (bytes sent and received) for all interfaces
const BYTE_UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];


// Formats a byte value into a human-readable string with appropriate units (e.g., KB, MB, GB).
pub fn format_bytes(bytes: u64) -> String {
    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < BYTE_UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    format!("{:.1} {}", size, BYTE_UNITS[unit])
}


// Formats a duration in seconds into a human-readable string (e.g., "2 days, 3 hours, 15 minutes").
pub fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    if days > 0 {
        format!("{} days, {} hours, {} minutes", days, hours, minutes)
    } else if hours > 0 {
        format!("{} hours, {} minutes", hours, minutes)
    } else {
        format!("{} minutes", minutes)
    }
}



