use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use rand::Rng;
use std::fs::OpenOptions;
#[derive(Debug, Deserialize)]
pub struct Record {
    // for reading in edges with serde
    user_loc:String,
    fr_loc:String,
    scaled_sci:u32
}


fn read_to_map(path: &str, cutoff : u32) ->  HashMap<String, Vec<(String, u32)>>{
    let rdr = csv::ReaderBuilder::new()
    .delimiter(b'\t')
    .has_headers(true)
    .flexible(true)
    .from_path(path);
    let mut graph_list : HashMap<String, Vec<(String, u32)>> = HashMap::new();
    for result in rdr.expect("Something failed").deserialize(){ //skips first line since that's the number of vertices
        let record: Record = result.expect("Something failed");
        println!("{:?} This is a location", record.user_loc);
        //graph_list.insert(record.user_loc, (record.fr_loc, record.scaled_sci));
        if record.scaled_sci > cutoff{
            graph_list.entry(record.user_loc).or_insert(Vec::new()).push((record.fr_loc, record.scaled_sci));
        }
            //graph_list[&record.user_loc].push((record.fr_loc, record.scaled_sci));
    }
    return graph_list
}

// fn write_test(){
//     // DO NOT USE
//     // Generates a file for a test, 
//     let path = String::from("test_new.tsv");
//     let csv_path = String::from("data/data.tsv");
//     let mut file = File::create(&path).expect("Unable to create file");
//     let mut file = OpenOptions::new()
//         .append(true)
//         .open(&path)
//         .expect("cannot open file");
//     let rdr = csv::ReaderBuilder::new()
//         .delimiter(b'\t')
//         .has_headers(true)
//         .flexible(true)
//         .from_path(csv_path);
//         //let mut graph_list : HashMap<String, (String, u32)> = HashMap::new();
//     println!("tester");
//     for result in rdr.expect("Something failed").deserialize(){ //skips first line since that's the number of vertices
//         let record: Record = result.expect("Something failed");
//         println!("{:?} This is a location", record.user_loc);
//         let rng = rand::thread_rng().gen_range(0..1000);
//         if rng == 0{
//             let s: String = format!("{0}\t{1}\t{2}\n", record.user_loc, record.fr_loc, record.scaled_sci);
//             file.write_all(s.as_bytes()).expect("Unable to write file");        
//         }
//     }
//     println!("Done!");
// }



fn main() {
    let start = Instant::now();
    println!("Hello, world!");
    let adjacency_map : HashMap<String, Vec<(String, u32)>> = read_to_map("test_new.tsv", 10000000 as u32);
    println!("{:?}", adjacency_map);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
