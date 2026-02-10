use sysinfo::System;

// Process Information Structure

pub struct ProcessInfo {
    pub name: String,
    pub pid: u32,
    pub memory: u64,
    pub cpu_usage: f32,
}

// Retrieves top memory-consuming processes from the system
pub fn get_top_processes(sys: &System, count: usize) -> Vec<ProcessInfo> {
    let mut processes: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .map(|(pid, process)| ProcessInfo {
            name: process.name().to_string_lossy().to_string(),
            pid: pid.as_u32(),
            memory: process.memory(),
            cpu_usage: process.cpu_usage(),
        })
        .collect();

    processes.sort_by(|a, b| b.memory.cmp(&a.memory));
    processes.truncate(count);
    processes
}