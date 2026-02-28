use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{State, Manager};

#[derive(Serialize)]
struct AppState {
    note_count: usize,
}

#[derive(Deserialize)]
struct AddNoteArgs {
    title: String,
    content: String,
}

type Db = Arc<Mutex<rusqlite::Connection>>;

fn init_db(db_path: &PathBuf) -> Result<Db, Box<dyn std::error::Error>> {
    let conn = rusqlite::Connection::open(db_path)?;
    
    // Create notes table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            knowledge_type TEXT CHECK(knowledge_type IN 
                ('Concept', 'Snippet', 'Checklist', 'Note', 'Process', 'SQLQuery', 'DebugPattern')),
            tags TEXT DEFAULT '[]'
        )",
        [],
    )?;

    println!("✅ Database initialized at {:?}", db_path);
    Ok(Arc::new(Mutex::new(conn)))
}

#[tauri::command]
fn add_note(state: State<Db>, args: AddNoteArgs) -> Result<u64, String> {
    let conn = state.get();
    let mut connection = conn.lock().unwrap();
    
    let knowledge_type = if args.content.to_lowercase().contains("select") || 
                         args.content.to_lowercase().contains("from ") ||
                         args.content.to_lowercase().contains("insert into") {
        "SQLQuery"
    } else if args.content.to_lowercase().contains("error") || 
              args.content.to_lowercase().contains("exception") ||
              args.content.to_lowercase().contains("panic") {
        "DebugPattern"
    } else {
        "Concept"
    };
    
    let id: u64 = connection
        .query_row(
            "INSERT INTO notes (title, content, knowledge_type) VALUES (?, ?, ?)",
            [&args.title, &args.content, knowledge_type],
            |row| row.get::<_, u64>(0),
        )
        .map_err(|e| format!("Failed to insert note: {}", e))?;

    Ok(id)
}

#[tauri::command]
fn search_notes(state: State<Db>, query: String) -> Result<Vec<serde_json::Value>, String> {
    let conn = state.get();
    let connection = conn.lock().unwrap();
    
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }

    let mut stmt = connection
        .prepare(
            "SELECT id, title, content, knowledge_type 
             FROM notes 
             WHERE title LIKE ?1 OR content LIKE ?1
             ORDER BY id DESC"
        )
        .map_err(|e| format!("Failed to prepare search: {}", e))?;

    let results = stmt
        .query_map([format!("%{}%", query)], |row| {
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
fn get_note_count(state: State<Db>) -> Result<usize, String> {
    let conn = state.get();
    let connection = conn.lock().unwrap();
    
    let count: usize = connection
        .query_row("SELECT COUNT(*) FROM notes", [], |row| row.get(0))
        .map_err(|e| format!("Failed to count notes: {}", e))?;

    Ok(count)
}

fn main() {
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
                
                init_db(&db_path).map(|_| {}).unwrap_or_else(|e| {
                    eprintln!("❌ Database initialization failed: {}", e);
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_note, search_notes, get_note_count])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
