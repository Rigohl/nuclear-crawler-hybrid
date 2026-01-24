#!/bin/bash
# 🧠 SETUP: Chapel AI - Complete Installation & Build Guide
# ═════════════════════════════════════════════════════════════════════════════

set -e

echo "╔════════════════════════════════════════════════════════════════════════════╗"
echo "║ 🧠 Chapel AI - Complete Setup & Build                                     ║"
echo "╚════════════════════════════════════════════════════════════════════════════╝"
echo ""

# ─────────────────────────────────────────────────────────────────────────────────
# STEP 1: Option A - Use Pre-compiled Binary (FASTEST - 5 minutes)
# ─────────────────────────────────────────────────────────────────────────────────

setup_chapel_binary() {
    echo "📥 INSTALLING CHAPEL FROM PRECOMPILED BINARY (Recommended)"
    echo "═══════════════════════════════════════════════════════════"
    echo ""
    
    cd /tmp
    
    # Download precompiled Binary
    echo "⬇️  Downloading Chapel 2.1.0 precompiled (Linux x86_64)..."
    wget -q --show-progress https://github.com/chapel-lang/chapel/releases/download/2.1.0/chapel-2.1.0-linux64-gnu.tar.gz
    
    # Extract
    echo "📦 Extracting..."
    tar xzf chapel-2.1.0-linux64-gnu.tar.gz
    
    # Install
    echo "🔧 Installing to /opt/chapel..."
    sudo mv chapel-2.1.0-linux64-gnu /opt/chapel
    
    echo ""
    echo "✅ SETUP COMPLETE!"
    echo ""
    echo "Add to your shell profile (~/.bashrc or ~/.zshrc):"
    echo ""
    echo '	export CHPL_HOME=/opt/chapel'
    echo '	export PATH=$CHPL_HOME/bin:$PATH'
    echo ""
    echo "Then source it:"
    echo "	source ~/.bashrc"
    echo ""
}

# ─────────────────────────────────────────────────────────────────────────────────
# STEP 2: Option B - Build from Source (SLOW - 30+ minutes, needs space)
# ─────────────────────────────────────────────────────────────────────────────────

setup_chapel_from_source() {
    echo "🏗️  BUILDING CHAPEL FROM SOURCE"
    echo "═════════════════════════════════════════════════"
    echo ""
    echo "⚠️  WARNING: This requires:"
    echo "  • 30-60 minutes"
    echo "  • 20+ GB disk space"
    echo "  • build-essential, llvm-14, cmake, python3"
    echo ""
    
    read -p "Continue? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        return
    fi
    
    # Install dependencies
    echo "📦 Installing build dependencies..."
    sudo apt update
    sudo apt install -y build-essential llvm-14 llvm-14-dev cmake python3-dev
    
    # Download source
    cd /tmp
    echo "⬇️  Downloading Chapel source..."
    wget -q --show-progress https://github.com/chapel-lang/chapel/releases/download/2.1.0/chapel-2.1.0.tar.gz
    
    # Extract
    echo "📦 Extracting..."
    tar xzf chapel-2.1.0.tar.gz
    cd chapel-2.1.0
    
    # Configure
    echo "⚙️  Configuring..."
    export CHPL_LLVM=bundled
    ./configure --prefix=/opt/chapel
    
    # Build (this takes 30+ minutes)
    echo "🔨 Building Chapel (this will take 30-60 minutes)..."
    make -j4
    
    # Install
    echo "📦 Installing..."
    make install
    
    echo ""
    echo "✅ BUILD COMPLETE!"
    echo ""
}

# ─────────────────────────────────────────────────────────────────────────────────
# STEP 3: Build Rust with Chapel FFI
# ─────────────────────────────────────────────────────────────────────────────────

build_rust_with_chapel() {
    echo ""
    echo "🦀 BUILDING RUST WITH CHAPEL FFI"
    echo "════════════════════════════════════════════════"
    echo ""
    
    # Make sure CHPL_HOME is set
    if [ -z "$CHPL_HOME" ]; then
        if [ -d "/opt/chapel" ]; then
            export CHPL_HOME=/opt/chapel
        else
            echo "❌ Chapel not found at /opt/chapel"
            echo "   Please run setup_chapel_binary() or setup_chapel_from_source() first"
            return 1
        fi
    fi
    
    echo "Chapel Home: $CHPL_HOME"
    echo ""
    
    cd /workspaces/nuclear-crawler-hybrid
    
    # Clean previous builds
    echo "🧹 Cleaning previous builds..."
    cargo clean
    
    # Build with Chapel FFI
    echo "🔨 Building nuclear-crawler-hybrid with Chapel FFI..."
    cargo build --release --features chapel_ffi
    
    echo ""
    echo "✅ BUILD COMPLETE!"
    echo ""
    echo "Binary location: ./target/release/nuclear-mcp"
    echo ""
}

# ─────────────────────────────────────────────────────────────────────────────────
# MAIN MENU
# ─────────────────────────────────────────────────────────────────────────────────

main() {
    echo "Choose installation method:"
    echo ""
    echo "1) Precompiled Binary (Recommended - 5 min)"
    echo "2) Build from Source (30-60 min)"
    echo "3) Build Rust (Assumes Chapel already installed)"
    echo "4) Exit"
    echo ""
    
    read -p "Select (1-4): " choice
    
    case $choice in
        1)
            setup_chapel_binary
            read -p "Build Rust now? (y/n) " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                build_rust_with_chapel
            fi
            ;;
        2)
            setup_chapel_from_source
            read -p "Build Rust now? (y/n) " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                build_rust_with_chapel
            fi
            ;;
        3)
            build_rust_with_chapel
            ;;
        4)
            echo "Exiting..."
            exit 0
            ;;
        *)
            echo "Invalid option"
            main
            ;;
    esac
}

# Check if running with argument
if [ $# -eq 0 ]; then
    main
else
    case $1 in
        binary)
            setup_chapel_binary
            ;;
        source)
            setup_chapel_from_source
            ;;
        build)
            build_rust_with_chapel
            ;;
        *)
            echo "Usage: $0 [binary|source|build]"
            echo ""
            echo "  binary  - Install Chapel from precompiled binary"
            echo "  source  - Build Chapel from source"
            echo "  build   - Build nuclear-crawler-hybrid with Chapel FFI"
            ;;
    esac
fi
