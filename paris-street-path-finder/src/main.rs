extern crate serde_json;

use std::fs::File;
use std::io::{
    BufReader,
    BufRead,
};

struct Point {
    polygon_index: usize,
    coordinates: (f64, f64)
}

fn main() {

    type Polygon = Vec<(f64, f64)>;
    let mut polygons: Vec<Polygon> = Vec::new();

    println!("Building streets polygons list...");

    const POLYGONS_FILE: &str = "data/streets_polygons";
    let file = File::open(POLYGONS_FILE).unwrap();

    let reader = BufReader::new(file);

    for line in reader.lines() {

        let json = line.unwrap();
        let polygon: Vec<(f64, f64)> = serde_json::from_str(&json).unwrap();
        polygons.push(polygon);
    }

    println!("Streets polygons list created.");

    println!("Building streets points list...");

    let mut points: Vec<Point> = Vec::new();

    for (index, polygon) in polygons.iter().enumerate() {

        for point in polygon {

            points.push(
                Point {
                    polygon_index: index,
                    coordinates: (point.0, point.1)
                }
            );
        }
    }

    println!("Streets points list created.");

    for point in points {
        println!("{:?}", point.polygon_index);
        println!("{:?}", point.coordinates.0);
        println!("{:?}", point.coordinates.1);
    }
}
