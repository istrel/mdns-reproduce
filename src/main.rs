extern crate mdns_responder;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate time;
extern crate net;

use env_logger::LogBuilder;
use log::{LogLevelFilter, LogRecord};
use std::env;
// use net;
// use std::net::{Ipv4Addr,IpAddr};


pub static LANLORD_DISCOVERY_PORT: u16 = 14821;
pub static LANLORD_SERVICE_NAME: &'static str = "_lanlord._tcp";

fn main() {
    start_env_logger();

    let txt = [];

    // let mut bound = false;
    // for iface in net::getifaddrs() {
    //     match iface.ip() {
    //         Some(IpAddr::V4(ip)) => {
    //             trace!("Joining IPv4 {:?} to the multicast group", ip);
    //             match socket.join_multicast_v4(&Ipv4Addr::new(224,0,0,251),&ip) {
    //                 Ok(_) => bound = true,
    //                 Err(err) => warn!("Failed to join to the multicast group: {}", err.to_string()),
    //             }
    //         },
    //         _ => continue,
    //     };
    // };
    // if !bound {
    //     warn!("Failed joining to every multicast group. Falling back to 0.0.0.0");
    //     return socket.join_multicast_v4(
    //         &Ipv4Addr::new(224,0,0,251),
    //         &Ipv4Addr::new(0,0,0,0)
    //     );
    // };

    mdns_responder::Responder::new().unwrap().register(
        LANLORD_SERVICE_NAME.to_owned(),
        "Lancastr Lanlord".to_owned(),
        LANLORD_DISCOVERY_PORT,
        &txt,
    );

    loop {
        ::std::thread::sleep(::std::time::Duration::from_secs(50));
    }
}

fn start_env_logger() {
    let formatter = |record: &LogRecord| {
        let t = time::now();
        format!(
            "{},{:03} - {} - {} - {}",
            time::strftime("%Y-%m-%d %H:%M:%S", &t).unwrap(),
            t.tm_nsec / 1_000_000,
            record.level(),
            record.target(),
            record.args()
        )
    };

    let mut builder = LogBuilder::new();
    builder.format(formatter).filter(None, LogLevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder.init().unwrap();
}
