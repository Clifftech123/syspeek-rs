use syspeek::system::network;

#[test]
fn test_network_info_returns_interfaces() {
    let interfaces = network::get_network_info();

    for net in &interfaces {
        assert!(!net.name.is_empty(), "Interface name should not be empty");
    }
}
