use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::Manager;

#[derive(Serialize)]
struct Note {
    id: u64,
    title: String,
    content: String,
}

#[derive(Deserialize)]
struct AddNoteArgs {
    title: String,
    content: String,
}

// Global database connection (thread-safe)
lazy_static::lazy_static! {
    static ref DB: std::sync::Mutex<rusqlite::Connection> = {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, content TEXT NOT NULL)",
            [],
        ).unwrap();
        std::sync::Mutex::new(conn)
    };
}

#[tauri::command]
fn add_note(args: AddNoteArgs) -> Result<Note, String> {
    let mut conn = DB.lock().map_err(|e| e.to_string())?;
    
    let id: u64 = conn
        .query_row(
            "INSERT INTO notes (title, content) VALUES (?, ?)",
            [&args.title, &args.content],
            |row| row.get::<_, u64>(0),
        )
        .map_err(|e| e.to_string())?;

    Ok(Note { id, title: args.title, content: args.content })
}

#[tauri::command]
fn get_notes() -> Result<Vec<Note>, String> {
    let conn = DB.lock().map_err(|e| e.to_string())?;
    
    let mut stmt = conn.prepare("SELECT id, title, content FROM notes ORDER BY id DESC").map_err(|e| e.to_string())?;
    
    let notes: Result<Vec<Note>, String> = stmt.query_map([], |row| {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
        })
    }).map_err(|e| e.to_string())?.collect();

    notes
}

#[tauri::command]
fn search_notes(query: String) -> Result<Vec<Note>, String> {
    let conn = DB.lock().map_err(|e| e.to_string())?;
    
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }

    let mut stmt = conn.prepare("SELECT id, title, content FROM notes WHERE title LIKE ? OR content LIKE ? ORDER BY id DESC").map_err(|e| e.to_string())?;
    
    let search_term = format!("%{}%", query);
    
    let notes: Result<Vec<Note>, String> = stmt.query_map([search_term.clone(), search_term], |row| {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
        })
    }).map_err(|e| e.to_string())?.collect();

    notes
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_note, get_notes, search_notes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
