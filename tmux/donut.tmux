#!/usr/bin/env bash

CURRENT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Default key binding
DEFAULT_KEY="d"

# Get user-defined key binding or use default
get_tmux_option() {
    local option="$1"
    local default_value="$2"
    local option_value=$(tmux show-option -gqv "$option")
    if [ -z "$option_value" ]; then
        echo "$default_value"
    else
        echo "$option_value"
    fi
}

# Main script function
main() {
    local key=$(get_tmux_option "@donut-key" "$DEFAULT_KEY")

    # Bind the key to open donut in a popup
    tmux bind-key "$key" display-popup -E -w 80% -h 80% -T " Donut " -d "#{pane_current_path}" \
        'command -v donut >/dev/null 2>&1 && donut || echo "donut not found. Install with: curl -fsSL https://raw.githubusercontent.com/saravenpi/donut/main/install.sh | bash"'
}

main