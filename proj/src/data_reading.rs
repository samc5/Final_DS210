pub mod data_reading {
    use serde::Deserialize;
    #[derive(Debug,Clone)]
    pub struct CountryPair {
        pub count: usize,
        pub distance: usize,
    }
    #[derive(Debug, Deserialize)]
    pub struct Record {
    // for reading in edges with serde
        pub user_loc:String,
        pub fr_loc:String,
        pub scaled_sci:usize
    }
}