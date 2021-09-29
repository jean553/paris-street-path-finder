extern crate serde_json;
extern crate geo;

use geo::Point;

use std::fs::File;
use std::io::{
    BufReader,
    BufRead,
};
use std::env;

struct PolygonPoint {
    polygon_index: usize,
    coordinates: Point<f64>
}

fn main() {

    type Polygon = Vec<Point<f64>>;
    let mut polygons: Vec<Polygon> = Vec::new();

    println!("Building streets polygons list...");

    const POLYGONS_FILE: &str = "data/streets_polygons";
    let file = File::open(POLYGONS_FILE).unwrap();

    let reader = BufReader::new(file);

    for line in reader.lines() {

        let json = line.unwrap();
        let points: Vec<(f64, f64)> = serde_json::from_str(&json).unwrap();

        let mut polygon: Vec<Point<f64>> = Vec::new();
        for point in points {
            polygon.push(point.into());
        }

        polygons.push(polygon);
    }

    println!("Streets polygons list created.");

    println!("Building streets points list...");

    let mut points: Vec<PolygonPoint> = Vec::new();

    for (index, polygon) in polygons.iter().enumerate() {

        for point in polygon {

            points.push(
                PolygonPoint {
                    polygon_index: index,
                    coordinates: point.clone(),
                }
            );
        }
    }

    println!("Streets points list created.");

    let args: Vec<String> = env::args().collect();

    let latitude = args[1].parse::<f64>().unwrap();
    let longitude = args[2].parse::<f64>().unwrap();

    let origin_coordinates: Point<f64> = (latitude, longitude).into();

    println!("Searching for [{}, {}] coordinates closest polygon point...", latitude, longitude);
}
