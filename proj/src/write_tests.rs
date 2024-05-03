pub mod write_tests{
    use std::fs::File;
    use std::io::prelude::*;
    use std::time::{Instant};
    use std::collections::HashMap;
    use std::fs::OpenOptions;
    fn write_line(file: &mut File, s: String){
        file.write_all(s.as_bytes()).expect("Unable to write file"); 
        
    }
    fn write_test_clique(){
        let write_path: &str = "tests/test_clique.tsv";
        let _file = File::create(&write_path).expect("Unable to create file");
        let mut file = OpenOptions::new()
        .append(true)
        .open(&write_path)
        .expect("cannot open file");
        write_line(&mut file, format!("{0}\t{1}\t{2}\n", String::from("user_loc"), String::from("fr_loc"),String::from("scaled_sci")));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","1", "2",3));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","2", "4",3));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","4", "3",3));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","3", "1",3));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","2", "3",1));    
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","4", "1",1));
    }
    
    fn write_test_oneside(){
        let write_path: &str = "tests/test_oneside.tsv";
        let _file = File::create(&write_path).expect("Unable to create file");
        let mut file = OpenOptions::new()
        .append(true)
        .open(&write_path)
        .expect("cannot open file");
        write_line(&mut file, format!("{0}\t{1}\t{2}\n", String::from("user_loc"), String::from("fr_loc"),String::from("scaled_sci")));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","1", "2",2));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","2", "3",1));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","3", "4",2));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","4", "5",2));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","5", "1",2));    
    }
    
    fn write_test_niner(){
        let write_path: &str = "tests/test_niner.tsv";
        let _file = File::create(&write_path).expect("Unable to create file");
        let mut file = OpenOptions::new()
        .append(true)
        .open(&write_path)
        .expect("cannot open file");
        write_line(&mut file, format!("{0}\t{1}\t{2}\n", String::from("user_loc"), String::from("fr_loc"),String::from("scaled_sci")));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","1", "2",2));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","1", "4",1));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","1", "5",10));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","2", "3",5));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","2", "5",1));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","3", "5",10));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","3", "6",5));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","4", "5",5));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","4", "7",1));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","5", "6",2));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","5", "7",10));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","5", "9",1));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","6", "9",5));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","7", "8",1));
    }
    
    fn write_test_straight(){
        let write_path: &str = "tests/test_straight.tsv";
        let _file = File::create(&write_path).expect("Unable to create file");
        let mut file = OpenOptions::new()
        .append(true)
        .open(&write_path)
        .expect("cannot open file");
        write_line(&mut file, format!("{0}\t{1}\t{2}\n", String::from("user_loc"), String::from("fr_loc"),String::from("scaled_sci")));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","1", "2",4));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","2", "3",5));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","3", "4",6));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","4", "5",7));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","5", "6",8));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","6", "7",9));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","7", "8",10));
    }


    
    fn write_test_components(){
        let write_path: &str = "tests/test_components.tsv";
        let _file = File::create(&write_path).expect("Unable to create file");
        let mut file = OpenOptions::new()
        .append(true)
        .open(&write_path)
        .expect("cannot open file");
        write_line(&mut file, format!("{0}\t{1}\t{2}\n", String::from("user_loc"), String::from("fr_loc"),String::from("scaled_sci")));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","1", "2",1));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","1", "3",1));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","4", "5",1));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","5", "6",1));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","7", "8",1));
    }

    fn write_test_two_components(){
        let write_path: &str = "tests/test_two_components.tsv";
        let _file = File::create(&write_path).expect("Unable to create file");
        let mut file = OpenOptions::new()
        .append(true)
        .open(&write_path)
        .expect("cannot open file");
        write_line(&mut file, format!("{0}\t{1}\t{2}\n", String::from("user_loc"), String::from("fr_loc"),String::from("scaled_sci")));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","1", "2",1));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","2", "3",1));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","3", "4",1));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","4", "5",1));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","6", "7",1));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","7", "8",1));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","6", "9",1));
        write_line(&mut file, format!("{0}\t{1}\t{2}\n","9", "10",1));

    }


    pub fn write_all_tests() {
        write_test_clique();
        write_test_oneside();
        write_test_niner();
        write_test_straight();
        write_test_components();
        write_test_two_components();
    }
    

}