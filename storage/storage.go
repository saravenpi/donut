package storage

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"

	"donut/config"
	"donut/models"
)

type Storage struct {
	donutDir string
}

func New() (*Storage, error) {
	cfg, err := config.Load()
	if err != nil {
		return nil, err
	}

	if err := os.MkdirAll(cfg.DonutDir, 0755); err != nil {
		return nil, err
	}

	return &Storage{
		donutDir: cfg.DonutDir,
	}, nil
}

func (s *Storage) Load() (*models.AppData, error) {
	data := models.NewAppData()

	files, err := os.ReadDir(s.donutDir)
	if err != nil {
		return &data, nil
	}

	for _, file := range files {
		if !file.IsDir() && strings.HasSuffix(file.Name(), ".md") {
			project, err := s.loadProject(file.Name())
			if err != nil {
				continue
			}
			data.Projects = append(data.Projects, project)
		}
	}

	if len(data.Projects) == 0 {
		defaultProject := models.NewProject("Personal")
		data.Projects = append(data.Projects, defaultProject)
		s.saveProject(&defaultProject)
	}

	return &data, nil
}

func (s *Storage) loadProject(filename string) (models.Project, error) {
	project := models.Project{
		Filename: filename,
		Todos:    []models.Todo{},
	}

	filePath := filepath.Join(s.donutDir, filename)
	file, err := os.Open(filePath)
	if err != nil {
		return project, err
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	lineNum := 0
	titleRegex := regexp.MustCompile(`^#\s+(.+)$`)
	todoRegex := regexp.MustCompile(`^-\s+\[([ x])\]\s+(.+)$`)

	for scanner.Scan() {
		lineNum++
		line := scanner.Text()

		if matches := titleRegex.FindStringSubmatch(line); matches != nil {
			project.Name = matches[1]
		} else if matches := todoRegex.FindStringSubmatch(line); matches != nil {
			completed := matches[1] == "x"
			title := matches[2]
			todo := models.Todo{
				Title:     title,
				Completed: completed,
				LineNum:   lineNum,
			}
			project.Todos = append(project.Todos, todo)
		}
	}

	if project.Name == "" {
		baseName := strings.TrimSuffix(filename, ".md")
		project.Name = strings.ReplaceAll(baseName, "-", " ")
		project.Name = strings.Title(project.Name)
	}

	return project, scanner.Err()
}

func (s *Storage) Save(data *models.AppData) error {
	for i := range data.Projects {
		if err := s.saveProject(&data.Projects[i]); err != nil {
			return err
		}
	}
	return nil
}

func (s *Storage) saveProject(project *models.Project) error {
	filePath := project.GetFilePath(s.donutDir)

	var content strings.Builder
	content.WriteString(fmt.Sprintf("# %s\n\n", project.Name))

	for _, todo := range project.Todos {
		checkbox := " "
		if todo.Completed {
			checkbox = "x"
		}
		content.WriteString(fmt.Sprintf("- [%s] %s\n", checkbox, todo.Title))
	}

	return os.WriteFile(filePath, []byte(content.String()), 0644)
}

func (s *Storage) DeleteProject(project *models.Project) error {
	filePath := project.GetFilePath(s.donutDir)
	return os.Remove(filePath)
}

func (s *Storage) GetDonutDir() string {
	return s.donutDir
}