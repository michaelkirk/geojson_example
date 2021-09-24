extern crate geo_types;
extern crate geojson;

use geojson::GeoJson;
use std::convert::TryInto;
use std::str::FromStr;

fn main() {
    let geojson_str = r#"
{
 "type": "Feature",
 "properties": {},
 "geometry": {
   "type": "Point",
   "coordinates": [
     -0.13583511114120483,
     51.5218870403801
   ]
 }
}
"#;

    let geojson = GeoJson::from_str(geojson_str).unwrap();
    let geom: geo_types::Geometry<f64> = geojson.try_into().unwrap();
    println!("Success! {:?}", geom);
}
