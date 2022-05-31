use std::collections::HashMap;
use std::fs::{self};

fn main() {
    let header_str = fs::read_to_string("./headers.txt").expect("Could not read `headers.txt`");

    let mut header_vec: Vec<String> = vec![];
    for line in header_str.split("\n") {
        header_vec.push(line.to_string());
    }

    let mut headers_map = HashMap::new();

    for line in header_vec {
        if !(line.contains(':')) {
            continue;
        }

        let header: Vec<&str> = line.split(':').collect();
        headers_map.insert(header[0].to_string(), header[1].trim().to_string());
    }

    println!("{:#?}", headers_map);
}
