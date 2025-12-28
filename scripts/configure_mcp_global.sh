#!/bin/bash
# Configure MCP Server for VS Code, Cursor, and Windsurf (Linux/macOS)

set -e

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux paths
    VSCODE_PATH="$HOME/.config/Code/User/settings.json"
    CURSOR_PATH="$HOME/.config/Cursor/User/settings.json"
    WINDSURF_PATH="$HOME/.config/Windsurf/User/settings.json"
    CLAUDE_PATH="$HOME/.config/Claude/claude_desktop_config.json"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS paths
    VSCODE_PATH="$HOME/Library/Application Support/Code/User/settings.json"
    CURSOR_PATH="$HOME/Library/Application Support/Cursor/User/settings.json"
    WINDSURF_PATH="$HOME/Library/Application Support/Windsurf/User/settings.json"
    CLAUDE_PATH="$HOME/Library/Application Support/Claude/claude_desktop_config.json"
else
    echo "❌ Unsupported OS"
    exit 1
fi

# MCP Config
MCP_CONFIG='{"type":"http","url":"http://127.0.0.1:8079"}'

echo "🔧 Nuclear Crawler Hybrid - MCP Configuration"
echo "=============================================="

# Function to add MCP config to editor
configure_editor() {
    local editor_name=$1
    local config_path=$2
    local is_claude=$3
    
    echo ""
    echo "📍 $editor_name:"
    echo "   Path: $config_path"
    
    # Create parent directory if needed
    mkdir -p "$(dirname "$config_path")"
    
    # Create or update config
    if [ -f "$config_path" ]; then
        # File exists, update it
        if [ "$is_claude" = "true" ]; then
            # For Claude Desktop config
            python3 -c "
import json
with open('$config_path', 'r') as f:
    config = json.load(f)
if 'mcpServers' not in config:
    config['mcpServers'] = {}
config['mcpServers']['nuclear-crawler-hybrid'] = json.loads('$MCP_CONFIG')
with open('$config_path', 'w') as f:
    json.dump(config, f, indent=2)
"
        else:
            # For VS Code, Cursor, Windsurf
            python3 -c "
import json
with open('$config_path', 'r') as f:
    config = json.load(f)
if 'modelContextProtocol.servers' not in config:
    config['modelContextProtocol.servers'] = {}
config['modelContextProtocol.servers']['nuclear-crawler-hybrid'] = json.loads('$MCP_CONFIG')
with open('$config_path', 'w') as f:
    json.dump(config, f, indent=2)
"
        fi
        echo "   ✅ Updated"
    else
        # Create new config file
        if [ "$is_claude" = "true" ]; then
            echo "{\"mcpServers\": {\"nuclear-crawler-hybrid\": $MCP_CONFIG}}" | python3 -m json.tool > "$config_path"
        else
            echo "{\"modelContextProtocol.servers\": {\"nuclear-crawler-hybrid\": $MCP_CONFIG}}" | python3 -m json.tool > "$config_path"
        fi
        echo "   ✅ Created"
    fi
}

# Configure all editors
configure_editor "VS Code" "$VSCODE_PATH" "false"
configure_editor "Cursor" "$CURSOR_PATH" "false"
configure_editor "Windsurf" "$WINDSURF_PATH" "false"
configure_editor "Claude Desktop" "$CLAUDE_PATH" "true"

# Summary
echo ""
echo "=============================================="
echo "✅ Configuration Complete!"
echo ""
echo "🚀 Start MCP server:"
echo "   cargo run --bin nuclear-mcp"
echo ""
echo "📝 Server Details:"
echo "   URL: http://127.0.0.1:8079"
echo "   Tools: websearch, deepweb_search, premium_content_scraper, file_search"
echo ""
echo "🔄 Restart your editors to apply changes"
