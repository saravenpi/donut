use crate::models::{AppData, Project, Todo};
use crate::storage::Storage;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph},
    Frame, Terminal,
};

const PRIMARY_COLOR: Color = Color::Rgb(138, 180, 248);
const SECONDARY_COLOR: Color = Color::Rgb(187, 134, 252);
const SUCCESS_COLOR: Color = Color::Rgb(129, 199, 132);
const DIM_COLOR: Color = Color::Rgb(117, 117, 117);

#[derive(Debug, PartialEq)]
pub enum ViewMode {
    ProjectView,
    TodoView,
    CreateProjectView,
    CreateTodoView,
    EditTodoView,
    ConfirmDeleteProjectView,
}

pub struct App {
    pub data: AppData,
    pub storage: Storage,
    pub view_mode: ViewMode,
    pub cursor: usize,
    pub input: String,
    pub edit_index: Option<usize>,
    pub show_help: bool,
    pub should_quit: bool,
}

impl App {
    pub fn new(storage: Storage) -> Self {
        let mut data = AppData::new();
        data.projects = storage.load_projects();

        Self {
            data,
            storage,
            view_mode: ViewMode::ProjectView,
            cursor: 0,
            input: String::new(),
            edit_index: None,
            show_help: false,
            should_quit: false,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> std::io::Result<()> {
        loop {
            terminal.draw(|f| self.render(f))?;

            if self.should_quit {
                break;
            }

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    self.handle_key(key);
                }
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if self.show_help {
            if matches!(key.code, KeyCode::Char('?') | KeyCode::Esc) {
                self.show_help = false;
            }
            return;
        }

        match self.view_mode {
            ViewMode::ProjectView => self.handle_project_view_key(key),
            ViewMode::TodoView => self.handle_todo_view_key(key),
            ViewMode::CreateProjectView => self.handle_create_project_key(key),
            ViewMode::CreateTodoView => self.handle_create_todo_key(key),
            ViewMode::EditTodoView => self.handle_edit_todo_key(key),
            ViewMode::ConfirmDeleteProjectView => self.handle_confirm_delete_key(key),
        }
    }

    fn handle_project_view_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char('?') => {
                self.show_help = true;
            }
            KeyCode::Char('n') => {
                self.view_mode = ViewMode::CreateProjectView;
                self.input.clear();
            }
            KeyCode::Char('d') => {
                if !self.data.projects.is_empty() && self.cursor < self.data.projects.len() {
                    self.view_mode = ViewMode::ConfirmDeleteProjectView;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.cursor < self.data.projects.len().saturating_sub(1) {
                    self.cursor += 1;
                }
            }
            KeyCode::Tab | KeyCode::Enter => {
                if !self.data.projects.is_empty() && self.cursor < self.data.projects.len() {
                    self.data.current_project = self.cursor;
                    self.view_mode = ViewMode::TodoView;
                    self.cursor = 0;
                }
            }
            KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            _ => {}
        }
    }


    fn handle_todo_view_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Char('?') => {
                self.show_help = true;
            }
            KeyCode::Backspace | KeyCode::Esc => {
                self.view_mode = ViewMode::ProjectView;
                self.cursor = self.data.current_project;
            }
            KeyCode::Char('n') => {
                self.view_mode = ViewMode::CreateTodoView;
                self.input.clear();
            }
            KeyCode::Char('e') => {
                if let Some(project) = self.data.projects.get(self.data.current_project) {
                    if self.cursor < project.todos.len() {
                        self.edit_index = Some(self.cursor);
                        self.input = project.todos[self.cursor].title.clone();
                        self.view_mode = ViewMode::EditTodoView;
                    }
                }
            }
            KeyCode::Char('d') => {
                if let Some(project) = self.data.projects.get_mut(self.data.current_project) {
                    if self.cursor < project.todos.len() {
                        project.todos.remove(self.cursor);
                        if self.cursor >= project.todos.len() && self.cursor > 0 {
                            self.cursor -= 1;
                        }
                        let _ = self.storage.save_project(project);
                    }
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if let Some(project) = self.data.projects.get(self.data.current_project) {
                    if self.cursor < project.todos.len().saturating_sub(1) {
                        self.cursor += 1;
                    }
                }
            }
            KeyCode::Char(' ') => {
                if let Some(project) = self.data.projects.get_mut(self.data.current_project) {
                    if self.cursor < project.todos.len() {
                        project.todos[self.cursor].completed =
                            !project.todos[self.cursor].completed;
                        project.sort_todos();
                        let _ = self.storage.save_project(project);
                    }
                }
            }
            KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    fn handle_create_project_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if !self.input.is_empty() {
                    let filename = Storage::sanitize_filename(&self.input);
                    let project = Project::new(self.input.clone(), filename);
                    let _ = self.storage.save_project(&project);
                    self.data.projects.push(project);
                    self.cursor = self.data.projects.len().saturating_sub(1);
                    self.view_mode = ViewMode::ProjectView;
                    self.input.clear();
                } else {
                    self.view_mode = ViewMode::ProjectView;
                    self.input.clear();
                }
            }
            KeyCode::Esc => {
                self.view_mode = ViewMode::ProjectView;
                self.input.clear();
            }
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            _ => {}
        }
    }

    fn handle_create_todo_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if !self.input.is_empty() {
                    if let Some(project) = self.data.projects.get_mut(self.data.current_project) {
                        let todo = Todo::new(self.input.clone(), project.todos.len() + 1);
                        project.todos.push(todo);
                        project.sort_todos();
                        let _ = self.storage.save_project(project);
                        self.cursor = 0;
                        self.view_mode = ViewMode::TodoView;
                        self.input.clear();
                    }
                } else {
                    self.view_mode = ViewMode::TodoView;
                    self.input.clear();
                }
            }
            KeyCode::Esc => {
                self.view_mode = ViewMode::TodoView;
                self.input.clear();
            }
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            _ => {}
        }
    }

    fn handle_edit_todo_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if !self.input.is_empty() {
                    if let Some(project) = self.data.projects.get_mut(self.data.current_project) {
                        if let Some(idx) = self.edit_index {
                            if idx < project.todos.len() {
                                project.todos[idx].title = self.input.clone();
                                let _ = self.storage.save_project(project);
                                self.cursor = idx;
                            }
                        }
                    }
                } else if let Some(idx) = self.edit_index {
                    self.cursor = idx;
                }
                self.view_mode = ViewMode::TodoView;
                self.input.clear();
                self.edit_index = None;
            }
            KeyCode::Esc => {
                if let Some(idx) = self.edit_index {
                    self.cursor = idx;
                }
                self.view_mode = ViewMode::TodoView;
                self.input.clear();
                self.edit_index = None;
            }
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            _ => {}
        }
    }

    fn handle_confirm_delete_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if self.cursor < self.data.projects.len() {
                    let project = &self.data.projects[self.cursor];
                    let _ = self.storage.delete_project(&project.filename);
                    self.data.projects.remove(self.cursor);
                    if self.cursor >= self.data.projects.len() && self.cursor > 0 {
                        self.cursor -= 1;
                    }
                }
                self.view_mode = ViewMode::ProjectView;
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                self.view_mode = ViewMode::ProjectView;
            }
            _ => {}
        }
    }

    fn render(&mut self, f: &mut Frame) {
        if self.show_help {
            self.render_help(f);
        } else {
            match self.view_mode {
                ViewMode::ProjectView => self.render_project_view(f),
                ViewMode::TodoView => self.render_todo_view(f),
                ViewMode::CreateProjectView => self.render_create_project(f),
                ViewMode::CreateTodoView => self.render_create_todo(f),
                ViewMode::EditTodoView => self.render_edit_todo(f),
                ViewMode::ConfirmDeleteProjectView => self.render_confirm_delete(f),
            }
        }
    }

    fn render_project_view(&mut self, f: &mut Frame) {
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(30), Constraint::Min(0)])
            .split(f.area());

        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(main_chunks[1]);

        let sidebar_block = Block::default()
            .borders(ratatui::widgets::Borders::RIGHT)
            .border_style(Style::default().fg(DIM_COLOR));

        f.render_widget(sidebar_block, main_chunks[0]);

        let sidebar_inner = Rect {
            x: main_chunks[0].x,
            y: main_chunks[0].y,
            width: main_chunks[0].width.saturating_sub(1),
            height: main_chunks[0].height,
        };

        let sidebar_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)])
            .split(sidebar_inner);

        let logo = Paragraph::new(vec![
            Line::from(Span::styled(
                "  DONUT",
                Style::default()
                    .fg(PRIMARY_COLOR)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                "  Todo Manager",
                Style::default().fg(DIM_COLOR),
            )),
        ]);
        f.render_widget(logo, sidebar_chunks[0]);

        let project_items: Vec<ListItem> = if self.data.projects.is_empty() {
            vec![ListItem::new(Line::from(Span::styled(
                "  No projects",
                Style::default().fg(DIM_COLOR),
            )))]
        } else {
            self.data
                .projects
                .iter()
                .enumerate()
                .map(|(i, project)| {
                    let is_selected = i == self.cursor;
                    let cursor = if is_selected { "▸ " } else { "  " };
                    let checkbox = if project.completed_count() == project.total_count() && project.total_count() > 0 {
                        "✓ "
                    } else {
                        "○ "
                    };
                    let counter = format!(" {}/{}", project.completed_count(), project.total_count());

                    let style = if is_selected {
                        Style::default()
                            .fg(PRIMARY_COLOR)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };

                    ListItem::new(Line::from(vec![
                        Span::styled(cursor, style),
                        Span::styled(checkbox, style),
                        Span::styled(&project.name, style),
                        Span::styled(counter, Style::default().fg(DIM_COLOR)),
                    ]))
                })
                .collect()
        };

        let project_list = List::new(project_items).block(Block::default());
        f.render_widget(project_list, sidebar_chunks[1]);

        let help_sidebar = Paragraph::new("n: new | d: del")
            .style(Style::default().fg(DIM_COLOR));
        f.render_widget(help_sidebar, sidebar_chunks[2]);

        if !self.data.projects.is_empty() && self.cursor < self.data.projects.len() {
            let project = &self.data.projects[self.cursor];

            let todo_items: Vec<ListItem> = if project.todos.is_empty() {
                vec![ListItem::new(Line::from(Span::styled(
                    "  No todos yet. Press 'n' to create one!",
                    Style::default().fg(DIM_COLOR),
                )))]
            } else {
                project
                    .todos
                    .iter()
                    .map(|todo| {
                        let checkbox = if todo.completed { "✓" } else { "○" };
                        let style = if todo.completed {
                            Style::default()
                                .fg(SUCCESS_COLOR)
                                .add_modifier(Modifier::CROSSED_OUT)
                        } else {
                            Style::default()
                        };

                        ListItem::new(Line::from(Span::styled(
                            format!("  {} {}", checkbox, todo.title),
                            style,
                        )))
                    })
                    .collect()
            };

            let title = Paragraph::new(format!(" {}", project.name))
                .style(
                    Style::default()
                        .fg(PRIMARY_COLOR)
                        .add_modifier(Modifier::BOLD),
                )
                .alignment(Alignment::Left);

            let title_area = Rect {
                x: content_chunks[0].x,
                y: content_chunks[0].y,
                width: content_chunks[0].width,
                height: 1,
            };

            f.render_widget(title, title_area);

            let list_area = Rect {
                x: content_chunks[0].x,
                y: content_chunks[0].y + 1,
                width: content_chunks[0].width,
                height: content_chunks[0].height.saturating_sub(1),
            };

            let todo_list = List::new(todo_items).block(Block::default());
            f.render_widget(todo_list, list_area);
        } else {
            let empty = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  Select a project to view todos",
                    Style::default().fg(DIM_COLOR),
                )),
            ]);
            f.render_widget(empty, content_chunks[0]);
        }

        let help = Paragraph::new(" Tab: todos | ?: help | q: quit")
            .style(Style::default().fg(DIM_COLOR))
            .alignment(Alignment::Left);
        f.render_widget(help, content_chunks[1]);
    }

    fn render_todo_view(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(f.area());

        let project = &self.data.projects[self.data.current_project];

        let title = Paragraph::new(format!(" {}", project.name))
            .style(
                Style::default()
                    .fg(PRIMARY_COLOR)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Left);

        f.render_widget(title, chunks[0]);

        let items: Vec<ListItem> = if project.todos.is_empty() {
            vec![ListItem::new(Line::from(Span::styled(
                "No todos yet. Press 'n' to create one!",
                Style::default().fg(DIM_COLOR),
            )))]
        } else {
            project
                .todos
                .iter()
                .enumerate()
                .map(|(i, todo)| {
                    let is_selected = i == self.cursor;
                    let cursor = if is_selected { "▸ " } else { "  " };
                    let checkbox = if todo.completed { "✓" } else { "○" };

                    let style = if todo.completed {
                        Style::default()
                            .fg(SUCCESS_COLOR)
                            .add_modifier(Modifier::CROSSED_OUT)
                    } else if is_selected {
                        Style::default()
                            .fg(PRIMARY_COLOR)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };

                    ListItem::new(Line::from(Span::styled(
                        format!("{}{} {}", cursor, checkbox, todo.title),
                        style,
                    )))
                })
                .collect()
        };

        let list = List::new(items).block(Block::default());

        let list_area = Rect {
            x: chunks[0].x,
            y: chunks[0].y + 1,
            width: chunks[0].width,
            height: chunks[0].height.saturating_sub(1),
        };

        f.render_widget(list, list_area);

        let help = Paragraph::new("n: new | e: edit | d: delete | space: toggle | ?: help | backspace: back")
            .style(Style::default().fg(DIM_COLOR))
            .alignment(Alignment::Left);

        f.render_widget(help, chunks[1]);
    }

    fn render_create_project(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Length(3),
                Constraint::Percentage(40),
            ])
            .split(f.area());

        let input = Paragraph::new(format!("Create New Project: {}▌", self.input))
            .style(
                Style::default()
                    .fg(PRIMARY_COLOR)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);

        f.render_widget(input, chunks[1]);
    }

    fn render_create_todo(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Length(3),
                Constraint::Percentage(40),
            ])
            .split(f.area());

        let input = Paragraph::new(format!("Create New Todo: {}▌", self.input))
            .style(
                Style::default()
                    .fg(PRIMARY_COLOR)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);

        f.render_widget(input, chunks[1]);
    }

    fn render_edit_todo(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Length(3),
                Constraint::Percentage(40),
            ])
            .split(f.area());

        let input = Paragraph::new(format!("Edit Todo: {}▌", self.input))
            .style(
                Style::default()
                    .fg(PRIMARY_COLOR)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);

        f.render_widget(input, chunks[1]);
    }

    fn render_confirm_delete(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Length(3),
                Constraint::Percentage(40),
            ])
            .split(f.area());

        let project_name = if self.cursor < self.data.projects.len() {
            &self.data.projects[self.cursor].name
        } else {
            ""
        };

        let confirm = Paragraph::new(format!("Delete '{}'? (y/n)", project_name))
            .style(
                Style::default()
                    .fg(SECONDARY_COLOR)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);

        f.render_widget(confirm, chunks[1]);
    }

    fn render_help(&self, f: &mut Frame) {
        let help_text = vec![
            Line::from(Span::styled(
                " Donut - Keyboard Shortcuts",
                Style::default()
                    .fg(PRIMARY_COLOR)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from("Project View (Sidebar):"),
            Line::from("  ↑/↓ or j/k  - Navigate projects"),
            Line::from("  Tab/Enter   - View project todos"),
            Line::from("  n           - New project"),
            Line::from("  d           - Delete project"),
            Line::from("  ?           - Toggle help"),
            Line::from("  q/Esc       - Quit"),
            Line::from(""),
            Line::from("Todo View:"),
            Line::from("  ↑/↓ or j/k  - Navigate"),
            Line::from("  Space       - Toggle completion"),
            Line::from("  n           - New todo"),
            Line::from("  e           - Edit todo"),
            Line::from("  d           - Delete todo"),
            Line::from("  Backspace   - Back to projects"),
            Line::from("  ?           - Toggle help"),
            Line::from("  q           - Quit"),
            Line::from(""),
            Line::from(Span::styled(
                "Press ? or Esc to close",
                Style::default().fg(DIM_COLOR),
            )),
        ];

        let paragraph = Paragraph::new(help_text)
            .alignment(Alignment::Left);

        f.render_widget(paragraph, f.area());
    }
}
