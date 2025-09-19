package ui

import (
	"fmt"
	"strings"

	"donut/models"
	"donut/storage"

	"github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

type ViewMode int

const (
	ProjectView ViewMode = iota
	TodoView
	CreateProjectView
	CreateTodoView
	EditTodoView
	HelpView
	ConfirmDeleteProjectView
)

type Model struct {
	storage      *storage.Storage
	data         *models.AppData
	mode         ViewMode
	projectCursor int
	todoCursor   int
	inputValue   string
	inputMode    bool
	message      string
	width        int
	height       int
}

func NewModel() (*Model, error) {
	s, err := storage.New()
	if err != nil {
		return nil, err
	}

	data, err := s.Load()
	if err != nil {
		return nil, err
	}

	if len(data.Projects) == 0 {
		defaultProject := models.NewProject("Personal")
		data.Projects = append(data.Projects, defaultProject)
		s.Save(data)
	}

	return &Model{
		storage:       s,
		data:          data,
		mode:          ProjectView,
		projectCursor: 0,
		todoCursor:    0,
		inputValue:    "",
		inputMode:     false,
		message:       "",
	}, nil
}

func (m Model) Init() tea.Cmd {
	return nil
}

func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		m.width = msg.Width
		m.height = msg.Height

	case tea.KeyMsg:
		return m.handleKeypress(msg)
	}

	return m, nil
}

func (m Model) handleKeypress(msg tea.KeyMsg) (tea.Model, tea.Cmd) {
	switch m.mode {
	case ProjectView:
		return m.handleProjectViewKeys(msg)
	case TodoView:
		return m.handleTodoViewKeys(msg)
	case CreateProjectView:
		return m.handleCreateProjectKeys(msg)
	case CreateTodoView:
		return m.handleCreateTodoKeys(msg)
	case EditTodoView:
		return m.handleEditTodoKeys(msg)
	case HelpView:
		return m.handleHelpViewKeys(msg)
	case ConfirmDeleteProjectView:
		return m.handleConfirmDeleteProjectKeys(msg)
	}
	return m, nil
}

func (m Model) handleProjectViewKeys(msg tea.KeyMsg) (tea.Model, tea.Cmd) {
	switch msg.String() {
	case "ctrl+c", "q":
		return m, tea.Quit
	case "up", "k":
		if m.projectCursor > 0 {
			m.projectCursor--
		}
	case "down", "j":
		if m.projectCursor < len(m.data.Projects)-1 {
			m.projectCursor++
		}
	case "enter":
		if len(m.data.Projects) > 0 {
			m.mode = TodoView
			m.todoCursor = 0
		}
	case "n":
		m.mode = CreateProjectView
		m.inputValue = ""
		m.inputMode = true
	case "d":
		if len(m.data.Projects) > 0 {
			m.mode = ConfirmDeleteProjectView
		}
	case "?":
		m.mode = HelpView
	}
	return m, nil
}

func (m Model) handleTodoViewKeys(msg tea.KeyMsg) (tea.Model, tea.Cmd) {
	switch msg.String() {
	case "ctrl+c", "q":
		return m, tea.Quit
	case "backspace":
		m.mode = ProjectView
	case "up", "k":
		if m.todoCursor > 0 {
			m.todoCursor--
		}
	case "down", "j":
		currentProject := m.getCurrentProject()
		if currentProject != nil && m.todoCursor < len(currentProject.Todos)-1 {
			m.todoCursor++
		}
	case " ":
		m.toggleTodo()
	case "n":
		m.mode = CreateTodoView
		m.inputValue = ""
		m.inputMode = true
	case "d":
		m.deleteTodo()
	case "e":
		currentProject := m.getCurrentProject()
		if currentProject != nil && len(currentProject.Todos) > 0 {
			m.mode = EditTodoView
			m.inputValue = currentProject.Todos[m.todoCursor].Title
			m.inputMode = true
		}
	case "?":
		m.mode = HelpView
	}
	return m, nil
}

func (m Model) handleCreateProjectKeys(msg tea.KeyMsg) (tea.Model, tea.Cmd) {
	switch msg.String() {
	case "ctrl+c", "q":
		return m, tea.Quit
	case "esc":
		m.mode = ProjectView
		m.inputMode = false
		m.inputValue = ""
	case "enter":
		if strings.TrimSpace(m.inputValue) != "" {
			m.createProject()
		}
		m.mode = ProjectView
		m.inputMode = false
		m.inputValue = ""
	case "backspace":
		if len(m.inputValue) > 0 {
			m.inputValue = m.inputValue[:len(m.inputValue)-1]
		}
	default:
		m.inputValue += msg.String()
	}
	return m, nil
}

func (m Model) handleCreateTodoKeys(msg tea.KeyMsg) (tea.Model, tea.Cmd) {
	switch msg.String() {
	case "ctrl+c", "q":
		return m, tea.Quit
	case "esc":
		m.mode = TodoView
		m.inputMode = false
		m.inputValue = ""
	case "enter":
		if strings.TrimSpace(m.inputValue) != "" {
			m.createTodo()
		}
		m.mode = TodoView
		m.inputMode = false
		m.inputValue = ""
	case "backspace":
		if len(m.inputValue) > 0 {
			m.inputValue = m.inputValue[:len(m.inputValue)-1]
		}
	default:
		m.inputValue += msg.String()
	}
	return m, nil
}

func (m Model) handleEditTodoKeys(msg tea.KeyMsg) (tea.Model, tea.Cmd) {
	switch msg.String() {
	case "ctrl+c", "q":
		return m, tea.Quit
	case "esc":
		m.mode = TodoView
		m.inputMode = false
		m.inputValue = ""
	case "enter":
		if strings.TrimSpace(m.inputValue) != "" {
			m.editTodo()
		}
		m.mode = TodoView
		m.inputMode = false
		m.inputValue = ""
	case "backspace":
		if len(m.inputValue) > 0 {
			m.inputValue = m.inputValue[:len(m.inputValue)-1]
		}
	default:
		m.inputValue += msg.String()
	}
	return m, nil
}

func (m Model) handleHelpViewKeys(msg tea.KeyMsg) (tea.Model, tea.Cmd) {
	switch msg.String() {
	case "ctrl+c", "q", "esc", "?":
		if m.mode == TodoView {
			m.mode = TodoView
		} else {
			m.mode = ProjectView
		}
	}
	return m, nil
}

func (m Model) handleConfirmDeleteProjectKeys(msg tea.KeyMsg) (tea.Model, tea.Cmd) {
	switch msg.String() {
	case "ctrl+c", "q":
		return m, tea.Quit
	case "esc", "n":
		m.mode = ProjectView
	case "y", "enter":
		m.deleteProject()
		m.mode = ProjectView
	}
	return m, nil
}

func (m Model) View() string {
	switch m.mode {
	case ProjectView:
		return m.renderProjectView()
	case TodoView:
		return m.renderTodoView()
	case CreateProjectView:
		return m.renderCreateProjectView()
	case CreateTodoView:
		return m.renderCreateTodoView()
	case EditTodoView:
		return m.renderEditTodoView()
	case HelpView:
		return m.renderHelpView()
	case ConfirmDeleteProjectView:
		return m.renderConfirmDeleteProjectView()
	}
	return ""
}

var (
	titleStyle = lipgloss.NewStyle().
			Bold(true).
			Foreground(lipgloss.Color("#FF6B6B")).
			MarginBottom(1)

	selectedStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#FF6B6B")).
			Bold(true)

	completedStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#51CF66")).
			Strikethrough(true)

	inputStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#FF6B6B")).
			Bold(true)
)

func (m Model) renderProjectView() string {
	title := titleStyle.Render("🍩 Donut - Projects")

	var projects []string
	for i, project := range m.data.Projects {
		cursor := " "
		if i == m.projectCursor {
			cursor = ">"
			project.Name = selectedStyle.Render(project.Name)
		}
		todoCount := len(project.Todos)
		completedCount := 0
		for _, todo := range project.Todos {
			if todo.Completed {
				completedCount++
			}
		}
		line := fmt.Sprintf("%s %s (%d/%d)", cursor, project.Name, completedCount, todoCount)
		projects = append(projects, line)
	}

	content := strings.Join(projects, "\n")
	if len(projects) == 0 {
		content = "No projects yet. Press 'n' to create one!"
	}

	help := "\nControls: ↑/↓ or j/k (navigate), Enter (select), n (new), d (delete), ? (help), q (quit)"

	return title + "\n" + content + help
}

func (m Model) renderTodoView() string {
	currentProject := m.getCurrentProject()
	if currentProject == nil {
		return "No project selected"
	}

	title := titleStyle.Render(fmt.Sprintf("🍩 Donut - %s", currentProject.Name))

	var todos []string
	for i, todo := range currentProject.Todos {
		cursor := " "
		if i == m.todoCursor {
			cursor = ">"
		}

		checkbox := "☐"
		todoText := todo.Title
		if todo.Completed {
			checkbox = "☑"
			todoText = completedStyle.Render(todoText)
		}

		if i == m.todoCursor && !todo.Completed {
			todoText = selectedStyle.Render(todoText)
		}

		line := fmt.Sprintf("%s %s %s", cursor, checkbox, todoText)
		todos = append(todos, line)
	}

	content := strings.Join(todos, "\n")
	if len(todos) == 0 {
		content = "No todos yet. Press 'n' to create one!"
	}

	help := "\nControls: ↑/↓ or j/k (navigate), Space (toggle), n (new), e (edit), d (delete), Backspace (projects), ? (help), q (quit)"

	return title + "\n" + content + help
}

func (m Model) renderCreateProjectView() string {
	title := titleStyle.Render("🍩 Create New Project")
	prompt := "Project name: "
	input := inputStyle.Render(m.inputValue + "█")
	help := "\nPress Enter to create, Esc to cancel"

	return title + "\n" + prompt + input + help
}

func (m Model) renderCreateTodoView() string {
	title := titleStyle.Render("🍩 Create New Todo")
	prompt := "Todo title: "
	input := inputStyle.Render(m.inputValue + "█")
	help := "\nPress Enter to create, Esc to cancel"

	return title + "\n" + prompt + input + help
}

func (m Model) renderEditTodoView() string {
	title := titleStyle.Render("🍩 Edit Todo")
	prompt := "Todo title: "
	input := inputStyle.Render(m.inputValue + "█")
	help := "\nPress Enter to save, Esc to cancel"

	return title + "\n" + prompt + input + help
}

func (m Model) renderHelpView() string {
	title := titleStyle.Render("🍩 Donut - Help")

	help := `
Project View:
  ↑/↓, j/k    Navigate projects
  Enter       Select project
  n           Create new project
  d           Delete project
  ?           Show/hide help
  q, Ctrl+C   Quit

Todo View:
  ↑/↓, j/k    Navigate todos
  Space       Toggle todo completion
  n           Create new todo
  e           Edit todo
  d           Delete todo
  Backspace   Return to projects
  ?           Show/hide help
  q, Ctrl+C   Quit

Input Mode:
  Type        Enter text
  Enter       Confirm
  Esc         Cancel
  Backspace   Delete character
`

	footer := "\nPress any key to return..."

	return title + help + footer
}

func (m Model) renderConfirmDeleteProjectView() string {
	currentProject := m.getCurrentProject()
	if currentProject == nil {
		return "No project selected"
	}

	title := titleStyle.Render("🍩 Delete Project")

	warning := fmt.Sprintf("Are you sure you want to delete the project '%s'?", currentProject.Name)
	todoCount := len(currentProject.Todos)
	if todoCount > 0 {
		warning += fmt.Sprintf("\nThis will permanently delete %d todo(s).", todoCount)
	}

	options := "\nPress 'y' or Enter to confirm, 'n' or Esc to cancel"

	return title + "\n" + warning + options
}

func (m *Model) getCurrentProject() *models.Project {
	if m.projectCursor >= 0 && m.projectCursor < len(m.data.Projects) {
		return &m.data.Projects[m.projectCursor]
	}
	return nil
}

func (m *Model) createProject() {
	project := models.NewProject(strings.TrimSpace(m.inputValue))
	m.data.Projects = append(m.data.Projects, project)
	m.projectCursor = len(m.data.Projects) - 1
	m.storage.Save(m.data)
}

func (m *Model) deleteProject() {
	if len(m.data.Projects) > 0 && m.projectCursor < len(m.data.Projects) {
		m.data.Projects = append(m.data.Projects[:m.projectCursor], m.data.Projects[m.projectCursor+1:]...)
		if m.projectCursor >= len(m.data.Projects) && len(m.data.Projects) > 0 {
			m.projectCursor = len(m.data.Projects) - 1
		}
		m.storage.Save(m.data)
	}
}

func (m *Model) createTodo() {
	currentProject := m.getCurrentProject()
	if currentProject != nil {
		todo := models.NewTodo(strings.TrimSpace(m.inputValue))
		currentProject.Todos = append(currentProject.Todos, todo)
		m.todoCursor = len(currentProject.Todos) - 1
		m.storage.Save(m.data)
	}
}

func (m *Model) deleteTodo() {
	currentProject := m.getCurrentProject()
	if currentProject != nil && len(currentProject.Todos) > 0 && m.todoCursor < len(currentProject.Todos) {
		currentProject.Todos = append(currentProject.Todos[:m.todoCursor], currentProject.Todos[m.todoCursor+1:]...)
		if m.todoCursor >= len(currentProject.Todos) && len(currentProject.Todos) > 0 {
			m.todoCursor = len(currentProject.Todos) - 1
		}
		m.storage.Save(m.data)
	}
}

func (m *Model) editTodo() {
	currentProject := m.getCurrentProject()
	if currentProject != nil && len(currentProject.Todos) > 0 && m.todoCursor < len(currentProject.Todos) {
		currentProject.Todos[m.todoCursor].Title = strings.TrimSpace(m.inputValue)
		m.storage.Save(m.data)
	}
}

func (m *Model) toggleTodo() {
	currentProject := m.getCurrentProject()
	if currentProject != nil && len(currentProject.Todos) > 0 && m.todoCursor < len(currentProject.Todos) {
		currentProject.Todos[m.todoCursor].Completed = !currentProject.Todos[m.todoCursor].Completed
		m.storage.Save(m.data)
	}
}