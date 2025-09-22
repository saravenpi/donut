package models

import (
	"fmt"
	"path/filepath"
	"sort"
	"strings"
	"time"
)

type Todo struct {
	Title     string
	Completed bool
	LineNum   int
	CreatedAt time.Time
}

type Project struct {
	Name     string
	Filename string
	Todos    []Todo
}

type AppData struct {
	Projects       []Project
	CurrentProject int
}

func NewTodo(title string) Todo {
	return Todo{
		Title:     title,
		Completed: false,
		LineNum:   -1,
		CreatedAt: time.Now(),
	}
}

func NewProject(name string) Project {
	filename := generateFilename(name)
	return Project{
		Name:     name,
		Filename: filename,
		Todos:    []Todo{},
	}
}

func NewAppData() AppData {
	return AppData{
		Projects:       []Project{},
		CurrentProject: 0,
	}
}

func generateFilename(name string) string {
	safe := strings.ToLower(name)
	safe = strings.ReplaceAll(safe, " ", "-")
	safe = strings.ReplaceAll(safe, "_", "-")

	var result strings.Builder
	for _, r := range safe {
		if (r >= 'a' && r <= 'z') || (r >= '0' && r <= '9') || r == '-' {
			result.WriteRune(r)
		}
	}

	filename := result.String()
	if filename == "" {
		filename = fmt.Sprintf("project-%d", time.Now().Unix())
	}

	return filename + ".md"
}

func (p *Project) GetFilePath(donutDir string) string {
	return filepath.Join(donutDir, p.Filename)
}

// SortTodos sorts todos with completed tasks at the bottom (muted)
// and within each group, sorts by creation date (latest first)
func (p *Project) SortTodos() {
	sort.SliceStable(p.Todos, func(i, j int) bool {
		todoI := &p.Todos[i]
		todoJ := &p.Todos[j]

		// If completion status differs, incomplete tasks come first
		if todoI.Completed != todoJ.Completed {
			return !todoI.Completed
		}

		// Within the same completion status, sort by creation date (latest first)
		return todoI.CreatedAt.After(todoJ.CreatedAt)
	})
}