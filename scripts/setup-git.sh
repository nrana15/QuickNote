#!/bin/bash
# QuickNote Git Setup Script
# This script initializes the repo and prepares it for GitHub push

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_DIR"

echo "🔧 Setting up Git repository..."

# Initialize git if needed
if [ ! -d ".git" ]; then
    git init
fi

# Add all files
git add .

# Create initial commit
if [ "$(git rev-parse HEAD 2>/dev/null)" = "" ]; then
    echo "📝 Creating initial commit..."
    git commit -m "Initial commit: QuickNote MVP scaffold"
fi

echo "✅ Git repository initialized!"
echo ""
echo "To push to GitHub, run:"
echo "  git remote add origin https://<TOKEN>@github.com/<USERNAME>/QuickNote.git"
echo "  git push -u origin main"
echo ""
echo "Or configure your default GitHub token in environment variables."
