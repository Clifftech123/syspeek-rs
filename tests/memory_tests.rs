use sysinfo::System;
use syspeek::system::memory;

#[test]
fn test_memory_info_returns_valid_data() {
    let mut sys = System::new_all();
    sys.refresh_all();
    let info = memory::get_memory_info(&sys);

    assert!(info.total > 0, "Total memory should be greater than 0");
    assert!(info.used <= info.total, "Used memory should not exceed total");
    assert_eq!(info.available, info.total - info.used);
    assert!(info.usage_percent >= 0.0 && info.usage_percent <= 100.0);
}
