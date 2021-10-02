extern crate serde_json;
extern crate geo;

use geo::Point;
use geo::algorithm::euclidean_distance::EuclideanDistance;

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

    let departure_input_latitude = args[1].parse::<f64>().unwrap();
    let departure_input_longitude = args[2].parse::<f64>().unwrap();

    let origin_coordinates: Point<f64> = (
        departure_input_longitude,
        departure_input_latitude
    ).into();

    println!(
        "Searching for departure coordinates [{}, {}] closest polygon point...",
        departure_input_latitude,
        departure_input_longitude,
    );

    // longest possible euclidean distance between two points in Paris
    const PARIS_LONGEST_DISTANCE: f64 = 0.16;
    let mut shortest_distance: f64 = PARIS_LONGEST_DISTANCE;

    let mut departure_polygon_index: usize = 0;

    for (index, point) in points.iter().enumerate() {

        let distance = origin_coordinates.euclidean_distance(&point.coordinates);

        if distance < shortest_distance {
            shortest_distance = distance;
            departure_polygon_index = index;
        }
    }

    let departure_point = points.get(departure_polygon_index).unwrap();
    let departure_coordinates = departure_point.coordinates;
    let departure_latitude = departure_point.coordinates.x();
    let departure_longitude = departure_point.coordinates.y();
    println!(
        "Departure closest polygon point [{}, {}] at index {} (distance = {})",
        departure_longitude,
        departure_latitude,
        departure_polygon_index,
        shortest_distance,
    );

    let arrival_input_latitude = args[3].parse::<f64>().unwrap();
    let arrival_input_longitude = args[4].parse::<f64>().unwrap();

    let origin_coordinates: Point<f64> = (
        arrival_input_longitude,
        arrival_input_latitude
    ).into();

    println!(
        "Searching for arrival coordinates [{}, {}] closest polygon point...",
        arrival_input_latitude,
        arrival_input_longitude,
    );

    let mut shortest_distance: f64 = PARIS_LONGEST_DISTANCE;

    let mut arrival_polygon_index: usize = 0;

    for (index, point) in points.iter().enumerate() {

        let distance = origin_coordinates.euclidean_distance(&point.coordinates);

        if distance < shortest_distance {
            shortest_distance = distance;
            arrival_polygon_index = index;
        }
    }

    let arrival_point = points.get(arrival_polygon_index).unwrap();
    let arrival_latitude = arrival_point.coordinates.x();
    let arrival_longitude = arrival_point.coordinates.y();
    println!(
        "Arrival closest polygon point [{}, {}] at index {} (distance = {})",
        arrival_longitude,
        arrival_latitude,
        arrival_polygon_index,
        shortest_distance,
    );

    println!("Searching for departure in-polygin index...");

    let mut shortest_distance: f64 = PARIS_LONGEST_DISTANCE;

    let polygon = polygons.get(departure_polygon_index).unwrap();
    println!("{:?}", polygon);

    for (index, point) in polygon.iter().enumerate() {

        println!("{:?}", point);

        //let distance = departure_coordinates.euclidean_distance(&point);

        //if distance < shortest_distance {
        //    shortest_distance = distance;
        //    arrival_polygon_index = index;
        //}
    }
}
