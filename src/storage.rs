use crate::models::{Project, Todo};
use chrono::Utc;
use regex::Regex;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct Storage {
    donut_dir: PathBuf,
}

impl Storage {
    pub fn new(donut_dir: PathBuf) -> Self {
        Self { donut_dir }
    }

    pub fn ensure_dir_exists(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.donut_dir)
    }

    pub fn load_projects(&self) -> Vec<Project> {
        let mut projects = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.donut_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("md") {
                    if let Some(project) = self.load_project(&path) {
                        projects.push(project);
                    }
                }
            }
        }

        projects
    }

    fn load_project(&self, path: &Path) -> Option<Project> {
        let content = fs::read_to_string(path).ok()?;
        let filename = path.file_name()?.to_str()?.to_string();

        let title_regex = Regex::new(r"^#\s+(.+)$").unwrap();
        let todo_regex = Regex::new(r"^-\s+\[([ x])\]\s+(.+)$").unwrap();

        let mut project_name = filename.trim_end_matches(".md").to_string();
        let mut todos = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            if let Some(caps) = title_regex.captures(line) {
                project_name = caps[1].to_string();
            } else if let Some(caps) = todo_regex.captures(line) {
                let completed = &caps[1] == "x";
                let title = caps[2].to_string();
                todos.push(Todo {
                    title,
                    completed,
                    line_num: line_num + 1,
                    created_at: Utc::now(),
                });
            }
        }

        let mut project = Project::new(project_name, filename);
        project.todos = todos;
        project.sort_todos();

        Some(project)
    }

    pub fn save_project(&self, project: &Project) -> std::io::Result<()> {
        let path = self.donut_dir.join(&project.filename);
        let mut file = fs::File::create(path)?;

        writeln!(file, "# {}", project.name)?;
        writeln!(file)?;

        for todo in &project.todos {
            let checkbox = if todo.completed { "x" } else { " " };
            writeln!(file, "- [{}] {}", checkbox, todo.title)?;
        }

        Ok(())
    }

    pub fn delete_project(&self, filename: &str) -> std::io::Result<()> {
        let path = self.donut_dir.join(filename);
        fs::remove_file(path)
    }

    pub fn sanitize_filename(name: &str) -> String {
        let sanitized: String = name
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect();

        let clean: String = sanitized
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-");

        if clean.is_empty() {
            format!("project-{}", Utc::now().timestamp())
        } else {
            format!("{}.md", clean)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(Storage::sanitize_filename("My Project"), "my-project.md");
        assert_eq!(Storage::sanitize_filename("Test!!!"), "test.md");
        assert_eq!(
            Storage::sanitize_filename("Hello World 123"),
            "hello-world-123.md"
        );
    }
}
