use std::net::Ipv4Addr;
use super::Subnet;
use super::get_number_of_hosts;

const IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const NETMASK_BIT: u8 = 29;

fn subnet(ip_address: Ipv4Addr, netmask_bit: u8) -> Subnet{
    return Subnet::new(ip_address, netmask_bit);
}

#[test]
fn test_get_reserved_addresses(){
    let subnet = subnet(IP, NETMASK_BIT);
    let real_network_ip = Ipv4Addr::new(127, 0, 0, 0);
    let real_broadcast_ip = Ipv4Addr::new(127, 0, 0, 7);
    assert_eq!(subnet.network_ip, real_network_ip);
    assert_eq!(subnet.broadcast_ip, real_broadcast_ip);
}

#[test]
fn test_get_number_of_hosts(){
    let number_of_hosts_29 = get_number_of_hosts(29);
    let number_of_hosts_28 = get_number_of_hosts(28);
    let number_of_hosts_32 = get_number_of_hosts(32);
    assert_eq!(number_of_hosts_29, 6);
    assert_eq!(number_of_hosts_28, 14);
    assert_eq!(number_of_hosts_32, 1);
}

#[test]
fn test_get_hosts_ip_range(){
    let netmask_bit: u8 = 32;
    let subnet = Subnet::new(IP, netmask_bit);
    let number_of_hosts = get_number_of_hosts(netmask_bit);
    let hosts_result = subnet.get_hosts_ip_range();
    match hosts_result{
        Ok(hosts) => {
            assert_eq!(hosts.len(), number_of_hosts as usize);
        }
        Err(_) => {
            assert!(false);
        }
    }
    
}
