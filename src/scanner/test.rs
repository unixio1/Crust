use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use crate::ports::{ManualStrategy, PortSelector};

use super::CrustScanner;

fn get_test_scanner() -> CrustScanner{
    const IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    const NETMASKBIT: u8 = 32;
    const TRIES: u8 = 5;
    const TIMEOUT: u32 = 500;
    let port_selector: Box<dyn PortSelector> = Box::new(ManualStrategy{selected_ports: vec![22]});
    return CrustScanner::new(IP,
                            NETMASKBIT,
                            port_selector,
                            TRIES,
                            TIMEOUT
                        );
}

#[tokio::test]
async fn test_socket_connection(){
    let is_port_opened = false;
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let scanner = get_test_scanner();
    let response = scanner.connect(socket).await;
    match response{
        Ok(_) => assert!(is_port_opened),
        Err(_) => assert!(!is_port_opened),
    }
}

