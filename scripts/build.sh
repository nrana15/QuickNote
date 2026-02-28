#!/bin/bash
# QuickNote Build Script
# Builds both CLI (Rust) and GUI (Tauri) versions

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_DIR"

echo "🔨 Building QuickNote..."
echo ""

# Check prerequisites
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Install from https://rustup.rs/"
    exit 1
fi

if ! command -v npm &> /dev/null; then
    echo "❌ Node.js/npm not found. Install from https://nodejs.org/"
    exit 1
fi

echo "✅ Prerequisites check passed"
echo ""

# Build CLI version (Rust only)
echo "📦 Building CLI version..."
cargo build --release -p quicknote
echo "✅ CLI binary built: target/release/quicknote"
echo ""

# Install Node dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing npm dependencies..."
    npm install
fi

# Build GUI version (Tauri)
echo "🎨 Building GUI version (this may take a few minutes)..."
npm run tauri build
echo ""

# Show output locations
echo "✅ Build complete! Output files:"
if [ -f "dist/bundler/msi/*.msi" ]; then
    echo "  • Windows Installer: dist/bundler/msi/*.msi"
fi
if [ -f "dist/bundler/appimage/*.appimage" ]; then
    echo "  • Linux AppImage: dist/bundler/appimage/*.appimage"
fi

echo ""
echo "To test the GUI version:"
echo "  npm run tauri dev"
echo ""
echo "To test the CLI version:"
echo "  cargo run --release -p quicknote"
echo ""
