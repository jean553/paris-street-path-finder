use std::fs::File;
use std::io::{
    BufReader,
    BufRead,
};

fn main() {

    println!("Building streets polygons list...");

    const POLYGONS_FILE: &str = "data/streets_polygons";
    let file = File::open(POLYGONS_FILE).unwrap();

    let reader = BufReader::new(file);

    for line in reader.lines() {

        let json = line.unwrap();
        println!("{}", json);
    }
}
