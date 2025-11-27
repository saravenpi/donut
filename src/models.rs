use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
    pub line_num: usize,
    pub created_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(title: String, line_num: usize) -> Self {
        Self {
            title,
            completed: false,
            line_num,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub filename: String,
    pub todos: Vec<Todo>,
}

impl Project {
    pub fn new(name: String, filename: String) -> Self {
        Self {
            name,
            filename,
            todos: Vec::new(),
        }
    }

    pub fn completed_count(&self) -> usize {
        self.todos.iter().filter(|t| t.completed).count()
    }

    pub fn total_count(&self) -> usize {
        self.todos.len()
    }

    pub fn sort_todos(&mut self) {
        self.todos.sort_by(|a, b| {
            match (a.completed, b.completed) {
                (true, false) => std::cmp::Ordering::Greater,
                (false, true) => std::cmp::Ordering::Less,
                _ => b.created_at.cmp(&a.created_at),
            }
        });
    }
}

#[derive(Debug)]
pub struct AppData {
    pub projects: Vec<Project>,
    pub current_project: usize,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            projects: Vec::new(),
            current_project: 0,
        }
    }
}

impl Default for AppData {
    fn default() -> Self {
        Self::new()
    }
}
