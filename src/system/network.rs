use sysinfo::Networks;

pub struct NetworkInfo {
    pub name: String,
    pub received: u64,
    pub transmitted: u64,
}

// Retrieves network information from the system
pub fn get_network_info() -> Vec<NetworkInfo> {
    let networks = Networks::new_with_refreshed_list();
    let mut result = Vec::new();

    for (name, data) in networks.iter() {
        result.push(NetworkInfo {
            name: name.to_string(),
            received: data.total_received(),
            transmitted: data.total_transmitted(),
        });
    }

    result
}