use std::fs::File;
use std::io::prelude::*;
use std::time::{Instant};
use std::collections::HashMap;
use std::fs::OpenOptions;
mod data_cleaning;
//use crate::data_cleaning::data_cleaning::counties_map;
mod data_reading;
use crate::data_reading::data_reading::{Record, CountryPair};
mod cc;
use crate::cc::cc::verify_connected_components;


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
    fn adjacency_list(&self, n:usize, vec_to_num_map: &HashMap<String, usize>) -> Vec<Vec<usize>>{
        let mut graph_list : Vec<Vec<usize>> = vec![vec![];n];
        for (v,w,_) in self.outedges.iter() {
            match vec_to_num_map.get(v){
                Some(vv) =>{
                    match vec_to_num_map.get(w){
                        Some (ww) => {
                            graph_list[*vv].push(*ww);
                            graph_list[*ww].push(*vv);
                        },
                        None => {
                            println!("This is the inner match -  v {:?}, w {:?}", v, w);
                            panic!("ahhhhhh");}
                    }
                },
                None => {
                    println!("This is the outer match -  v {:?}, w {:?}", v, w);
                    panic!("ahhhhh");
                }
            }

        };
        return graph_list;
    }
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
                    next_edge = next_edge + 1;
                    println!("problem AT {:?}", v);
                },
                Some(v_numeric) => {
                    //println!("{:?} out of {:?}", num_mst_e, self.n);
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

fn read_to_counts(path: &str) -> HashMap<(String, String), CountryPair>{
    let rdr = csv::ReaderBuilder::new()
    .delimiter(b'\t')
    .has_headers(true)
    .flexible(true)
    .from_path(path);
    let mut counts_map : HashMap<(String, String), CountryPair> = HashMap::new();
    for result in rdr.expect("Something failed").deserialize(){ 
        let record: Record = result.expect("Something failed");
        if record.user_loc != record.fr_loc{
            let user : &str = &record.user_loc;
            let fr : &str = &record.fr_loc;
            match counts_map.get(&(String::from(user), String::from(fr))) {
                None => {
                    counts_map.entry((String::from(fr), String::from(user))).or_insert(CountryPair{count: 0, distance: 0}).count += 1;
                    counts_map.entry((String::from(fr), String::from(user))).or_insert(CountryPair{count: 0, distance: 0}).distance += record.scaled_sci;
                }
                Some(_val) => {
                    counts_map.entry((String::from(user), String::from(fr))).or_insert(CountryPair{count: 0, distance: 0}).count += 1;
                    counts_map.entry((String::from(user), String::from(fr))).or_insert(CountryPair{count: 0, distance: 0}).distance += record.scaled_sci;
                }
            }
    
        }
    }

    return counts_map;
    
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
        if let None = vec_to_num_map.get(vertex1){ // my hypothesis: Zimbabwe being last always comes up as vertex 2
          //  println!("{:?}", vertex1); // ok so basically, this loop stops right before Zimbabwe - I just have to figure out why
            vec_to_num_map.insert(String::from(vertex1), counter);
            num_to_vec_map.insert(counter, String::from(vertex1));
            counter += 1;
        }
        else if let None = vec_to_num_map.get(vertex2){
            vec_to_num_map.insert(String::from(vertex2), counter);
            counter += 1;
        }
        graph_list.push((String::from(vertex1), String::from(vertex2), true_distance));
    
    }
    println!("vec to num length: {:?}", vec_to_num_map.keys().len());   
    return (graph_list, vec_to_num_map, num_to_vec_map, counter);
}


fn read_to_vec_aggregate(path: &str) ->  (Vec<Edge>, HashMap<String, usize>, HashMap<usize, String>, usize){
    let counts_map : HashMap<(String, String), CountryPair> = read_to_counts(path);
    let tup :  (Vec<Edge>, HashMap<String, usize>, HashMap<usize, String>, usize) = counts_to_vector(counts_map);
    println!("{:?}", tup.3);
    return tup;
}



fn main() {
    let start = Instant::now();
    let vec_tuple : (Vec<Edge>, HashMap<String, usize>, HashMap<usize, String>, usize) = read_to_vec_aggregate("data/cleaned.tsv");
    println!("Creating undirected");
    let mut g : Graph = Graph::create_undirected(vec_tuple.3, vec_tuple.0);
    let adjacency_list : Vec<Vec<usize>> = g.adjacency_list(vec_tuple.3, &vec_tuple.1);
    verify_connected_components(vec_tuple.3, adjacency_list);
    println!("MST going");
    let path = String::from("output_MST.tsv");
    let _file = File::create(&path).expect("Unable to create file");
    let mut file = OpenOptions::new()
         .append(true)
         .open(&path)
         .expect("cannot open file");
    let mst : Vec<Edge> = g.KruskalMST(vec_tuple.1);
     for i in &mst{
         let s: String = format!("{0}\t{1}\t{2}\n", i.0, i.1, i.2);
         file.write_all(s.as_bytes()).expect("Unable to write file");        
     }
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
