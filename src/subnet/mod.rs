use std::net::Ipv4Addr;
use crate::constants::MAX_INDEXABLE_HOSTS;
use crate::error::CustomError;

pub struct Subnet{
    network_ip: Ipv4Addr,
    broadcast_ip: Ipv4Addr,
    netmask_bit: u8
}

impl Subnet{

    pub fn new(ip_address: Ipv4Addr, netmask_bit: u8) -> Self{
        return get_subnet_from_address(ip_address, netmask_bit);
    }

    pub fn get_hosts_ip_range(&self)-> Result<Vec<Ipv4Addr>, CustomError>{
        let number_of_hosts = get_number_of_hosts(self.netmask_bit);
        println!("{}, {}", self.network_ip, self.broadcast_ip);
        if number_of_hosts > MAX_INDEXABLE_HOSTS{
            return Err(CustomError::TooManyHosts);
        }
        let mut hosts = Vec::new();
        let first_ip = u32::from(self.network_ip);
        let last_ip = u32::from(self.broadcast_ip);
        for ip in first_ip..=last_ip{
            hosts.push(Ipv4Addr::from(ip))
        }
        return Ok(hosts);
    }
}

fn get_subnet_from_address(ip: Ipv4Addr, netmask_bit: u8) -> Subnet{
    let netmask_octets = get_netmask_octets(netmask_bit);
    let mut network_octets: [u8; 4] = [0; 4];
    let mut broadcast_octets: [u8; 4] = [0; 4];
    let provided_ip_octets = ip.octets();
    for i in 0..4{
        network_octets[i] =  netmask_octets[i] & provided_ip_octets[i];
        let inverted_octet = !netmask_octets[i] & 255;
        broadcast_octets[i] =  inverted_octet | provided_ip_octets[i];
    }
    let network_ip = Ipv4Addr::new(network_octets[0], network_octets[1],
        network_octets[2], network_octets[3]);
    let broadcast_ip = Ipv4Addr::new(broadcast_octets[0], broadcast_octets[1],
        broadcast_octets[2], broadcast_octets[3]);
        
    return Subnet{
        network_ip: network_ip,
        broadcast_ip: broadcast_ip,
        netmask_bit: netmask_bit
    };
}

fn get_netmask_octets(netmask_bit: u8) -> [u8; 4]{
    let netmask_bits = u32::MAX << (32 - netmask_bit); //bitwise representation of the netmask bit
    return [
        ((netmask_bits >> 24) & 255) as u8,
        ((netmask_bits >> 16) & 255) as u8,
        ((netmask_bits >> 8) & 255) as u8,
        (netmask_bits & 255) as u8,
    ];
}

pub fn get_number_of_hosts(netmask_bit: u8) -> u32{
    if netmask_bit >= 32{
        return 1;
    }
    let base_two: i32 = 2;
    let network_bits = (32 - netmask_bit) as u32;
    (base_two.pow(network_bits) - 2) as u32
}

#[cfg(test)]
pub mod test;