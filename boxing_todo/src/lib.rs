mod err;

use std::{error::Error, fs, path::Path};
use serde::{Deserialize, Serialize};
use crate::err::{ParseErr, ReadErr};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let file_path = Path::new(path);

        let contents = fs::read_to_string(file_path).map_err(|e| ReadErr { child_err: Box::new(e) })?;

        if contents.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        serde_json::from_str(&contents).map_err(|e| ParseErr::Malformed(Box::new(e)).into())
    }
}