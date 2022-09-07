use chrono::{Datelike, DateTime};
use std::process::Command;
use crate::Result;
use super::ARTICLES;
use std::fs::File;
use std::io::BufReader;
use rss::{Channel, Item};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Article{
    pub title: String,
    pub categories: Vec<String>,
    pub published_date: String,
    pub link: String,
}
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Latest{
    pub title: String,
    pub link: String,
    pub published_date: String,
}

impl Latest{
    pub fn new(article: &Article) -> Self{
        Self{
            title: article.title.clone(),
            link: article.link.clone(),
            published_date: article.published_date.clone()
        }
    }
}

impl Article {
    pub fn new(item: &Item) -> Self{
        let categories = item.categories.iter().map(|x|x.name.clone()).collect();
        let datetime = DateTime::parse_from_rfc2822(&item.pub_date.clone().unwrap()).unwrap();
        let date = datetime.date();
        let month = if date.month() > 9{
            format!("{}", date.month())
        } else {
            format!("0{}", date.month())
        };
        let day = if date.day() > 9{
            format!("{}", date.day())
        } else {
            format!("0{}", date.day())
        };
        let published_date = format!("{}-{}-{}", date.year(), month, day);
        Self{
            title: item.title.clone().unwrap(),
            categories,
            published_date,
            link: item.link.clone().unwrap()
        }
    }
}


fn get_feed() -> Result<()>{
    let _ = Command::new("wget")
        .arg("https://dev.to/feed/mustafif")
        .arg("-O")
        .arg("feed.xml")
        .output()?;
    Ok(())
}
// updates the feed to be lazily evaluated by ARTICLES
pub fn update_feed() -> Result<()>{
    get_feed()?;
    let mut articles = ARTICLES.lock().unwrap();
    let file = File::open("feed.xml")?;
    let channel = Channel::read_from(BufReader::new(file))?;
    // if the length is the same, no change has happened
    if articles.len() == channel.items.len(){
        return Ok(())
    }
    let mut vec = Vec::new();
    for article in &channel.items{
        vec.push(Article::new(article))
    }
    *articles = vec;
    Ok(())
}

pub fn latest_feed() -> Vec<Latest>{
    let articles = ARTICLES.lock().unwrap();
    let mut latest = Vec::new();
    for i in 0..4usize{
        latest.push(Latest::new(&articles[i]))
    }
    latest
}
