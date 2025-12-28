#!/usr/bin/env python3
"""
Configure MCP Server for VS Code, Cursor, and Windsurf
"""

import json
import os
import sys
from pathlib import Path


def get_config_paths():
    """Get MCP config paths for all editors"""
    
    if sys.platform == "win32":
        appdata = os.getenv("APPDATA")
        return {
            "vscode": Path(appdata) / "Code" / "User" / "settings.json",
            "cursor": Path(appdata) / "Cursor" / "User" / "settings.json",
            "windsurf": Path(appdata) / "Windsurf" / "User" / "settings.json",
            "claude": Path(appdata) / "Claude" / "claude_desktop_config.json",
        }
    elif sys.platform == "darwin":  # macOS
        home = Path.home()
        return {
            "vscode": home / "Library" / "Application Support" / "Code" / "User" / "settings.json",
            "cursor": home / "Library" / "Application Support" / "Cursor" / "User" / "settings.json",
            "windsurf": home / "Library" / "Application Support" / "Windsurf" / "User" / "settings.json",
            "claude": home / "Library" / "Application Support" / "Claude" / "claude_desktop_config.json",
        }
    else:  # Linux
        home = Path.home()
        return {
            "vscode": home / ".config" / "Code" / "User" / "settings.json",
            "cursor": home / ".config" / "Cursor" / "User" / "settings.json",
            "windsurf": home / ".config" / "Windsurf" / "User" / "settings.json",
            "claude": home / ".config" / "Claude" / "claude_desktop_config.json",
        }


def add_mcp_config(editor_name, config_path):
    """Add MCP server config to editor"""
    
    mcp_config = {
        "type": "http",
        "url": "http://127.0.0.1:8079"
    }
    
    # Create parent directories if needed
    config_path.parent.mkdir(parents=True, exist_ok=True)
    
    # Load existing config or create new
    if config_path.exists():
        try:
            with open(config_path, 'r') as f:
                config = json.load(f)
        except json.JSONDecodeError:
            config = {}
    else:
        config = {}
    
    # Add or update MCP server config
    if editor_name == "claude":
        if "mcpServers" not in config:
            config["mcpServers"] = {}
        config["mcpServers"]["nuclear-crawler-hybrid"] = mcp_config
    else:
        if "modelContextProtocol.servers" not in config:
            config["modelContextProtocol.servers"] = {}
        config["modelContextProtocol.servers"]["nuclear-crawler-hybrid"] = mcp_config
    
    # Save updated config
    with open(config_path, 'w') as f:
        json.dump(config, f, indent=2)
    
    return True


def configure_all():
    """Configure all editors"""
    
    print("🔧 Nuclear Crawler Hybrid - MCP Server Configuration")
    print("=" * 60)
    
    paths = get_config_paths()
    results = {}
    
    for editor_name, config_path in paths.items():
        print(f"\n📍 {editor_name.upper()}:")
        print(f"   Path: {config_path}")
        
        try:
            if add_mcp_config(editor_name, config_path):
                print(f"   ✅ Configured successfully")
                results[editor_name] = "success"
            else:
                print(f"   ❌ Configuration failed")
                results[editor_name] = "failed"
        except Exception as e:
            print(f"   ❌ Error: {e}")
            results[editor_name] = f"error: {e}"
    
    # Summary
    print("\n" + "=" * 60)
    print("📊 Configuration Summary:")
    print("=" * 60)
    
    for editor_name, result in results.items():
        status = "✅" if result == "success" else "❌"
        print(f"{status} {editor_name.upper()}: {result}")
    
    print("\n🚀 Start the MCP server with:")
    print("   cargo run --bin nuclear-mcp")
    
    print("\n📝 Configuration details:")
    print("   URL: http://127.0.0.1:8079")
    print("   Tools: websearch, deepweb_search, premium_content_scraper, file_search")


if __name__ == "__main__":
    configure_all()
