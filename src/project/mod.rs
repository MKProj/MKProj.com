use lazy_static::lazy_static;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use toml::{from_str, to_string_pretty};
use std::fs::read_to_string;
use std::path::PathBuf;
lazy_static!{   
    pub static ref PROJECTS: Mutex<Vec<Project>> = Mutex::new(Vec::new());
    static ref BASE: &str = r#"<div class="w3-container w3-blue w3-center" style="margin-left: 50px; margin-right: 50px;">
    <h3>{name}</h3>
    <h5><a href="{repo}">Repo</a> | <a href="{doc}">Documentation</a> | <a href="{crate}">Crates.io</a></h5>
    <h5>Current Version: {curr} </h5>
    <h5>Planned Next Version: {next}</h5>
    <p class="w3-large">{description}</p>
</div>
    "#;
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
        let mut base = BASE.to_string();
        base = base.replace("{name}", &self.name);
        base = base.replace("{repo}", &self.repo);
        base = base.replace("{doc}", &self.documentation);
        base = base.replace("{crate}", &self.crates_io);
        base = base.replace("{curr}", &self.curr_vers);
        match &self.next_vers{
            Some(v) => {
                base = base.replace("{next}", &v);
            }, 
            None => {
                let next = "<h5>Planned Next Version: {next}</h5>";
                base = base.replace(next, "");
            }
        }
        base = base.replace("{description}", &self.description);
        base 
    }
}


pub fn get_projects() -> Result<(), std::io::Error>{
    let path = PathBuf::from("json");
    let dir = std::fs::read_dir(path)?;
    for entry in dir{
        
    }
}



