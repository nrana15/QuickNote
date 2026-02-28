use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{State, Manager};

// Thread-safe database wrapper
type Db = Arc<Mutex<rusqlite::Connection>>;

#[derive(Serialize)]
struct AppState {
    note_count: usize,
}

#[derive(Deserialize)]
struct AddNoteArgs {
    title: String,
    content: String,
}

// Initialize database and return thread-safe connection
fn init_db(db_path: &PathBuf) -> Result<Db, String> {
    let conn = rusqlite::Connection::open(db_path).map_err(|e| format!("DB open error: {}", e))?;
    
    // Create notes table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| format!("Table creation error: {}", e))?;

    println!("✅ Database initialized at {:?}", db_path);
    Ok(Arc::new(Mutex::new(conn)))
}

// Add a new note
#[tauri::command]
fn add_note(db: State<Db>, args: AddNoteArgs) -> Result<u64, String> {
    let conn = db.get();
    let mut connection = conn.lock().map_err(|e| format!("Mutex lock error: {}", e))?;
    
    // Auto-detect type
    let lower_content = args.content.to_lowercase();
    let note_type = if lower_content.contains("select") || 
                     lower_content.contains("from ") ||
                     lower_content.contains("insert into") {
        "SQL Query"
    } else if lower_content.contains("error") || 
              lower_content.contains("exception") ||
              lower_content.contains("panic") {
        "Debug Pattern"
    } else {
        "Note"
    };
    
    let id: u64 = connection
        .query_row(
            "INSERT INTO notes (title, content) VALUES (?, ?)",
            [&args.title, &args.content],
            |row| row.get::<_, u64>(0),
        )
        .map_err(|e| format!("Insert error: {}", e))?;

    Ok(id)
}

// Search notes by title or content
#[tauri::command]
fn search_notes(db: State<Db>, query: String) -> Result<Vec<serde_json::Value>, String> {
    let conn = db.get();
    let connection = conn.lock().map_err(|e| format!("Mutex lock error: {}", e))?;
    
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }

    let mut stmt = connection
        .prepare("SELECT id, title, content FROM notes WHERE title LIKE ? OR content LIKE ? ORDER BY id DESC")
        .map_err(|e| format!("Prepare error: {}", e))?;

    let search_term = format!("%{}%", query);
    
    let results = stmt
        .query_map([search_term.clone(), search_term], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, u64>(0)?,
                "title": row.get::<_, String>(1)?,
                "content": row.get::<_, String>(2)?
            }))
        })
        .map_err(|e| format!("Query error: {}", e))?;

    let mut notes = Vec::new();
    for result in results {
        if let Ok(note) = result {
            notes.push(note);
        }
    }

    Ok(notes)
}

// Get total note count
#[tauri::command]
fn get_note_count(db: State<Db>) -> Result<usize, String> {
    let conn = db.get();
    let connection = conn.lock().map_err(|e| format!("Mutex lock error: {}", e))?;
    
    let count: usize = connection
        .query_row("SELECT COUNT(*) FROM notes", [], |row| row.get(0))
        .map_err(|e| format!("Count error: {}", e))?;

    Ok(count)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            
            // Get data directory path
            let db_path = app_handle
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory")
                .join("vault.db");

            println!("📁 Database will be stored at: {:?}", db_path);

            // Initialize database if it doesn't exist
            if !db_path.exists() {
                println!("📦 Initializing new vault...");
                
                // Create parent directories if needed
                if let Some(parent) = db_path.parent() {
                    std::fs::create_dir_all(parent).expect("Failed to create data directory");
                }
                
                match init_db(&db_path) {
                    Ok(_) => println!("✅ Vault created successfully!"),
                    Err(e) => eprintln!("❌ Failed to create vault: {}", e),
                }
            }

            // Share database connection with commands
            let db = Arc::new(Mutex::new(rusqlite::Connection::open(&db_path).map_err(|e| format!("Failed to open DB: {}", e))?));
            
            app.manage(db);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_note, search_notes, get_note_count])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
