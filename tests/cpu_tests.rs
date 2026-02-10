use sysinfo::System;
use syspeek::system::cpu;

#[test]
fn test_cpu_info_returns_valid_data() {
    let mut sys = System::new_all();
    sys.refresh_all();
    let info = cpu::get_cpu_info(&sys);

    assert!(!info.name.is_empty(), "CPU name should not be empty");
    assert!(info.cores > 0, "CPU should have at least 1 core");
    assert!(info.usage >= 0.0, "CPU usage should not be negative");
}
