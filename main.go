package main

import (
	"flag"
	"fmt"
	"log"

	"donut/ui"

	tea "github.com/charmbracelet/bubbletea"
)

var version = "dev"

func main() {
	var showVersion = flag.Bool("version", false, "Show version information")
	var showHelp = flag.Bool("help", false, "Show help information")
	flag.Parse()

	if *showVersion {
		fmt.Printf("donut version %s\n", version)
		return
	}

	if *showHelp {
		showHelpText()
		return
	}

	model, err := ui.NewModel()
	if err != nil {
		log.Fatal(err)
	}

	p := tea.NewProgram(model, tea.WithAltScreen())

	if _, err := p.Run(); err != nil {
		log.Fatal(err)
	}
}

func showHelpText() {
	fmt.Println(`🍩 Donut - Simple todolist TUI application

USAGE:
    donut [OPTIONS]

OPTIONS:
    --version    Show version information
    --help       Show this help message

KEYBOARD CONTROLS:

Project View:
    ↑/↓, j/k     Navigate projects
    Enter        Select project
    n            Create new project
    d            Delete project
    ?            Show/hide help
    q, Ctrl+C    Quit

Todo View:
    ↑/↓, j/k     Navigate todos
    Space        Toggle todo completion
    n            Create new todo
    e            Edit todo
    d            Delete todo
    Backspace    Return to projects
    ?            Show/hide help
    q, Ctrl+C    Quit

Input Mode:
    Type         Enter text
    Enter        Confirm
    Esc          Cancel
    Backspace    Delete character

TMUX INTEGRATION:
    Install the tmux plugin by adding to ~/.tmux.conf:
        set -g @plugin 'saravenpi/donut'

    Then install with TPM: prefix + I
    Use prefix + d to open donut in a floating popup.

INSTALLATION:
    curl -fsSL https://raw.githubusercontent.com/saravenpi/donut/main/install.sh | bash

DATA STORAGE:
    Your todos are stored in ~/.local/share/donut/data.json

For more information, visit: https://github.com/saravenpi/donut`)
}