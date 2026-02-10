use sysinfo::System;

// CPU Information Structure
pub struct CpuInfo {
    pub name: String,
    pub cores: usize,
    pub usage: f32,
}

// Retrieves CPU information from the system
pub fn get_cpu_info(system: &System) -> CpuInfo {
   let name  = if let Some(cpu) = system.cpus().first() {
        cpu.brand().to_string()
    } else {
        "Unknown CPU".to_string()
    };

    let cores = system.cpus().len();
    let usage = system.global_cpu_usage();

    CpuInfo {
        name,
        cores,
        usage,
    }
}