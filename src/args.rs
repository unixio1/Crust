use clap::Parser;
use crate::{ports::{ManualStrategy, PortSelector, RandomStrategy, RangeStrategy}, scanner::CrustScanner};
use std::net::Ipv4Addr;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct CrustArgs{
    #[clap(short, long)]
    ///An IP address on the subnet to scan
    pub ip: String,
    #[clap(short, long)]
    ///The netmask bit value of the subnet in CIDR notation
    pub netmask_bit: u8,
    #[clap(short, long)]
    ///The strategy to follow when scanning the ports: 1 ports in a range, 2: random ports in a range and 3: A selection of ports
    pub strategy: u8,
    #[clap(short, long)]
    ///The values required for the selected strategy
    pub port_vlues: String,
    #[clap(short, long)]
    ///In the case of the random strategy, the amount of ports to scan
    pub amount: Option<u16>,
    #[clap(short, long)]
    ///Number of tries after failed connection
    pub tries: u8,
    #[clap(short='m', long)]
    ///Timeout for each connection (miliseconds)
    pub timeout: u32
}

pub fn parse_arguments() -> CrustScanner{
    let args = CrustArgs::parse();
    let ip_address: Ipv4Addr = args.ip.parse().expect("Invalid IP address");
    let port_strategy = match args.strategy{
        1 => get_range_strategy(args.port_vlues),
        2 => get_random_strategy(args.port_vlues),
        3 => get_manual_strategy(args.port_vlues),
        _ => panic!("Non existent strategy!")
    };
    return CrustScanner::new(ip_address, args.netmask_bit, port_strategy, args.tries, args.timeout);
}

fn get_range_strategy(ports_argument: String) -> Box<dyn PortSelector>{
    let ports_range = get_ports_range(ports_argument);
    return Box::new(RangeStrategy{low: ports_range[0], high: ports_range[1]});
}

fn get_random_strategy(ports_argument: String) -> Box<dyn PortSelector>{
    let ports_range = get_ports_range(ports_argument);
    let amount = ports_range.len() / 2;
    return Box::new(RandomStrategy{low: ports_range[0], high: ports_range[1], amount: amount});
}

fn get_manual_strategy(ports_argument: String) -> Box<dyn PortSelector>{
    let ports = get_selection_ports(ports_argument);
    return Box::new(ManualStrategy{selected_ports: ports});
}

fn get_ports_range(ports_argument: String) -> [u16; 2]{
    let ports_string = ports_argument.replace(" ", "");
    let parts: Vec<&str> = ports_string.split(",").collect();
    if parts.len() != 2{
        panic!("Error in specified port range (desired format: low, high)");
    }
    let mut range: [u16; 2] = [0; 2];
    for i in 0..2{
        range[i] = match parts[0].parse::<u16>(){
            Ok(value) => value,
            Err(_) => panic!("Error parsing the speficied port range")
        };
    }
    return range;
}

fn get_selection_ports(ports_argument: String) -> Vec<u16>{
    let ports_string = ports_argument.replace(" ", "");
    let parts: Vec<&str> = ports_string.split(",").collect();
    let n_ports = parts.len();
    let mut range: Vec<u16> = Vec::with_capacity(n_ports);
    for i in 0..n_ports{
        let value = match parts[i].parse::<u16>(){
            Ok(value) => value,
            Err(_) => panic!("Error parsing the speficied port range")
        };
        range.push(value);
    }
    return range;
}