use rocket::{get, launch, routes};
use std::fs::File;
use std::io::Write;
use rocket::fs::NamedFile;
use rocket::response::Redirect;
use rocket::uri;
mod index;
mod project;

use index::*;
use project::*;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[get("/")]
async fn home() -> Option<NamedFile>{
    let index_string = match base(){
        Ok(s) => s,
        Err(_) => return None
    };
    let mut file = File::create("index.html").expect("Couldn't create index.html");
    file.write_all(index_string.as_bytes()).unwrap();
    NamedFile::open("index.html").await.ok()
}

#[get("/curr_proj")]
async fn curr_proj() -> Option<NamedFile>{
    let mut file = File::create("curr_proj.html").unwrap();
    let html = gen_html();
    file.write_all(html.as_bytes()).unwrap();
    NamedFile::open("curr_proj.html").await.ok()
}

#[get("/<path>")]
async fn redirect(path: &str) -> Redirect{
    match path{
        "/curr_proj" => Redirect::to(uri!(curr_proj)),
        _ => Redirect::to(uri!(home))
    }
}


#[launch]
fn rocket() -> _{
   rocket::build()
       .mount("/", routes![redirect,home,curr_proj])
}
