use std::fs::File;
use std::io::prelude::*;
use std::time::{Instant};
use std::collections::HashMap;
use std::fs::OpenOptions;
mod data_cleaning;
use crate::data_cleaning::data_cleaning::run_cleaner;
mod data_reading;
use crate::data_reading::data_reading::{Record, CountryPair};
mod cc;
use crate::cc::cc::verify_connected_components;
type Vertex = String;
type Distance = usize;
type Edge = (Vertex, Vertex, Distance);

/// Holds graph information necessary for minimum spanning trees
#[derive(Debug)]
struct Graph {
    n: usize,
    edges: Vec<Edge>,
    parent: Vec<usize>,
    rank: Vec<usize>,
    vertex_to_num_map: HashMap<String, usize>
}

/// Returns an adjacency list representationn of the graph, of vectors with indices determined by vertex_to_num_map
impl Graph {
    fn adjacency_list(&self) -> Vec<Vec<usize>>{
        let mut graph_list : Vec<Vec<usize>> = vec![vec![];self.n];
        for (v,w,_) in self.edges.iter() {
            match self.vertex_to_num_map.get(v){
                Some(vv) =>{
                    match self.vertex_to_num_map.get(w){
                        Some (ww) => {
                            graph_list[*vv].push(*ww);
                            graph_list[*ww].push(*vv);
                        },
                        None => {
                        }
                    }
                },
                None => {
                }
            }

        };
        return graph_list;
    }
    /// Initializes the graph specifically for Kruskal MST, sorting the edge list so it decreases instead of increases, so that the minimum spanning tree actually maximizes the connectivity rather than minimizing distance
    fn create_undirected(path: &str) -> Graph {
        let vec_tuple : (Vec<Edge>, HashMap<String, usize>, usize) = read_to_vec_aggregate(path);
        let edges : Vec<Edge> = vec_tuple.0;
        let n : usize = vec_tuple.2;
        let vertex_to_num_map : HashMap<String, usize> = vec_tuple.1;
        let parent: Vec<usize> = vec![];
        let rank: Vec<usize> = vec![];
        let mut g = Graph{n,edges,parent,rank, vertex_to_num_map};
        g.edges.sort_by(|a, b| b.2.cmp(&a.2));
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
    /// The minimum spanning tree algorithm: Constructs the minimum number of edges (number of vertices - 1), while maximizing the amount of connectedness. It goes down the list of (already sorted) edges, and connects two nodes if they are not already connected
    fn kruskal_mst(&mut self) -> Vec<Edge> {
        let adjacency_list : Vec<Vec<usize>> = self.adjacency_list();
        let components_num: usize = verify_connected_components(self.n, adjacency_list);
        if components_num != 1{
            panic!("There is not exactly 1 component in this graph, so minimum spanning trees is impossible!");
        }
        println!("There is exactly 1 component, so we proceed with MST");
        let mut result: Vec<Edge> = vec![];
        let mut num_mst_e = 0;
        let mut next_edge = 0;
        let vec_map = self.vertex_to_num_map.clone();
        while num_mst_e < self.n - 1 {
            let (u,v,w) = &self.edges[next_edge];
            match vec_map.get(v){
                None => {
                    next_edge = next_edge + 1;
                    println!("problem AT {:?}", v);
                },
                Some(v_numeric) => {
                    let u_numeric = vec_map.get(u).unwrap();
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
        }
        result
    }
}
/// Reads in a given dataset, similarly to clean_data_counts in data_cleaning module, storing the connectivity between two vertices in a hashmaoo
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

/// Converts the hashmap generated in read_to_counts into a vector of edges where the original vertices, which are strings, map to integers. Returns the vector and the hashmap mapping the strings to integers
fn counts_to_vector(counts_map: HashMap<(String, String), CountryPair>) -> (Vec<Edge>, HashMap<String, usize>, usize){
    let mut vertex_to_num_map : HashMap<String, usize> = HashMap::new();
    let mut graph_list : Vec<Edge> = Vec::new();
    let mut counter : usize = 0; // counts number of edges
    for (key, val) in counts_map.iter(){ // HashMap so this goes in a random order
        let pair : &CountryPair = val;
        let true_distance : usize = (*pair).distance / (*pair).count; // Being integer division, this computation loses a little precision, but not enough (IMO) to warrant changing to float and back
        let vertex1 = &key.0;
        let vertex2 = &key.1;
        if let None = vertex_to_num_map.get(vertex1){
            vertex_to_num_map.insert(String::from(vertex1), counter);
            counter += 1;
        }
        if let None = vertex_to_num_map.get(vertex2){
            vertex_to_num_map.insert(String::from(vertex2), counter);
            counter += 1;
        }
        graph_list.push((String::from(vertex1), String::from(vertex2), true_distance));
    
    }
    return (graph_list, vertex_to_num_map, counter);
}

/// Given a dataset path, returns the result of counts_to_vector for that dataset by calling read_to_counts first
fn read_to_vec_aggregate(path: &str) ->  (Vec<Edge>, HashMap<String, usize>, usize){
    let counts_map : HashMap<(String, String), CountryPair> = read_to_counts(path);
    let tup :  (Vec<Edge>, HashMap<String, usize>, usize) = counts_to_vector(counts_map);
    return tup;
}

/// Given a dataset path, returns a tuple with the number connected components and total connectivity sum generated by the minimum spanning tree
fn run_test(test_path: &str) -> (usize, usize){
    let mut g : Graph = Graph::create_undirected(test_path);
    let adjacency_list : Vec<Vec<usize>> = g.adjacency_list();
    let components_count : usize = verify_connected_components(g.n, adjacency_list);
    let mut post_counter : usize = 0;
    let mst : Vec<Edge> = g.kruskal_mst();
     for i in &mst{
         post_counter += i.2;
     }
     return (components_count, post_counter);
}

#[test]
fn test_components(){
    let mut g : Graph = Graph::create_undirected("tests/test_components.tsv");
    let adjacency_list : Vec<Vec<usize>> = g.adjacency_list();
    println!("{:?}", adjacency_list);
    let components_count : usize = verify_connected_components(g.n, adjacency_list);
    assert_eq!(components_count, 3)
}

#[test]
fn test_two_components(){
    let mut g : Graph = Graph::create_undirected("tests/test_two_components.tsv");
    let adjacency_list : Vec<Vec<usize>> = g.adjacency_list();
    println!("{:?}", adjacency_list);
    let components_count : usize = verify_connected_components(g.n, adjacency_list);
    assert_eq!(components_count, 2)
}

#[test]
fn test_one_component(){
    let mut g : Graph = Graph::create_undirected("tests/test_one_component.tsv");
    let adjacency_list : Vec<Vec<usize>> = g.adjacency_list();
    println!("{:?}", adjacency_list);
    let components_count : usize = verify_connected_components(g.n, adjacency_list);
    assert_eq!(components_count, 1)
}
#[test]
fn test_many_components(){
    let mut g : Graph = Graph::create_undirected("tests/test_many_components.tsv");
    let adjacency_list : Vec<Vec<usize>> = g.adjacency_list();
    println!("{:?}", adjacency_list);
    let components_count : usize = verify_connected_components(g.n, adjacency_list);
    assert_eq!(components_count, 9)
}


#[test]
fn test_clique(){
    let test_result : (usize, usize) = run_test("tests/test_clique.tsv");
    assert_eq!(test_result, (1, 9));
}

#[test]
fn test_oneside(){
    let test_result : (usize, usize) = run_test("tests/test_oneside.tsv");
    assert_eq!(test_result, (1, 8));
}

#[test]
fn test_niner(){
    let test_result : (usize, usize) = run_test("tests/test_niner.tsv");
    assert_eq!(test_result, (1, 51));}

#[test]
fn test_straight(){
    let test_result : (usize, usize) = run_test("tests/test_straight.tsv");
    assert_eq!(test_result, (1, 49));}
    
#[test]
fn test_hanger(){
    let test_result : (usize, usize) = run_test("tests/test_hanger.tsv");
    assert_eq!(test_result, (1, 25));}
      
    




fn main() {
    let start = Instant::now();
    println!("Creating undirected...");
    let cleaned_path : &str = "data/cleaned.tsv";
    let mut g : Graph = Graph::create_undirected(cleaned_path);
    let mut pre_counter : usize = 0;
    for i in &g.edges{
        pre_counter += i.2;
    }
    let mut post_counter : usize = 0;
    println!("MST going...");
    let path = String::from("output_MST.tsv");
    let _file = File::create(&path).expect("Unable to create file");
    let mut file = OpenOptions::new()
         .append(true)
         .open(&path)
         .expect("cannot open file");
    let mst : Vec<Edge> = g.kruskal_mst();
     for i in &mst{
         let s: String = format!("{0}\t{1}\t{2}\n", i.0, i.1, i.2);
         post_counter += i.2;
         file.write_all(s.as_bytes()).expect("Unable to write file");        
     }
    println!("\nResults\n------------------");
    println!("Before MST: Total connectedness = {:?} on {:?} edges\nAfter MST: Total connectedness = {:?} on {:?} edges\n{:.2}% of connectedness maintained despite only keeping {:.3}% of connections", pre_counter, g.edges.len(), post_counter, mst.len(), post_counter as f64/pre_counter as f64 * 100.0, 100.0 * mst.len() as f64/ g.edges.len() as f64);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
