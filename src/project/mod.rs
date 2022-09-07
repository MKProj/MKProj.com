use lazy_static::lazy_static;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use toml::{from_str, to_string_pretty};
use std::fs::read_to_string;
lazy_static!{   
    pub static ref PROJECTS: Mutex<Vec<Project>> = Mutex::new(Vec::new());
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project{
    pub name: String, 
    pub license: String, 
    pub curr_vers: String, 
    pub next_vers: Option<String>, 
    pub repo: String, 
    pub documentation: String, 
    pub crates_io: String, 
    pub description: String, 
}

impl Project{
    pub fn from_file(path: &str) -> Self{
        let string = read_to_string(path).unwrap();
        from_str(&string).unwrap()
    }
    pub fn to_string(&self) -> String{
        to_string_pretty(&self).unwrap()
    }
    pub fn to_html(&self) -> String{
        todo!("Will do once we figure out template")
    }
}

