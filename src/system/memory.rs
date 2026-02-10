use sysinfo::System;

// Memory Information Structure
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f64,
}


// Retrieves memory information from the system
pub fn get_memory_info(system: &System) -> MemoryInfo {
    let total = system.total_memory();
    let used = system.used_memory();
    let available = total - used;
    let usage_percent = if total > 0 {
        (used as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    MemoryInfo {
        total,
        used,
        available,
        usage_percent,
    }
}