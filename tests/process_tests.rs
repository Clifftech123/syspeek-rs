use sysinfo::System;
use syspeek::system::process;

#[test]
fn test_top_processes_respects_count() {
    let mut sys = System::new_all();
    sys.refresh_all();
    let procs = process::get_top_processes(&sys, 3);

    assert!(procs.len() <= 3, "Should return at most 3 processes");
}

#[test]
fn test_top_processes_sorted_by_memory() {
    let mut sys = System::new_all();
    sys.refresh_all();
    let procs = process::get_top_processes(&sys, 5);

    for i in 1..procs.len() {
        assert!(
            procs[i - 1].memory >= procs[i].memory,
            "Processes should be sorted by memory descending"
        );
    }
}
