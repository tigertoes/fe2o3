extern crate fe2o3;
extern crate clap;
extern crate maxminddb;
extern crate hyper;

use clap::{Arg, App};
use fe2o3::GeoIPService;

use std::sync::Arc;
use hyper::server::Http;
static VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Default interface to start up on
static DEFAULT_IFACE: &'static str = "127.0.0.1:8080";

fn main() {
    let matches = App::new("fe2o3")
                    .version(VERSION)
                    .arg(Arg::with_name("interface")
                        .short("i")
                        .long("interface")
                        .value_name("INTERFACE")
                        .takes_value(true)
                        .help("Interface to listen to"))
                    .arg(Arg::with_name("maxminddb-country")
                        .short("c")
                        .long("maxminddb-country")
                        .required(true)
                        .value_name("FILE")
                        .help("Path to Maxmind country database"))
                    .get_matches();

    let addr = matches.value_of("interface").unwrap_or(DEFAULT_IFACE).parse().unwrap();
    let maxdb_cc = matches.value_of("maxminddb-country").unwrap().to_string();
    let cc_reader = Arc::new(maxminddb::Reader::open(&maxdb_cc).unwrap());

    let server = Http::new().bind(&addr, move || 
        Ok(GeoIPService::new(&cc_reader))).unwrap();
    server.run().unwrap();
}
