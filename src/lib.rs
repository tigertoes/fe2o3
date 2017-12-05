extern crate maxminddb;
extern crate hyper;
extern crate futures;

use futures::future::Future;
use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};
use maxminddb::geoip2::{Country, City, Isp};
use maxminddb::MaxMindDBError;

use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;

/// Country code to render when we can't find a country for the IP...
static CC_NOT_FOUND: &'static str = "zz";

/// Number to render when we are unable to find the Autonomous System Number...
static AS_NOT_FOUND: &'static str = "0";

// TODO: Add City database support
pub struct GeoIPService<'a> {
    country: &'a Arc<maxminddb::Reader>,
    city: &'a Arc<maxminddb::Reader>,
    autonomous_system: &'a Arc<maxminddb::Reader>
}

impl<'a> GeoIPService<'a> {
    pub fn new(cc: &'a Arc<maxminddb::Reader>,
               ct: &'a Arc<maxminddb::Reader>,
               a_s: &'a Arc<maxminddb::Reader>) -> Self {
        GeoIPService {
            country: cc,
            city: ct,
            autonomous_system: a_s
        }
    }
}

impl<'a> Service for GeoIPService<'a> {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let ip: IpAddr = FromStr::from_str(&req.path()[1..]).unwrap();
        let cc_result: Result<Country, MaxMindDBError> = self.country.lookup(ip);
        let ct_result: Result<City, MaxMindDBError> = self.city.lookup(ip);
        let as_result: Result<Isp, MaxMindDBError> = self.autonomous_system.lookup(ip);

        // TODO: Add more data as headers

        let country_code = match cc_result {
            Ok(country) => match country.country {
                Some(c) => c.iso_code.unwrap_or(CC_NOT_FOUND.to_string()),
                None    => CC_NOT_FOUND.to_string()
            }
            Err(_why)   => CC_NOT_FOUND.to_string()
        };

        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(country_code.len() as u64))
                .with_body(country_code)
        ))
    }
}
