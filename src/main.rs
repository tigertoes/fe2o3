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
                    .arg(Arg::with_name("maxminddb-city")
                        .short("t")
                        .long("maxminddb-city")
                        .required(true)
                        .value_name("FILE")
                        .help("Path to Maxmind country database"))
                    .arg(Arg::with_name("maxminddb-as")
                        .short("as")
                        .long("maxminddb-as")
                        .required(true)
                        .value_name("FILE")
                        .help("Path to Maxmind AS database"))
                    .get_matches();

    let addr = matches.value_of("interface").unwrap_or(DEFAULT_IFACE).parse().unwrap();
    let maxdb_cc = matches.value_of("maxminddb-country").unwrap().to_string();
    let maxdb_ct = matches.value_of("maxminddb-city").unwrap().to_string();
    let maxdb_as = matches.value_of("maxminddb-as").unwrap().to_string();

    let cc_reader = Arc::new(maxminddb::Reader::open(&maxdb_cc).unwrap());
    let ct_reader = Arc::new(maxminddb::Reader::open(&maxdb_ct).unwrap());
    let as_reader = Arc::new(maxminddb::Reader::open(&maxdb_as).unwrap());

    let server = Http::new().bind(&addr, move || 
        Ok(GeoIPService::new(&cc_reader, &ct_reader, &as_reader))).unwrap();
    server.run().unwrap();
}
