#!/bin/bash
# Wrapper script to ensure environment variables are set correctly

# Set DISPLAY if not already set
if [ -z "$DISPLAY" ]; then
    export DISPLAY=:0
fi

# Set XAUTHORITY if not already set
if [ -z "$XAUTHORITY" ]; then
    export XAUTHORITY="$HOME/.Xauthority"
fi

# Ensure the X authority file exists
if [ ! -f "$XAUTHORITY" ]; then
    # Try common locations
    for auth_file in "$HOME/.Xauthority" "$XDG_RUNTIME_DIR/gdm/Xauthority" "/run/user/$(id -u)/gdm/Xauthority"; do
        if [ -f "$auth_file" ]; then
            export XAUTHORITY="$auth_file"
            break
        fi
    done
fi

# Run the actual daemon
exec "$HOME/.local/bin/clipboard-daemon"
