use syspeek::system::disk;

#[test]
fn test_disk_info_returns_disks() {
    let disks = disk::get_disk_info();

    assert!(!disks.is_empty(), "Should have at least one disk");

    for d in &disks {
        assert!(d.total > 0, "Disk total should be greater than 0");
        assert!(d.used <= d.total, "Used should not exceed total");
        assert!(d.usage_percent >= 0.0 && d.usage_percent <= 100.0);
        assert!(!d.mount.is_empty(), "Mount point should not be empty");
    }
}
