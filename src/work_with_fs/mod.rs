use serde::{Serialize, Deserialize};
use ron;
use super::calculations::Diagram;
use crate::calculations::HashDiagram;
use chrono::{NaiveDate, Local};
use chrono::Datelike;
use std::collections::HashMap;

pub fn save(diagram: &HashMap<NaiveDate, HashDiagram>) -> Result<(), Box<dyn std::error::Error>> {
    let ron_string = ron::ser::to_string_pretty(
        &diagram,
        ron::ser::PrettyConfig::default(),
    )?;
    std::fs::write("diagram_neo.ron", ron_string)?;

    Ok(())
}
pub fn load() -> Result<HashMap<NaiveDate, HashDiagram>, Box<dyn std::error::Error>> {
    //read
    let content = std::fs::read_to_string("diagram.ron")?;
    let parsed: HashMap<NaiveDate, HashDiagram> = ron::from_str(&content)?;
    Ok(parsed)
}

