use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use rand::Rng;
use std::fs::OpenOptions;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
pub struct Record {
    // for reading in edges with serde
    user_loc:String,
    fr_loc:String,
    scaled_sci:usize
}

#[derive(Debug,Clone)]
struct Outedge {
    vertex: Vertex,
    length: Distance,
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

type Edge = (Vertex, Vertex, Distance);

type Vertex = String;
type Distance = usize;

#[derive(Debug)]
struct Graph {
    n: usize,
    outedges: Vec<Edge>,
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl Graph {
    fn create_undirected(n:usize,outedges:Vec<Edge>) -> Graph {
        let parent: Vec<usize> = vec![];
        let rank: Vec<usize> = vec![];
        let mut g = Graph{n,outedges,parent,rank};
        g.outedges.sort_by(|a, b| b.2.cmp(&a.2));
        for node in 0..g.n {
            g.parent.push(node);
            g.rank.push(0);
        }
        g
    }
    fn find(&mut self, i:usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        return self.parent[i];
    }
    fn union(&mut self, i:usize, j:usize) {
        if self.rank[i] < self.rank[j] {
            self.parent[i] = j;
        } else if self.rank[i] > self.rank[j] {
            self.parent[j] = i;
        } else {
            self.parent[j] = i;
            self.rank[i] += 1;
        }
    }
    fn KruskalMST(&mut self, vec_to_num_map: HashMap<String, usize>) -> Vec<Edge> {
        let mut result: Vec<Edge> = vec![];
        let mut num_mst_e = 0;
        let mut next_edge = 0;
        while num_mst_e < self.n - 1 {
            let (u,v,w) = &self.outedges[next_edge];
           // println!("{:?}", u);
           // println!("{:?}", u);
            match vec_to_num_map.get(v){
                None => {
                    //num_mst_e += 1;
                    next_edge = next_edge + 1;
                    println!("MONENONENONENONE AT {:?}", v);
                },
                Some(v_numeric) => {
                    println!("{:?} out of {:?}", num_mst_e, self.n);
                    let u_numeric = vec_to_num_map.get(u).unwrap();
                    let new_w : usize = *w;
                    let new_u : String = String::from(u);
                    let new_v : String = String::from(v);
                    next_edge = next_edge + 1;
                    
                    let x = self.find(*u_numeric);
                    let y = self.find(*v_numeric);
                    if x != y {
                        num_mst_e += 1;
                        result.push((new_u, new_v, new_w));
                        self.union(x,y);
                    }
                }
            }
            //let u_numeric = vec_to_num_map.get(u).unwrap();

        }
        result
    }
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


fn read_to_counts(path: &str, county_map: HashMap<String, String>) -> HashMap<(String, String), CountryPair>{
    let rdr = csv::ReaderBuilder::new()
    .delimiter(b'\t')
    .has_headers(true)
    .flexible(true)
    .from_path(path);
    let mut counts_map : HashMap<(String, String), CountryPair> = HashMap::new();
    for result in rdr.expect("Something failed").deserialize(){ 
        let record: Record = result.expect("Something failed");
        println!("{:?}", record.user_loc);
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
            "gadm2" => &record.user_loc[0..3],
            "nuts3" => &record.user_loc[0..2],
            &_ => &record.user_loc
        };
        let level2_slice : &str = match level2 {
            "gadm1" => &record.fr_loc[0..3],
            "gadm2" => &record.user_loc[0..3],
            "nuts3" => &record.fr_loc[0..2],
            &_ => &record.fr_loc
        };
        if String::from(level_slice) != String::from(level2_slice){
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
    }
    return counts_map;
    
}

// fn read_to_counts_updated(path: &str, county_map: HashMap<String, String>) -> HashMap<(String, String), CountryPair>{
//     let rdr = csv::ReaderBuilder::new()
//     .delimiter(b'\t')
//     .has_headers(true)
//     .flexible(true)
//     .from_path(path);
//     let mut counts_map : HashMap<(String, String), CountryPair> = HashMap::new();
// }



fn counts_to_hashmap(counts_map: HashMap<(String, String), CountryPair>) -> HashMap<String, Vec<Outedge>>{
    let mut graph_list : HashMap<String, Vec<Outedge>> = HashMap::new();
    for (key, val) in counts_map.iter(){
        let pair : &CountryPair = val;
        let true_distance : usize = (*pair).distance / (*pair).count; // I know this is integer division but it shouldn't lose much precision and I feel like converting to floats and back would add a fair amount of operations
        let vertex1 = &key.0;
        let vertex2 = &key.1;
        graph_list.entry(String::from(vertex1)).or_insert(Vec::new()).push(Outedge{vertex: String::from(vertex2), length: true_distance});    
    }   
    return graph_list
}

fn counts_to_vector(counts_map: HashMap<(String, String), CountryPair>) -> (Vec<Edge>, HashMap<String, usize>, HashMap<usize, String>, usize){
    let mut vec_to_num_map : HashMap<String, usize> = HashMap::new();
    let mut num_to_vec_map : HashMap<usize, String> = HashMap::new();
    let mut graph_list : Vec<Edge> = Vec::new();
    let mut counter : usize = 0;
    for (key, val) in counts_map.iter(){
        let pair : &CountryPair = val;
        let true_distance : usize = (*pair).distance / (*pair).count; // I know this is integer division but it shouldn't lose much precision and I feel like converting to floats and back would add a fair amount of operations
        let vertex1 = &key.0;
        let vertex2 = &key.1;
        if let None = vec_to_num_map.get(vertex1){
            vec_to_num_map.insert(String::from(vertex1), counter);
            num_to_vec_map.insert(counter, String::from(vertex1));
            counter += 1;

        }
        graph_list.push((String::from(vertex1), String::from(vertex2), true_distance));
        
    }   
    return (graph_list, vec_to_num_map, num_to_vec_map, counter);
}


fn read_to_map_aggregate(path: &str) ->  HashMap<String, Vec<Outedge>>{
    let counts_map : HashMap<(String, String), CountryPair> = read_to_counts(path, counties_map());
    return counts_to_hashmap(counts_map);
}

fn read_to_vec_aggregate(path: &str) ->  (Vec<Edge>, HashMap<String, usize>, HashMap<usize, String>, usize){
    let counts_map : HashMap<(String, String), CountryPair> = read_to_counts(path, counties_map());
    let tup :  (Vec<Edge>, HashMap<String, usize>, HashMap<usize, String>, usize) = counts_to_vector(counts_map);
    println!("{:?}", tup.3);
    return tup;
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
  //  let adjacency_map : HashMap<String, Vec<Outedge>> = read_to_map_aggregate("test_new.tsv");
    let vec_tuple : (Vec<Edge>, HashMap<String, usize>, HashMap<usize, String>, usize) = read_to_vec_aggregate("data/copy-updated.tsv");
    println!("Creating undirected");
    let mut g : Graph = Graph::create_undirected(vec_tuple.3, vec_tuple.0);
    println!("MST going");
    let path = String::from("output_MST.tsv");
    let mut file = File::create(&path).expect("Unable to create file");
    let mut file = OpenOptions::new()
        .append(true)
        .open(&path)
        .expect("cannot open file");
    let mst : Vec<Edge> = g.KruskalMST(vec_tuple.1);
    for i in &mst{
        println!("{:?} - {:?} -> {:?}", i.0, i.1, i.2);
        let s: String = format!("{0}\t{1}\t{2}\n", i.0, i.1, i.2);
        file.write_all(s.as_bytes()).expect("Unable to write file");        
    }
    // println!("MST result: {:?}", mst);
    let counties_map = counties_map();
    //println!("{:?}", counties_map);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
