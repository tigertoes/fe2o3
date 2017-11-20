extern crate maxminddb;
extern crate hyper;
extern crate futures;

use futures::future::Future;
use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};
use maxminddb::geoip2::Country;
use maxminddb::MaxMindDBError;

use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;

/// Country code to render when we can't find a country for the IP...
static NOT_FOUND: &'static str = "zz";

pub struct GeoIPService<'a> {
    country: &'a Arc<maxminddb::Reader>
}

impl<'a> GeoIPService<'a> {
    pub fn new(cc: &Arc<maxminddb::Reader>) -> GeoIPService {
        GeoIPService {
            country: cc
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
        let result: Result<Country, MaxMindDBError> = self.country.lookup(ip);

        let country_code = match result {
            Ok(country) => match country.country {
                Some(c) => c.iso_code.unwrap_or(NOT_FOUND.to_string()),
                None    => NOT_FOUND.to_string()
            }
            Err(_why)   => NOT_FOUND.to_string()
        };

        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(country_code.len() as u64))
                .with_body(country_code)
        ))
    }
}
