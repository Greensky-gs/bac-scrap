use crate::result::Resultat;
use crate::tools::{fetch, wait, get_url_for};

use rand::{Rng, rng};
use rand::rngs::ThreadRng;
use std::fs;
use std::path::{Path, PathBuf};

const LETTERS: u8 = 25;

pub struct Metadata {
    errors: Vec<String>,
    results: Vec<Resultat>,
    indexes: [u8; 4],
    errors_path: PathBuf,
    res_path: PathBuf,
    ind_path: PathBuf,
    rng: ThreadRng,
    
    pub errors_path_str: String,
    pub res_path_str: String,
    pub ind_path_str: String
}

impl Metadata {
    pub fn new(errors_path_str: &String, res_path_str: &String, ind_path_str: &String) -> Metadata {
        let errs_path = Path::new(errors_path_str);
        let res_path = Path::new(res_path_str);
        let ind_path = Path::new(ind_path_str);

        if errs_path.exists() && errs_path.metadata().unwrap().is_dir() {
            panic!("Errors path cannot be a directory");
        }
        if res_path.exists() && res_path.metadata().unwrap().is_dir() {
            panic!("Results path cannot be a directory");
        }
        if ind_path.exists() && ind_path.metadata().unwrap().is_dir() {
            panic!("Indexes path cannot be a directory");
        }

        let mut structure = Metadata {
            errors: vec![],
            results: vec![],
            indexes: [0,0,0,0],
            errors_path_str: errors_path_str.clone(),
            res_path_str: res_path_str.clone(),
            ind_path_str: ind_path_str.clone(),
            ind_path: ind_path.to_path_buf(),
            errors_path: errs_path.to_path_buf(),
            res_path: res_path.to_path_buf(),
            rng: rng()
        };

        structure.load_errors();
        structure.load_results();
        structure.load_indexes();

        return structure;
    }
    
    pub fn load_errors(&mut self) {
        if !self.errors_path.exists() {
            return;
        }
        let content = fs::read_to_string(self.errors_path.clone()).expect("File should exist");
        for err in content.split(",") {
            if err.len() <= 0 {
                continue;
            }
            self.errors.push(err.to_string());
        }
    }
    pub fn load_results(&mut self) {
        if !self.res_path.exists() {
            return;
        }

        let content = fs::read_to_string(self.res_path.clone()).expect("File should exist");

        let results: Vec<Resultat> = serde_json::from_str(&content).unwrap();
        for res in results {
            self.results.push(res);
        }
    }
    pub fn load_indexes(&mut self) {
        if !self.ind_path.exists() {
            return;
        }
        let content = fs::read_to_string(self.ind_path.clone()).expect("Cannot read file");
        let mut i = 0;
        for number in content.split(",") {
            let value = number.trim().parse::<u8>().expect("Cannot parse number");
            self.indexes[i] = value;
            i+=1;
        }
    }

    pub fn save_errors(&self) {
        let _ = fs::write(&self.errors_path, self.errors.join(",")).expect("Cannot write to the errors path");
    }
    pub fn save_results(&self) {
       let content = serde_json::to_string(&self.results).expect("Shoudn't be unable to serialize");
       let _ = fs::write(&self.res_path, content);
    }
    pub fn save_indexes(&self) {
        let _ = fs::write(&self.ind_path, self.indexes.map(|x| x.to_string()).join(",")).expect("Cannot write to file");
    }
    fn handle_name(&mut self, name: &String) {
        println!("Fetching \x1b[91m{}\x1b[0m", name);

        let url = get_url_for(name);
        let response = fetch(&url);

        match response {
            None => {
                println!("\x1b[31mFAILED\x1b[0m to fetch \x1b[91m{}\x1b[0m", name);
                self.errors.push(name.to_string());

                println!("\x1b[33mSecurity waiting\x1b[0m of \x1b[93m3000ms\x1b[0m...");
                self.wait(3000, 3001);
            },
            Some(values) => {
                println!("\x1b[92mFETCHED\x1b[0m \x1b[91m{}\x1b[0m, got \x1b[93m{}\x1b[0m results", name, values.len());

                for res in values {
                    self.results.push(res);
                }
            }
        }
    }
    fn wait(&mut self, min: u16, max: u16) {
        let val = self.rng.random_range(min..max);
        println!("Waiting \x1b[90m{}ms\x1b[0m", val);
        wait(val);
    }
    pub fn start_fetch(&mut self) { 
        while self.indexes[0] <= LETTERS || self.errors.len() > 0 {
            if self.errors.len() > 0 && self.indexes[0] > LETTERS {
                let name = self.errors.pop().unwrap();

                self.handle_name(&name);
                
                self.save_errors();
                self.save_results();

                self.wait(2, 10);

                continue;
            } else {
                while self.indexes[1] <= LETTERS {
                    let name = format!("{}{}", (self.indexes[0] + 97) as char, (97 + self.indexes[1]) as char);

                    self.handle_name(&name);

                    self.wait(2, 10);
                    while self.indexes[2] <= LETTERS {
                        while self.indexes[3] <= LETTERS {
                            let name: String = vec![0,1,2,3].iter().map(|x| (97 + self.indexes[*x as usize]) as char).collect();

                            self.handle_name(&name);

                            self.indexes[3] += 1;
                            self.wait(2, 10);
                        }
                        self.indexes[3] = 0;
                        self.indexes[2] += 1;
                    }
                    self.indexes[2] = 0;
                    self.indexes[3] = 0;
                    self.indexes[1] += 1;
                }
                self.indexes[0] += 1;
                self.indexes[1] = 0;
                self.indexes[2] = 0;
                self.indexes[3] = 0;

                println!("Finished letter \x1b[91m{}\x1b[0m. Saving and going next...", (97 + self.indexes[0] - 1) as char);

                self.save_errors();
                self.save_results();
                self.save_indexes();

                self.wait(1500, 2000);
           }
        }

        self.save_errors();
        self.save_results();
        self.save_indexes();
        println!("\x1b[32mSuccesfully fetched \x1b[93m{}\x1b[32mresults\x1b[0m", self.results.len());
    }
}
