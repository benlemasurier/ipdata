extern crate ipdata;

use std::net;

fn main() {
    let ip = net::Ipv4Addr::new(1,1,1,1);
    let resp = ipdata::lookup(net::IpAddr::V4(ip));

    match resp {
        Ok(resp) => {
            println!("{}: {}, {}",
                     resp.organization(),
                     resp.latitude(), resp.longitude());
        }
        Err(err) => {
            eprintln!("error: {}", err);
            ::std::process::exit(1);
        }
    }
}
