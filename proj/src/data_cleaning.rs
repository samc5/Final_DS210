pub mod data_cleaning{
    use std::fs::File;
    use std::io::prelude::*;
    use serde::Deserialize;
    use std::collections::HashMap;
    use rand::Rng;
    use std::fs::OpenOptions;
    // mod data_reading;
    use crate::data_reading::data_reading::{Record, CountryPair};

    #[derive(Deserialize)]
    struct CountyRecord{
        key: String,
        level: String
    }


    pub fn write_test(){
        // DO NOT USE
        // Generates a file for a test, 
        let path = String::from("test_new.tsv");
        let csv_path = String::from("data/data.tsv");
        let mut _file = File::create(&path).expect("Unable to create file");
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
    
    fn clean_data_counts(county_map: HashMap<String, String>) -> HashMap<(String, String), CountryPair>{
        let read_path: &str = "data/data.tsv";
        let rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .flexible(true)
        .from_path(read_path);
        let mut counts_map : HashMap<(String, String), CountryPair> = HashMap::new();
        let mut print_counter: usize = 0;
        for result in rdr.expect("CSV reading failed").deserialize(){
            let record: Record = result.expect("Something failed");
            if print_counter % 100000 == 0{
                println!("{:?}", record.user_loc);
            }
            print_counter += 1;
            let user_level = match county_map.get(&record.user_loc){ // please god let me refactor this
                None => "None",
                Some(val) => {val}
            };
            let fr_level = match county_map.get(&record.fr_loc){
                None => "None",
                Some(val) => {val}
            };
            let user_slice : &str = match user_level {
                "gadm1" => &record.user_loc[0..3],
                "gadm2" => &record.user_loc[0..3],
                "nuts3" => &record.user_loc[0..2],
                &_ => &record.user_loc
            };
            let fr_slice : &str = match fr_level {
                "gadm1" => &record.fr_loc[0..3],
                "gadm2" => &record.fr_loc[0..3],
                "nuts3" => &record.fr_loc[0..2],
                &_ => &record.fr_loc
            };
            if String::from(user_slice) != String::from(fr_slice){
                match counts_map.get(&(String::from(user_slice), String::from(fr_slice))) {
                    None => {
                        counts_map.entry((String::from(fr_slice), String::from(user_slice))).or_insert(CountryPair{count: 0, distance: 0}).count += 1;
                        counts_map.entry((String::from(fr_slice), String::from(user_slice))).or_insert(CountryPair{count: 0, distance: 0}).distance += record.scaled_sci;
                    }
                    Some(_val) => {
                        counts_map.entry((String::from(user_slice), String::from(fr_slice))).or_insert(CountryPair{count: 0, distance: 0}).count += 1;
                        counts_map.entry((String::from(user_slice), String::from(fr_slice))).or_insert(CountryPair{count: 0, distance: 0}).distance += record.scaled_sci;
                    }
                }
        
            }
        }
        return counts_map;
    }
    
    fn write_cleaned_data(counts_map: HashMap<(String, String), CountryPair>){
        let write_path: &str = "data/cleaned.tsv";
        let _file = File::create(&write_path).expect("Unable to create file");
        let mut file = OpenOptions::new()
            .append(true)
            .open(&write_path)
            .expect("cannot open file");
        let s: String = format!("{0}\t{1}\t{2}\n", String::from("user_loc"), String::from("fr_loc"),String::from("scaled_sci"));
        file.write_all(s.as_bytes()).expect("Unable to write file");   
        let mut print_counter: usize = 0;
        for (key, val) in counts_map.iter(){
            if print_counter % 100000 == 0{
                println!("{:?}", key);
            }
            print_counter += 1;
            let pair : &CountryPair = val;
            let true_distance : usize = (*pair).distance / (*pair).count; // I know this is integer division but it shouldn't lose much precision and I feel like converting to floats and back would add a fair amount of operations
            let vertex1 = &key.0;
            let vertex2 = &key.1;
            let s: String = format!("{0}\t{1}\t{2}\n", String::from(vertex1), String::from(vertex2), true_distance);
            file.write_all(s.as_bytes()).expect("Unable to write file");   
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
    pub fn run_cleaner(){
        let counties_map = counties_map();
        let count : HashMap<(String, String), CountryPair> = clean_data_counts(counties_map);
        write_cleaned_data(count); 
    }
}