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

#[derive(Debug,Clone)]
struct CountryPair {
    count: usize,
    distance: usize,
}

#[derive(Debug, Deserialize)]
struct CountyRecord{
    key: String,
    level: String
}



fn counties_map() -> HashMap<String, String>{
    let path: &str = "data/counties2.csv";
    let rdr = csv::ReaderBuilder::new()
    .delimiter(b',')
    .has_headers(true)
    .flexible(true)
    .from_path(path);
    let mut county_map : HashMap<String, String> = HashMap::new();
    for result in rdr.expect("something failed").deserialize(){
        let record : CountyRecord = result.expect("Something failed");
        county_map.insert(record.key, record.level);
    }
    return county_map;
}



fn read_to_map(path: &str, cutoff : usize) ->  HashMap<String, Vec<Outedge>>{
    let rdr = csv::ReaderBuilder::new()
    .delimiter(b'\t')
    .has_headers(true)
    .flexible(true)
    .from_path(path);
    let mut graph_list : HashMap<String, Vec<Outedge>> = HashMap::new();
    let county_map = counties_map();
    for result in rdr.expect("Something failed").deserialize(){ 
        let record: Record = result.expect("Something failed");
        let level = match county_map.get(&record.user_loc){
            None => "None",
            Some(val) => {val}
        };
      //  println!("{:?}", level);
        //println!("{:?} This is a location", record.user_loc);
        if record.scaled_sci > cutoff{
            graph_list.entry(record.user_loc).or_insert(Vec::new()).push(Outedge{vertex: record.fr_loc, length: record.scaled_sci});
        }
            //graph_list[&record.user_loc].push((record.fr_loc, record.scaled_sci));
    }
    return graph_list
}





fn read_to_map_aggregate(path: &str, cutoff : usize) ->  HashMap<String, Vec<Outedge>>{
    let rdr = csv::ReaderBuilder::new()
    .delimiter(b'\t')
    .has_headers(true)
    .flexible(true)
    .from_path(path);
    let mut counts_map : HashMap<(String, String), CountryPair> = HashMap::new();
    let mut graph_list : HashMap<String, Vec<Outedge>> = HashMap::new();
    let county_map = counties_map();
    for result in rdr.expect("Something failed").deserialize(){ 
        let record: Record = result.expect("Something failed");
        if record.scaled_sci > cutoff{
       // println!("{:?}", record.user_loc);
        let level = match county_map.get(&record.user_loc){ // please god let me refactor this
            None => "None",
            Some(val) => {val}
        };
        let level2 = match county_map.get(&record.fr_loc){
            None => "None",
            Some(val) => {val}
        };
        let level_slice : &str = match level {
            "gadm1" => &record.user_loc[0..3],
            "nuts3" => &record.user_loc[0..2],
            &_ => &record.user_loc
        };
        let level2_slice : &str = match level2 {
            "gadm1" => &record.fr_loc[0..3],
            "nuts3" => &record.fr_loc[0..2],
            &_ => &record.fr_loc
        };
        match counts_map.get(&(String::from(level_slice), String::from(level2_slice))) {
            None => {
                counts_map.entry((String::from(level2_slice), String::from(level_slice))).or_insert(CountryPair{count: 0, distance: 0}).count += 1;
                counts_map.entry((String::from(level2_slice), String::from(level_slice))).or_insert(CountryPair{count: 0, distance: 0}).distance += record.scaled_sci;
            }
            Some(_val) => {
                counts_map.entry((String::from(level_slice), String::from(level2_slice))).or_insert(CountryPair{count: 0, distance: 0}).count += 1;
                counts_map.entry((String::from(level_slice), String::from(level2_slice))).or_insert(CountryPair{count: 0, distance: 0}).distance += record.scaled_sci;

            }

        }
        
    }
        //println!("{:?}", level);
        //println!("{:?} This is a location", record.user_loc);

        // if record.scaled_sci > cutoff{
        //     graph_list.entry(record.user_loc).or_insert(Vec::new()).push(Outedge{vertex: record.fr_loc, length: record.scaled_sci});
        // }
            //graph_list[&record.user_loc].push((record.fr_loc, record.scaled_sci));
    }
    let mut counter : usize = 0;
    for (key, val) in counts_map.iter(){
        counter += 1;
        println!("{:?}, {:?}", key, counter);
        let pair : &CountryPair = val;
        let true_distance : usize = (*pair).distance / (*pair).count; // I know this is integer division but it shouldn't lose much precision and I feel like converting to floats and back would add a fair amount of operations
        let vertex1 = &key.0;
        let vertex2 = &key.1;
        graph_list.entry(String::from(vertex1)).or_insert(Vec::new()).push(Outedge{vertex: String::from(vertex2), length: true_distance});
    }   
   // println!("{:?}", graph_list);
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
            println!("!---!");
            println!("The while loop is at the top, the some is {:?}, {:?}", dist, v);
            println!("{:?}", pq);
        }
        match map.get(v) {
            None => {break},
            Some(edges) => {
                for Outedge{vertex,length} in edges.iter() {
                    if rng == 0{
                        println!("{:?}, {:?}", vertex, v);
                    }
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
        if rng == 0{
          //  println!("!---!");
          //  println!("The while loop is at the top, the some is {:?}, {:?}", dist, v);
            println!("{:?}", pq);
            println!("{:?}", distances);
        }

    };
   // println!("{:?}", distances);

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
   // println!("Hello, world!");
    let adjacency_map : HashMap<String, Vec<Outedge>> = read_to_map_aggregate("test_new.tsv", 10000 as usize);
    // let adjacency_map : HashMap<String, Vec<Outedge>> = read_to_map_aggregate("data/data.tsv", 10000 as usize);

    //let adjacency_map : HashMap<String, Vec<Outedge>> = read_to_map("data/data.tsv", 100000 as usize);
    for i in adjacency_map.keys(){
       // println!("{:?}", i);
        shortest_paths(&adjacency_map, String::from(i));
    }
    let counties_map = counties_map();
    //println!("{:?}", counties_map);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
