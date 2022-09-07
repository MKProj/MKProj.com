use rocket::{get, launch, routes};
use std::fs::File;
use std::io::Write;
use rocket::fs::NamedFile;
mod index;
use index::*;

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

#[launch]
fn rocket() -> _{
   rocket::build()
       .mount("/", routes![home])
}
