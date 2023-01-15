use chrono::{DateTime, FixedOffset};

#[derive(Debug, Default)]
pub struct Author {
    pub full: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Default)]
pub struct Stat {
    pub added: u32,
    pub deleted: u32,
    pub path: String,
    pub extension: String,
}

#[derive(Debug, Default)]
pub struct Numstat {
    pub commit: String,
    pub merges: Vec<String>,
    pub author: Author,
    pub date: DateTime<FixedOffset>,
    pub branches: Vec<String>,
    pub tags: Vec<String>,
    pub message: String,
    pub stats: Vec<Stat>,
}
