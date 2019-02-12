ipdata is an ipdata.co rust api client

### Configuration
#### Required
Configure your ipdata.co API key via the `IPDATA_KEY` environment variable.

#### Optional
Change the ipdata.co default api endpoint with the `IPDATA_URL` environment variable.


### Example

```rust
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
```

```
cargo run --example lookup

Cloudflare, Inc.: -33.494, 143.2104
```
