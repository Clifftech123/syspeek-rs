mod display;
mod system;
mod utils;

use clap::Parser;
use sysinfo::System;

use display::formatter;
use utils::platform;



#[derive(Parser)]
#[command(name = "syspeek", version, about = "A blazingly fast system monitoring tool")]
struct Cli {
    /// Show only CPU information
    #[arg(long)]
    cpu: bool,

    /// Show only memory information
    #[arg(long)]
    memory: bool,

    /// Show only disk information
    #[arg(long)]
    disk: bool,

    /// Show only network information
    #[arg(long)]
    network: bool,

    /// Show top processes
    #[arg(long)]
    processes: bool,
}


fn main() {
   let cli = Cli::parse();
  
  
    let mut sys = System::new_all();
    sys.refresh_all();
    // Small wait then refresh again for accurate CPU readings
    std::thread::sleep(std::time::Duration::from_millis(500));
    sys.refresh_all();
   
   // IF  no flags are provided, show everything
     let show_all = !cli.cpu && !cli.memory && !cli.disk && !cli.network && !cli.processes;
     formatter::banner(env!("CARGO_PKG_VERSION"));
    
    
    if cli.cpu || show_all {
        show_cpu(&sys);
    }

    if cli.memory || show_all {
        show_memory(&sys);
    }

    if cli.disk || show_all {
        show_disk();
    }

    if cli.network || show_all {
        show_network();
    }

    if cli.processes || show_all {
        show_processes(&sys);
    }
  
}

// Shows CPU information with a progress bar for usage
fn show_cpu(sys: &System) {
    let info = system::cpu::get_cpu_info(sys);
    formatter::section_header("System / CPU Information");
    formatter::key_value("Uptime:", &platform::format_uptime(System::uptime()));
    formatter::key_value("Model:", &info.name);
    formatter::key_value("Cores:", &info.cores.to_string());
    formatter::progress_bar("Usage:", info.usage as f64);
    formatter::separator();
}

// Shows memory usage with a progress bar
fn show_memory(sys: &System) {
    let info = system::memory::get_memory_info(sys);
    formatter::section_header("Memory Information");
    formatter::key_value("Total:", &platform::format_bytes(info.total));
    formatter::key_value("Used:", &platform::format_bytes(info.used));
    formatter::key_value("Available:", &platform::format_bytes(info.available));
    formatter::progress_bar("Usage:", info.usage_percent);
    formatter::separator();
}

// Shows disk usage for each mounted disk

fn show_disk() {
    let disks = system::disk::get_disk_info();
    formatter::section_header("Disk Information");
    for disk in &disks {
        let label = format!(
            "{} ({})",
            disk.mount,
            disk.name
        );
        formatter::key_value(
            &label,
            &format!(
                "{} / {}",
                platform::format_bytes(disk.used),
                platform::format_bytes(disk.total)
            ),
        );
        formatter::progress_bar("  Used:", disk.usage_percent);
    }
    formatter::separator();
}


// Shows network interfaces and their traffic
fn show_network() {
    let interfaces = system::network::get_network_info();
    formatter::section_header("Network Information");
    for net in &interfaces {
        formatter::key_value("Interface:", &net.name);
        formatter::key_value("  Received:", &platform::format_bytes(net.received));
        formatter::key_value("  Sent:", &platform::format_bytes(net.transmitted));
    }
    formatter::separator();
}


// Shows top processes by memory usage
fn show_processes(sys: &System) {
    let procs = system::process::get_top_processes(sys, 5);
    formatter::section_header("Top Processes (by memory)");
    for p in &procs {
        let detail = format!(
            "PID {} | {} | CPU {:.1}%",
            p.pid,
            platform::format_bytes(p.memory),
            p.cpu_usage
        );
        formatter::key_value(&p.name, &detail);
    }
    formatter::separator();
}