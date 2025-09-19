# Donut Tmux Plugin

A tmux plugin that provides quick access to the donut todolist application via floating popup windows.

## Installation

### With TPM (Tmux Plugin Manager)

1. Add plugin to your `~/.tmux.conf`:

```bash
set -g @plugin 'saravenpi/donut'
```

2. Install with TPM: `prefix + I`

### Manual Installation

1. Clone the repository:

```bash
git clone https://github.com/saravenpi/donut ~/.tmux/plugins/donut
```

2. Add to your `~/.tmux.conf`:

```bash
run-shell ~/.tmux/plugins/donut/tmux/donut.tmux
```

3. Reload tmux configuration:

```bash
tmux source-file ~/.tmux.conf
```

## Usage

- **Default keybinding**: `prefix + d`
- Opens donut in a floating popup window (80% width, 80% height)
- Popup inherits the current pane's working directory
- Close popup with `q` or `Ctrl+C` in donut, or `Esc` in tmux

## Configuration

### Custom Key Binding

You can customize the key binding by setting the `@donut-key` option in your `~/.tmux.conf`:

```bash
# Use 't' instead of 'd'
set -g @donut-key 't'

# Use 'C-t' for Ctrl+t
set -g @donut-key 'C-t'
```

### Popup Size

The popup size is currently set to 80% width and 80% height. To customize this, you can manually modify the binding:

```bash
# Custom popup size (60% width, 70% height)
bind-key d display-popup -E -w 60% -h 70% -d "#{pane_current_path}" 'donut'
```

## Requirements

- tmux 3.2+ (for `display-popup` support)
- donut binary installed in PATH

If donut is not installed, the popup will show installation instructions.

## Troubleshooting

### "donut not found" error

The donut binary is not in your PATH. Install it with:

```bash
curl -fsSL https://raw.githubusercontent.com/saravenpi/donut/main/install.sh | bash
```

Make sure `~/.local/bin` is in your PATH by adding this to your shell profile:

```bash
export PATH="$PATH:~/.local/bin"
```

### Popup doesn't appear

- Ensure you have tmux 3.2 or later: `tmux -V`
- Check that the plugin is properly loaded: `tmux list-keys | grep donut`
- Verify the key binding: press `prefix + ?` to see all bindings

### Plugin not loading

- For TPM: Make sure you've pressed `prefix + I` to install
- For manual installation: Check the path in your `run-shell` command
- Verify the script is executable: `ls -la ~/.tmux/plugins/donut/tmux/donut.tmux`