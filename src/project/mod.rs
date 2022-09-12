use lazy_static::lazy_static;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::fs::read_to_string;
use std::ops::Deref;
use std::path::PathBuf;
lazy_static!{   
    pub static ref PROJECTS: Mutex<Vec<Project>> = Mutex::new(Vec::new());

}
const BASE: &str = r#"<div class="w3-container w3-{color} w3-center" style="margin-left: 50px; margin-right: 50px;">
    <h3>{name}</h3>
    <h5><a href="{repo}">Repo</a> | <a href="{doc}">Documentation</a> | <a href="{crate}">Crates.io</a></h5>
    <h5>Current Version: {curr} </h5>
    <h5>Planned Next Version: {next}</h5>
    <p class="w3-large">{description}</p>
</div>
    "#;
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
    pub color: String, 
}

impl Project{
    pub fn from_file(path: PathBuf) -> Self{
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
        base = base.replace("{color}", &self.color);
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
    let mut projects = PROJECTS.lock().unwrap();
    // clear projects
    projects.clear();
    for entry in dir{
        let entry = entry?;
        let project = Project::from_file(entry.path()) ;
        projects.push(project);
    }
    Ok(())
}

pub fn get_base() -> String{
    read_to_string("templates/base_proj.html").unwrap()
}

pub fn gen_html() -> String{
    get_projects().unwrap();
    let mut html = Vec::new();
    let projects = PROJECTS.lock().unwrap();
    let base = get_base();
    for p in projects.deref(){
        html.push(p.to_html())
    }
    base.replace("{projects}", &html.join("\n"))
}

