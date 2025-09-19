# 🍩 Donut

A simple, keyboard-controlled todolist TUI application with tmux plugin integration, built with Go and Bubbletea.

## Features

- 📝 **Project-based todos**: Organize your tasks into different projects
- ⌨️ **Fully keyboard controlled**: Navigate without touching your mouse
- 🎨 **Beautiful TUI**: Built with Charm Bracelet's Bubbletea
- 📂 **Expandable projects**: View tasks inline with tab to expand/collapse
- 🔧 **Tmux integration**: Floating popup access via tmux plugin
- 💾 **Persistent storage**: Your todos are saved locally
- ⚙️ **Configurable**: Custom storage paths via ~/.donut.yml

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

## Configuration

Donut can be configured via `~/.donut.yml`:

```yaml
donut_dir: "~/Library/Mobile Documents/iCloud~md~obsidian/Documents/brain/todo/"
```

### Configuration Options

- `donut_dir` - Directory where project files are stored (supports tilde expansion)

If no config file exists, donut defaults to storing files in `~/.donut/`.

## Keyboard Controls

### Project View
- `↑/↓` or `j/k` - Navigate projects and expanded tasks
- `Tab` - Expand/collapse project to show tasks inline
- `Space` - Toggle task completion (when on expanded task)
- `Enter` - Open project view or select specific task
- `n` - Create new project
- `d` - Delete project
- `?` - Show help
- `q`, `Ctrl+C`, or `Esc` - Quit application

### Todo View
- `↑/↓` or `j/k` - Navigate todos
- `Space` - Toggle todo completion
- `n` - Create new todo
- `e` - Edit todo
- `d` - Delete todo
- `Backspace` or `Esc` - Return to projects
- `?` - Show help
- `q` or `Ctrl+C` - Quit application

### Input Mode (Create/Edit)
- `Type` - Enter text
- `Enter` - Confirm
- `Esc` - Cancel
- `Backspace` - Delete character

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