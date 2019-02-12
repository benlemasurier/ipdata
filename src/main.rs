extern crate ipdata;

use std::env;
use std::net;

fn main() {
    let mut addr: Option<net::IpAddr> = None;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].parse::<net::IpAddr>() {
            Ok(ip) => addr = Some(ip),
            Err(e) => {
                eprintln!("failed to parse ip: {:?}", e);
            }
        }
    }

    let data = ipdata::lookup(addr);
    match data {
        Ok(data) => {
            println!("{:?}", data.ip());
            println!("{:?}", data);
        }
        Err(err) => {
            eprintln!("Application error: {}", err);
            ::std::process::exit(1);
        }
    }
}
