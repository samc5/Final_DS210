use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use rand::Rng;
use std::fs::OpenOptions;
use std::collections::BinaryHeap;

#[derive(Debug, Deserialize)]
pub struct Record {
    // for reading in edges with serde
    user_loc:String,
    fr_loc:String,
    scaled_sci:usize
}

#[derive(Debug,Clone)]
struct Outedge {
    vertex: String,
    length: usize,
}



fn read_to_map(path: &str, cutoff : usize) ->  HashMap<String, Vec<Outedge>>{
    let rdr = csv::ReaderBuilder::new()
    .delimiter(b'\t')
    .has_headers(true)
    .flexible(true)
    .from_path(path);
    let mut graph_list : HashMap<String, Vec<Outedge>> = HashMap::new();
    for result in rdr.expect("Something failed").deserialize(){ //skips first line since that's the number of vertices
        let record: Record = result.expect("Something failed");
        
        //println!("{:?} This is a location", record.user_loc);
        //graph_list.insert(record.user_loc, (record.fr_loc, record.scaled_sci));
        if record.scaled_sci > cutoff{
            graph_list.entry(record.user_loc).or_insert(Vec::new()).push(Outedge{vertex: record.fr_loc, length: record.scaled_sci});
        }
            //graph_list[&record.user_loc].push((record.fr_loc, record.scaled_sci));
    }
    return graph_list
}







fn shortest_paths(map: &HashMap<String, Vec<Outedge>>, start : String){
    // Repurposed from class to work with a hashmap that includes an Outedge (a weighted edge)
    let mut distances: HashMap<&String, Option<usize>> = HashMap::new();
    distances.entry(&start).or_insert(Some(0));
    let mut pq = BinaryHeap::<(usize,&String)>::new();
    pq.push((0,&start));
    // the real stf
    while let Some((dist,v)) = pq.pop() {
        let rng = rand::thread_rng().gen_range(0..50000);
        // problem is when a node connects to itself
        if rng == 0{
            println!("The while loop is at the top, the some is {:?}, {:?}", dist, v);
        }
        match map.get(v) {
            None => {break},
            Some(edges) => {
                for Outedge{vertex,length} in edges.iter() {
                    //println!("{:?}", vertex);
                    let new_dist = dist + *length;
                    let update = match distances.get(vertex) {
                        None => {true}
                        Some(d) => {
                            match d{
                                None => {true},
                                Some(real_d) => v != vertex && {new_dist > *real_d} // the first part of the and is to account for the vertex being connected to itself
                            }
                        }
                    };
                    if update {
                        distances.entry(vertex).or_insert(Some(new_dist)); //may be missing something
                        pq.push((new_dist,vertex));
                    }
                }
            }
        };

    };
    println!("{:?}", distances);

}

fn write_test(){
    // DO NOT USE
    // Generates a file for a test, 
    let path = String::from("test_new.tsv");
    let csv_path = String::from("data/data.tsv");
    let mut file = File::create(&path).expect("Unable to create file");
    let mut file = OpenOptions::new()
        .append(true)
        .open(&path)
        .expect("cannot open file");
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .flexible(true)
        .from_path(csv_path);
        //let mut graph_list : HashMap<String, (String, u32)> = HashMap::new();
    println!("tester");
    for result in rdr.expect("Something failed").deserialize(){ //skips first line since that's the number of vertices
        let record: Record = result.expect("Something failed");
        println!("{:?} This is a location", record.user_loc); 
        let rng = rand::thread_rng().gen_range(0..1000); // 1 in 1000 chance for each line to be included
        if rng == 0{
            let s: String = format!("{0}\t{1}\t{2}\n", record.user_loc, record.fr_loc, record.scaled_sci);
            file.write_all(s.as_bytes()).expect("Unable to write file");        
        }
    }
    println!("Done!");
}



fn main() {
    let start = Instant::now();
    println!("Hello, world!");
    let adjacency_map : HashMap<String, Vec<Outedge>> = read_to_map("test_new.tsv", 10000 as usize);
    //let adjacency_map : HashMap<String, Vec<Outedge>> = read_to_map("data/data.tsv", 100000 as usize);
    for i in adjacency_map.keys(){
       // println!("{:?}", i);
        shortest_paths(&adjacency_map, String::from(i));
    }
        //println!("{:?}", adjacency_map);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
