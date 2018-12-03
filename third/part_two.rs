use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let buffer = BufReader::new(file);

    let mut hash = HashMap::new();
    let mut ids = Vec::new();
    let mut point;

    for line in buffer.lines() {
        let parts = line.as_ref().unwrap().split(" ").collect::<Vec<_>>();
        let id = parts[0].clone().to_string();
        let location = parts[2].split(":").collect::<Vec<_>>()[0].split(",").collect::<Vec<_>>();
        let size = parts[3].split("x").collect::<Vec<_>>();
        // #1 @ 1,3: 4x4

        ids.push(id.clone());
        
        let from_x = location[0].parse::<i32>().unwrap();
        let to_x = from_x + size[0].parse::<i32>().unwrap();
        let from_y = location[1].parse::<i32>().unwrap();
        let to_y = from_y + size[1].parse::<i32>().unwrap();

        for x in from_x..to_x {
            for y in from_y..to_y {
                point = format!("{}x{}", x, y);

                hash.entry(point).or_insert(vec![]).push(id.clone());
            }
        }
    }

    for (_k, v) in &hash {
        if v.len() > 1 {
            for id in v {
                ids.iter()
                    .position(|n| n == id)
                    .map(|e| ids.remove(e));
            }
        }
    }

    println!("Result count: {}", ids[0]);

    Ok(())
}
