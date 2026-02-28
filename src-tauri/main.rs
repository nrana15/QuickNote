use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{State, AppHandle};

#[derive(Serialize)]
struct AppState {
    note_count: usize,
}

#[derive(Deserialize)]
struct AddNoteArgs {
    title: String,
    content: String,
}

#[tauri::command]
fn add_note(
    state: State<rusqlite::Connection>,
    args: AddNoteArgs,
) -> Result<u64, String> {
    let mut conn = state.get().lock().unwrap();
    
    // Auto-categorize based on content patterns
    let knowledge_type = auto_categorize(&args.content);
    
    // Insert note with transaction
    let id: u64 = conn
        .query_row(
            "INSERT INTO notes (title, content, knowledge_type) VALUES (?, ?, ?)",
            [&args.title, &args.content, &knowledge_type],
            |row| row.get::<_, u64>(0),
        )
        .map_err(|e| format!("Failed to insert note: {}", e))?;

    Ok(id)
}

#[tauri::command]
fn search_notes(
    state: State<rusqlite::Connection>,
    query: String,
) -> Result<Vec<serde_json::Value>, String> {
    let conn = state.get().lock().unwrap();
    
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }

    // Use FTS5 for full-text search
    let mut stmt = conn
        .prepare(
            "SELECT id, title, content, knowledge_type 
             FROM notes_fts 
             WHERE notes_fts MATCH ?1
             ORDER BY rowid DESC"
        )
        .map_err(|e| format!("Failed to prepare search: {}", e))?;

    let results = stmt
        .query_map([query], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, u64>(0)?,
                "title": row.get::<_, String>(1)?,
                "content": row.get::<_, String>(2)?,
                "knowledge_type": row.get::<_, String>(3)?
            }))
        })
        .map_err(|e| format!("Failed to query notes: {}", e))?;

    let mut notes = Vec::new();
    for result in results {
        if let Ok(note) = result {
            notes.push(note);
        }
    }

    Ok(notes)
}

#[tauri::command]
fn get_note_count(state: State<rusqlite::Connection>) -> Result<usize, String> {
    let conn = state.get().lock().unwrap();
    
    let count: usize = conn
        .query_row("SELECT COUNT(*) FROM notes", [], |row| row.get(0))
        .map_err(|e| format!("Failed to count notes: {}", e))?;

    Ok(count)
}

fn auto_categorize(content: &str) -> String {
    let lower_content = content.to_lowercase();
    
    if lower_content.contains("select") || lower_content.contains("from ") || lower_content.contains("insert into") {
        return "SQLQuery".to_string();
    }
    
    if lower_content.contains("error") || lower_content.contains("exception") || lower_content.contains("panic") {
        return "DebugPattern".to_string();
    }
    
    // Default to Concept for most knowledge items
    "Concept".to_string()
}

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

    println!("✅ Database initialized at {:?}", db_path);
    Ok(())
}

fn main() {
    println!("🚀 QuickNote — Portable Knowledge Pocket v0.1");
    
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            
            // Initialize database if not exists
            let db_path = app_handle
                .path()
                .app_data_dir()
                .expect("Failed to get data directory")
                .join("vault.db");

            println!("Database path: {:?}", db_path);

            if !db_path.exists() {
                println!("📦 Initializing new vault...");
                
                // Ensure parent directories exist
                if let Some(parent) = db_path.parent() {
                    std::fs::create_dir_all(parent).expect("Failed to create data directory");
                }
                
                init_database(&db_path).unwrap_or_else(|e| {
                    eprintln!("❌ Database initialization failed: {}", e);
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_note, 
            search_notes, 
            get_note_count
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
