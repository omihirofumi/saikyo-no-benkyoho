use std::error::Error;

#[derive(Debug)]
pub struct Record {
    pub id: i32,
    pub category: String,
    pub title: String,
    pub contents: String,
}

impl Record {
    pub fn new(id: i32, category: String, title: String, contents: String) -> Record {
        Record {
            id,
            category,
            title,
            contents,
        }
    }
}

pub async fn create_record() -> String {
    "Record created".to_string()
}
