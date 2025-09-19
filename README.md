# 🍩 Donut

A simple, keyboard-controlled todolist TUI application with tmux plugin integration, built with Go and Bubbletea.

## Features

- 📝 **Project-based todos**: Organize your tasks into different projects
- ⌨️ **Fully keyboard controlled**: Navigate without touching your mouse
- 🎨 **Beautiful TUI**: Built with Charm Bracelet's Bubbletea
- 🔧 **Tmux integration**: Floating popup access via tmux plugin
- 💾 **Persistent storage**: Your todos are saved locally

## Quick Install

Install with a single command:

```bash
curl -fsSL https://raw.githubusercontent.com/saravenpi/donut/main/install.sh | bash
```

## Manual Installation

### Using Go

```bash
go install github.com/saravenpi/donut@latest
```

### Build from source

```bash
git clone https://github.com/saravenpi/donut.git
cd donut
make install
```

## Usage

### Standalone CLI

```bash
# Start the donut TUI
donut

# Show version
donut --version

# Show help
donut --help
```

### Tmux Plugin

Add to your `~/.tmux.conf`:

```bash
set -g @plugin 'saravenpi/donut'
```

Then install with TPM: `prefix + I`

Use the default keybinding `prefix + d` to open donut in a floating popup.

## Keyboard Controls

### Global
- `q` or `Ctrl+C` - Quit application
- `Tab` / `Shift+Tab` - Navigate between panels
- `?` - Show help

### Project View
- `↑/↓` or `j/k` - Navigate projects
- `Enter` - Select project
- `n` - New project
- `d` - Delete project
- `r` - Rename project

### Todo View
- `↑/↓` or `j/k` - Navigate todos
- `Space` - Toggle todo completion
- `n` - New todo
- `d` - Delete todo
- `e` - Edit todo
- `Backspace` - Return to projects

## Development

### Prerequisites

- Go 1.21+
- Make (optional, for build automation)

### Building

```bash
# Build for current platform
make build

# Build for all platforms
make build-all

# Run tests
make test

# Install locally
make install
```

### Project Structure

```
donut/
├── main.go           # Application entry point
├── models/           # Data models
├── ui/               # TUI components
├── storage/          # Data persistence
├── tmux/             # Tmux plugin files
└── install.sh        # Installation script
```

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.