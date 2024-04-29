use std::net::{Ipv4Addr, SocketAddr, IpAddr};
use tokio::time::timeout;
use tokio::net::TcpStream;
use std::time::Duration;
use crate::error::CustomError;
use crate::subnet::Subnet;
use crate::ports::PortSelector;

//Box alocates memory on the heap for unknown sized data at compiile time
//dyn trait means that the trait si being used as if it was a struct
//This way the scanner owns a trait implemented object on itself
pub struct CrustScanner{
    subnet: Subnet,
    port_selector: Box<dyn PortSelector>,
    tries: u8,
    timeout: u32,
}

impl CrustScanner{
    pub fn new(
        ip_address: Ipv4Addr,
        netmask_bit: u8,
        port_selector: Box<dyn PortSelector>,
        tries: u8,
        timeout: u32
    ) -> Self{
        let subnet = Subnet::new(ip_address, netmask_bit);
        return Self{
            subnet: subnet,
            port_selector: port_selector,
            tries: tries,
            timeout: timeout
        }
    }

    pub async fn scan(&self){
        let hosts = match self.subnet.get_hosts_ip_range() {
            Ok(addresses) => addresses,
            Err(err) => panic!("{}", err)
        };
        let ports = self.port_selector.get_ports();
        for ip_address in hosts{
            println!("Scanning IP {}", ip_address);
            //Instead of copying the values in ports, we make a reference to each value so 
            //the ownership is not changed
            for port in &ports{
                println!("Scanning {}", port);
                let socket = SocketAddr::new(IpAddr::V4(ip_address), *port);
                match self.connect(socket).await {
                    Ok(_) => println!("Port {} for IP {} up!", port, ip_address),
                    Err(_) => continue
                }
            }
        }
    }

    async fn connect(&self, socket: SocketAddr) -> Result<TcpStream, CustomError>{
        let timeout_duration = Duration::from_millis(self.timeout as u64);
        for _ in 0..= self.tries{
            match timeout(timeout_duration, TcpStream::connect(socket)).await{
                Ok(Ok(stream)) => return Ok(stream),
                Ok(Err(_)) => return Err(CustomError::Connection),
                Err(_) => continue
            };
        }
        eprintln!("Timeout error on address {} and port {}", socket.ip(), socket.port());
        return Err(CustomError::Timeout);
    }
}

#[cfg(test)]
pub mod test;