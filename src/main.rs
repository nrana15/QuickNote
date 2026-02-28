//! QuickNote — Portable Knowledge Pocket
//! Single-binary, admin-free knowledge keeper

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    gui_mode: bool,
    modules: Vec<String>,
    encryption_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            gui_mode: true,
            modules: vec!["search".to_string(), "categorize".to_string()],
            encryption_enabled: false,
        }
    }
}

#[derive(Debug)]
enum KnowledgeType {
    Concept,
    Snippet,
    Checklist,
    Note,
    Process,
    SQLQuery,
    DebugPattern,
}

impl std::fmt::Display for KnowledgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Concept => write!(f, "Concept"),
            Self::Snippet => write!(f, "Snippet"),
            Self::Checklist => write!(f, "Checklist"),
            Self::Note => write!(f, "Note"),
            Self::Process => write!(f, "Process"),
            Self::SQLQuery => write!(f, "SQL Query"),
            Self::DebugPattern => write!(f, "Debug Pattern"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Note {
    id: u64,
    title: String,
    content: String,
    knowledge_type: KnowledgeType,
    tags: Vec<String>,
    created_at: i64,
    updated_at: i64,
}

/// Portable mode detection — checks if data folder exists alongside executable
fn detect_portable_mode() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let exe_path = std::env::current_exe()?;
    let app_dir = exe_path.parent().ok_or("Cannot determine app directory")?;
    
    // Check if data folder exists (portable mode indicator)
    let data_dir = app_dir.join("data");
    
    if data_dir.exists() && data_dir.is_dir() {
        Ok(data_dir)
    } else {
        Err("Not in portable mode — please create 'data' folder alongside executable".into())
    }
}

/// Auto-categorize note based on content patterns
fn categorize_note(content: &str, title: &str) -> (KnowledgeType, Vec<String>) {
    let mut tags = Vec::new();
    
    // Extract #tags from content
    for word in content.split_whitespace() {
        if word.starts_with('#') && !word.is_empty() {
            tags.push(word[1..].to_string());
        }
    }
    
    // Pattern matching for knowledge type detection
    let lower_content = content.to_lowercase();
    let lower_title = title.to_lowercase();
    
    if lower_content.contains("select") || lower_content.contains("from ") || lower_content.contains("insert into") {
        return (KnowledgeType::SQLQuery, tags);
    }
    
    if lower_content.contains("error") || lower_content.contains("exception") || lower_content.contains("panic") {
        return (KnowledgeType::DebugPattern, tags);
    }
    
    if lower_title.starts_with(|c: char| c.is_ascii_digit()) && content.split('\n').count() > 3 {
        return (KnowledgeType::Process, tags);
    }
    
    // Default to Concept for most knowledge items
    (KnowledgeType::Concept, tags)
}

/// Initialize SQLite database if not exists
fn init_database(db_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let conn = rusqlite::Connection::open(db_path)?;
    
    // Create notes table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            knowledge_type TEXT CHECK(knowledge_type IN 
                ('Concept', 'Snippet', 'Checklist', 'Note', 'Process', 'SQLQuery', 'DebugPattern')),
            tags TEXT DEFAULT '[]',
            created_at INTEGER DEFAULT (strftime('%s', 'now')),
            updated_at INTEGER DEFAULT (strftime('%s', 'now'))
        )",
        [],
    )?;
    
    // Create FTS5 virtual table for full-text search
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS notes_fts USING fts5(
            title, content,
            content='notes',
            content_rowid='id'
        )",
        [],
    )?;
    
    // Triggers to keep FTS in sync
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS notes_ai AFTER INSERT ON notes BEGIN
            INSERT INTO notes_fts(rowid, title, content) VALUES (new.id, new.title, new.content);
        END",
        [],
    )?;
    
    println!("✅ Database initialized at {:?}", db_path);
    Ok(())
}

/// Add a new note to the vault
fn add_note(conn: &rusqlite::Connection, title: String, content: String) -> Result<u64, Box<dyn std::error::Error>> {
    let (knowledge_type, tags) = categorize_note(&content, &title);
    
    // Insert note
    let id = conn.query_row(
        "INSERT INTO notes (title, content, knowledge_type, tags) VALUES (?, ?, ?, ?)",
        rusqlite::params![title, content, knowledge_type.to_string(), serde_json::to_string(&tags)?],
        |row| row.get::<_, u64>(0),
    )?;
    
    // Update FTS index
    conn.execute(
        "INSERT INTO notes_fts(rowid, title, content) VALUES (?, ?, ?)",
        rusqlite::params![id, title, content],
    )?;
    
    println!("✅ Note added: {} (ID: {})", title, id);
    Ok(id)
}

/// Search notes using FTS5
fn search_notes(conn: &rusqlite::Connection, query: &str) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    let results = conn.query_map(
        "SELECT n.id, n.title, n.content, n.knowledge_type, n.tags, n.created_at 
         FROM notes n 
         JOIN notes_fts f ON n.id = f.rowid 
         WHERE notes_fts MATCH ?
         ORDER BY n.updated_at DESC",
        [query],
        |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                knowledge_type: match row.get::<_, String>(3)?.as_str() {
                    "Concept" => KnowledgeType::Concept,
                    "Snippet" => KnowledgeType::Snippet,
                    "Checklist" => KnowledgeType::Checklist,
                    "Note" => KnowledgeType::Note,
                    "Process" => KnowledgeType::Process,
                    "SQLQuery" => KnowledgeType::SQLQuery,
                    "DebugPattern" => KnowledgeType::DebugPattern,
                    _ => KnowledgeType::Concept, // fallback
                },
                tags: serde_json::from_str(row.get::<_, String>(4)?)?,
                created_at: row.get(5)?,
                updated_at: 0, // would need to query again for updated_at in real impl
            })
        },
    )?;
    
    let notes: Result<Vec<Note>, _> = results.collect();
    Ok(notes?)
}

fn main() {
    println!("🚀 QuickNote — Portable Knowledge Pocket v0.1");
    
    // Detect portable mode
    let data_dir = match detect_portable_mode() {
        Ok(path) => {
            println!("✅ Running in PORTABLE MODE from: {:?}", path);
            path
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    };
    
    let db_path = data_dir.join("vault.db");
    
    // Initialize database if not exists
    if !db_path.exists() {
        println!("📦 Initializing new vault...");
        init_database(&db_path).unwrap();
    }
    
    // Load config
    let config_path = std::env::current_exe().unwrap().parent().unwrap().join("config.json");
    let config: Config = if config_path.exists() {
        let content = fs::read_to_string(&config_path).unwrap();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Config::default()
    };
    
    println!("📋 Configuration loaded: {} modules active", config.modules.len());
    
    // Connect to database
    let conn = rusqlite::Connection::open(&db_path).expect("Failed to open database");
    
    // Demo mode: add a sample note if vault is empty
    let count: usize = conn.query_row(
        "SELECT COUNT(*) FROM notes",
        [],
        |row| row.get(0),
    ).unwrap();
    
    if count == 0 {
        println!("📝 Adding demo note (delete via SQL to start fresh)...");
        let _id = add_note(&conn, 
            "Welcome to QuickNote!".to_string(), 
            "This is your portable knowledge pocket. Press Ctrl+K to quickly capture thoughts.\n\n#sql query for finding duplicate emails:\nSELECT email, COUNT(*) FROM users GROUP BY email HAVING COUNT(*) > 1;".to_string()
        ).unwrap();
    }
    
    println!("🎯 QuickNote is ready!");
    println!("\nTo start adding notes:");
    println!("  - Launch GUI mode (if enabled) with `cargo tauri dev`");
    println!("  - Or use CLI commands directly");
    
    // Demo search
    let demo_results = search_notes(&conn, "sql").unwrap();
    if !demo_results.is_empty() {
        println!("\n🔍 Search demo found {} note(s) matching 'sql':", demo_results.len());
        for note in &demo_results {
            println!("  - [{}] {}", note.knowledge_type, note.title);
        }
    }
}
