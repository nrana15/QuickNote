# QuickNote — Portable Knowledge Pocket 📝

**A lightweight, single-binary knowledge keeper that runs without admin rights.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Rust](https://img.shields.io/badge/Rust-1.75+-orange)
![Tauri](https://img.shields.io/badge/Tauri-2.0-blue)

---

## ✨ Features

- **🚀 Portable**: Single executable that runs anywhere (no installation needed)
- **🔒 Admin-Free**: Works on any PC without administrator privileges
- **⚡ Search-First**: Instant full-text search with FTS5 indexing
- **🧠 Smart Categorization**: Auto-tags notes based on content patterns
- **📚 Spaced Repetition**: Built-in review mode using SM-2 algorithm
- **💾 Local Only**: All data stays on your machine — no cloud sync ever

---

## 🎯 Use Cases

QuickNote is perfect for:

- **Students**: Capture lecture notes, formulas, and concepts
- **Developers**: Store SQL queries, code snippets, debugging patterns
- **Professionals**: Document processes, checklists, interview questions
- **Researchers**: Organize ideas, references, and insights

---

## 📦 Installation (No Install Required!)

### Quick Start (Portable Mode)

1. Download the latest release binary for your platform:
   - [Windows (.exe)](https://github.com/YOUR_USERNAME/QuickNote/releases/latest/download/quicknote.exe)
   - [macOS (Intel/Apple Silicon)](https://github.com/YOUR_USERNAME/QuickNote/releases/latest/download/quicknote.app.zip)
   - [Linux AppImage](https://github.com/YOUR_USERNAME/QuickNote/releases/latest/download/quicknote.AppImage)

2. **Double-click to launch** — that's it!

3. Press `Ctrl+K` to start adding notes immediately.

---

## 🛠️ Development Setup

### Prerequisites

- **Rust 1.70+**: [Install via rustup](https://rustup.rs/)
- **Node.js 18+**: [Download from nodejs.org](https://nodejs.org/)
- **Tauri CLI** (optional): `cargo install tauri-cli`

### Build from Source

```bash
cd /Users/nitin/.openclaw/workspace/projects/QuickNote

# Install dependencies
npm install

# Run in development mode (with hot reload)
npm run tauri dev

# Or build production binaries
npm run tauri build
```

### CLI-Only Mode (Rust)

For the terminal-based version without GUI:

```bash
cargo run --release -p quicknote
```

---

## 🎮 Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+K` | Quick add note |
| `Ctrl+F` / `/` | Focus search bar |
| `Ctrl+E` | Export vault to ZIP |
| `Ctrl+R` | Start review session |
| `Escape` | Close modal |

---

## 📖 How It Works

### Auto-Categorization

QuickNote automatically identifies note types based on content patterns:

- **SQL Query**: Detects SELECT, FROM, INSERT INTO keywords
- **Debug Pattern**: Recognizes ERROR, exception, panic messages  
- **Process**: Identifies numbered lists (1., 2., 3.)
- **Concept/Note**: Default for most other items

### Spaced Repetition (SM-2)

The built-in review system uses the SuperMemo-2 algorithm:

1. Click "Review Mode" or press `Ctrl+R`
2. Review cards due today (based on your learning progress)
3. Rate each card: **Again** / **Hard** / **Good** / **Easy**
4. System schedules next review based on your rating

---

## 🗂️ Data Storage

All data stored locally in SQLite:

```
QuickNote/
├── QuickNote.exe          # Main executable (or .app on macOS)
└── data/                  # Portable storage folder (auto-created)
    ├── vault.db           # SQLite database with all notes
    └── cache/             # Search index and temp files
```

**Moving to another machine?** Just copy the entire folder — everything travels with you!

---

## 🔐 Security & Privacy

- **No Cloud Sync**: All data stays on your device
- **Optional Encryption**: Coming in v0.2 (AES-GCM vault lock)
- **Zero Telemetry**: No analytics or crash reporting
- **Open Source**: MIT License — free for personal and commercial use

---

## 📚 Project Structure

```
QuickNote/
├── src/                     # Rust core engine
│   ├── main.rs             # Application entry point
│   └── vite-env.d.ts       # TypeScript declarations
├── src-tauri/              # Tauri GUI wrapper
│   ├── main.rs             # App initialization & commands
│   └── tauri.conf.json     # Configuration
├── src/components/         # React UI components
│   ├── SearchBar.tsx
│   ├── QuickAddModal.tsx
│   ├── NoteList.tsx
│   └── ReviewMode.tsx
├── scripts/                # Build & setup utilities
│   ├── build.sh            # Build script
│   └── setup-git.sh        # Git initialization helper
├── README.md               # This file
└── GETTING_STARTED.md      # Development guide
```

---

## 🚀 Roadmap

### v0.1 (Current) - MVP ✅
- [x] Portable single-binary distribution
- [x] SQLite storage with FTS5 search
- [x] Auto-categorization engine
- [x] Review mode with SM-2 algorithm
- [x] ZIP export functionality

### v0.2 (Next)
- [ ] Vault encryption (password protection)
- [ ] PDF export for printable reports
- [ ] Dark mode theme
- [ ] Tag cloud visualization

### v0.3 (Future)
- [ ] Cloud sync toggle (Dropbox/Google Drive optional)
- [ ] Mobile companion app (iOS/Android)
- [ ] Collaboration features (shared vaults, team tags)

---

## 🤝 Contributing

Contributions welcome! Here's how:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

**See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.**

---

## 📄 License

MIT License — Free for personal and commercial use.

```
Copyright (c) 2026 QuickNote Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## 🙏 Acknowledgments

- Built with **Rust** and **Tauri** for the desktop framework
- Uses **SQLite** with FTS5 for fast full-text search
- Implements **SM-2 algorithm** from SuperMemo research
- Icons from **Lucide React** (when added)

---

## 📬 Contact & Support

- **GitHub Issues**: [Report bugs or request features](https://github.com/YOUR_USERNAME/QuickNote/issues)
- **Discussions**: [Join the conversation](https://github.com/YOUR_USERNAME/QuickNote/discussions)
- **Twitter**: [@quicknote_app](https://twitter.com/quicknote_app) (coming soon)

---

**Made with ❤️ using Rust + Tauri**

*Ready to capture your knowledge, anywhere you go!* 🚀
