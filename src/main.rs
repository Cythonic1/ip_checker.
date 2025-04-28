use std::net::{IpAddr, Ipv4Addr};
use clap::Parser;

fn ip_parser(ip: String) -> Ipv4Addr {
    let ip_res: Result<IpAddr, _>   = ip.parse();

    let _isv4 = match ip_res {
        Ok(res) => {
            match res {
                IpAddr::V4(ip) => {
                    return ip
                },
                IpAddr::V6(_) => {
                    eprintln!("Ipv6 is not supported");
                    std::process::exit(1); // exit because no valid IP
                }
            }
        },
        Err(err) => {
            eprintln!("Error parsing {err}");
            std::process::exit(1); // exit because no valid IP
        }
    };


}
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(short, long)]
    ip: String,


    #[arg(short, long)]
    subnet_mask: String,


    #[arg(short, long)]
    other_ip: String

}

fn bit_wise_ip_mask(ip: Ipv4Addr, subnetmask: Ipv4Addr, another_ip : Ipv4Addr) {
    let ip_bytes = ip.octets();
    let subnet_bytes = subnetmask.octets();
    let another_ip_bytes = another_ip.octets();

    println!("ip bytes: {:?}", ip_bytes);
    println!("subnet mask bytes: {:?}", subnet_bytes);
    let network_bytes = [
        (ip_bytes[0] ^ another_ip_bytes[0]) & subnet_bytes[0],
        (ip_bytes[1] ^ another_ip_bytes[1]) & subnet_bytes[1],
        (ip_bytes[2] ^ another_ip_bytes[2]) & subnet_bytes[2],
        (ip_bytes[3] ^ another_ip_bytes[3]) & subnet_bytes[3],
    ];

    println!("results : {:?}", network_bytes);
}
fn main() {
    let args = Args::parse();

    let ip = ip_parser(args.ip);
    let subnet_mask = ip_parser(args.subnet_mask);
    let another_ip = ip_parser(args.other_ip);

    bit_wise_ip_mask(ip, subnet_mask, another_ip);
    println!("IP: {:?}", ip);

}
