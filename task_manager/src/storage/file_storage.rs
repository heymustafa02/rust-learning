use crate::models::task::Task;
use std::fs;

const FILE: &str = "tasks.json";

pub fn save(tasks: &Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write(FILE, data)?;
    Ok(())
}

pub fn load() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(FILE).unwrap_or("[]".to_string());
    let tasks: Vec<Task> = serde_json::from_str(&data)?;
    Ok(tasks)
}
