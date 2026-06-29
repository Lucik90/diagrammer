use std::collections::HashMap;
use chrono::{NaiveDate, Local};
use chrono::Datelike;
use serde::{Serialize, Deserialize};
pub fn transformer(diagram: &HashMap<NaiveDate, HashDiagram>) -> Vec<Diagram> {
    //init new sorted diagram
    let mut sorted_diagram: Vec<Diagram> = vec![];
    //check diagram
    if diagram.is_empty() {
        return sorted_diagram;
    }
    let mut dates: Vec<NaiveDate> = diagram.keys().cloned().collect();
    dates.sort();

    //get sorted diagram
    let mut first_date: NaiveDate = *dates.first().unwrap();
    let last_date: NaiveDate = *dates.last().unwrap();
    while first_date <= last_date {
        sorted_diagram.push(Diagram {
            date: first_date,
            count: match diagram.get(&first_date) {
                Some(value) => value.count,
                None => 0 as u32,
            },
        });
        first_date = first_date.succ_opt().expect("error: date overflow");
    }

    sorted_diagram
}
pub fn chunk_string_by_chars(string: &str, chunk_size: usize) -> Vec<String> {
    string.chars()
        .collect::<Vec<char>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Diagram {
    pub date: NaiveDate,
    pub count: u32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HashDiagram {
    pub count: u32,
    pub comment: String,
}
