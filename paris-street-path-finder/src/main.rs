extern crate serde_json;

use std::fs::File;
use std::io::{
    BufReader,
    BufRead,
};

fn main() {

    type Polygon = Vec<(f64, f64)>;
    let mut street_polygons: Vec<Polygon> = Vec::new();

    println!("Building streets polygons list...");

    const POLYGONS_FILE: &str = "data/streets_polygons";
    let file = File::open(POLYGONS_FILE).unwrap();

    let reader = BufReader::new(file);

    for line in reader.lines() {

        let json = line.unwrap();
        let polygon: Vec<(f64, f64)> = serde_json::from_str(&json).unwrap();
        street_polygons.push(polygon);
    }

    println!("Streets polygons list created.");
}
