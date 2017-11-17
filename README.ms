# fe2o3
This binary exposes a GeoIP resolution service, capable of working with the
MaxMind country dataset. It aims to be small in footprint, and incredibly simple
so its author can glue it together as a data enricher when trawling logs. If it
can't find the country code, it'll return "zz" in its response.

## Build instructions
`cargo build`. Job done.

## Firing up
`fe2o3 -i "localhost:4000" -c GeoLite2-Country.mmdb`

## Querying with it
`curl http://localhost:8080/127.0.0.1`
