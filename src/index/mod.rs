mod feed;

use std::fs::read_to_string;
use std::ops::Deref;
use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::index::feed::{Article, latest_feed, update_feed};
use crate::Result;
lazy_static!{
    pub static ref ARTICLES: Mutex<Vec<Article>> = Mutex::new(Vec::new());
    static ref LATEST: &'static str = r#"<a href="{link}">
    <div class="w3-quarter">
        <div class="w3-container w3-{color} w3-padding-16 w3-hover-white">
            <div class="w3-left"><i class="w3-xlarge">{title}</i></div>
            <br>
            <h5 class="w3-right">{pub_date}</h5>
           </div>
    </div>
</a>
    "#;
    static ref TABLE: &'static str = r#"<tr>
                                <td>
                                    <a href="{link}" class="w3-hover-white">
                                    {title}
                                    </a>
                                </td>
                                <td>{categories}</td>
                                <td><i>{pub_date}</i></td>
                            </tr>"#;
}

pub fn base() -> Result<String>{
    update_feed()?;
    let mut base_string = read_to_string("templates/base.html")?;
    base_string = gen_latest(&base_string);
    base_string = gen_articles(&base_string);
    Ok(base_string)
}

fn gen_latest(base: &str) -> String{
    let latests = latest_feed();
    let colors = ["green", "purple", "orange", "teal"];
    let mut posts = Vec::new();
    for i in 0..latests.len(){
        let mut string = LATEST.to_string();
        let latest = &latests[i];
        let color = colors[i];
        string = string.replace("{link}", &latest.link);
        string = string.replace("{color}", color);
        string = string.replace("{title}", &latest.title);
        string = string.replace("{pub_date}", &latest.published_date);
        posts.push(string)
    }
    base.replace("{latest posts}", posts.join("\n").as_str())
}

fn gen_articles(base_string: &str) -> String {
    let articles = ARTICLES.lock().unwrap();
    let mut tables = Vec::new();
    for a in articles.deref(){
        let mut table = TABLE.to_string();
        table = table.replace("{link}", &a.link);
        table = table.replace("{title}", &a.title);
        table = table.replace("{categories}", &a.categories.join(","));
        table = table.replace("{pub_date}", &a.published_date);
        tables.push(table)
    }
    base_string.replace("{articles}", &tables.join("\n"))
}