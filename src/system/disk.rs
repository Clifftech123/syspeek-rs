use sysinfo::Disks;

// Disk Information Structure
pub struct DiskInfo {
    pub name: String,
    pub mount: String,
    pub total: u64,
    pub used: u64,
    pub usage_percent: f64,
}


// Retrieves disk information from the system
pub fn get_disk_info() -> Vec<DiskInfo> {
    let disks = Disks::new_with_refreshed_list();
    let mut result = Vec::new();

    for disk in disks.list() {
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total - available;
        let usage_percent = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        result.push(DiskInfo {
            name: disk.name().to_string_lossy().to_string(),
            mount: disk.mount_point().to_string_lossy().to_string(),
            total,
            used,
            usage_percent,
        });
    }

    result
}