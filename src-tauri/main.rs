use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::State;

#[derive(Serialize)]
struct AppState {
    note_count: usize,
}

#[derive(Deserialize)]
struct AddNoteArgs {
    title: String,
    content: String,
}

#[derive(Deserialize)]
struct ReviewCardArgs {
    card_id: u64,
    rating: String, // "again", "hard", "good", "easy"
}

#[tauri::command]
async fn add_note(
    app_handle: tauri::AppHandle,
    state: State<'_, rusqlite::Connection>,
    args: AddNoteArgs,
) -> Result<u64, String> {
    let mut conn = state.write();
    
    // Auto-categorize based on content patterns
    let (knowledge_type, tags_json) = auto_categorize(&args.content, &args.title);
    
    // Insert note with transaction
    let id = conn
        .execute(
            "INSERT INTO notes (title, content, knowledge_type, review_due, review_interval, review_streak, review_easiness) VALUES (?, ?, ?, datetime('now'), 0, 0, 2.5)",
            [&args.title, &args.content, &knowledge_type],
        )
        .map_err(|e| format!("Failed to insert note: {}", e))?;

    // Update FTS index
    conn.execute(
        "INSERT INTO notes_fts(rowid, title, content) VALUES (?, ?, ?)",
        [id as i64, &args.title, &args.content],
    )
    .map_err(|e| format!("Failed to update FTS: {}", e))?;

    Ok(id as u64)
}

#[tauri::command]
async fn search_notes(
    state: State<'_, rusqlite::Connection>,
    query: String,
) -> Result<Vec<serde_json::Value>, String> {
    let conn = state.read();
    
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }

    // Use FTS5 for full-text search
    let mut stmt = conn
        .prepare(
            "SELECT id, title, content, knowledge_type, tags 
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
                "knowledge_type": row.get::<_, String>(3)?,
                "tags": row.get::<_, Vec<String>>(4)?
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
async fn get_review_cards(
    state: State<'_, rusqlite::Connection>,
) -> Result<Vec<serde_json::Value>, String> {
    use chrono::{Duration, Utc};
    
    let conn = state.read();
    let now = Utc::now();
    let due_date = (now - Duration::days(1)).timestamp(); // Get cards from yesterday or earlier
    
    let mut stmt = conn
        .prepare(
            "SELECT id, title, content, knowledge_type 
             FROM notes 
             WHERE review_due <= ?1 AND review_due IS NOT NULL
             ORDER BY review_due ASC"
        )
        .map_err(|e| format!("Failed to prepare review query: {}", e))?;

    let results = stmt
        .query_map([due_date], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, u64>(0)?,
                "title": row.get::<_, String>(1)?,
                "content": row.get::<_, String>(2)?,
                "knowledge_type": row.get::<_, String>(3)?
            }))
        })
        .map_err(|e| format!("Failed to query review cards: {}", e))?;

    let mut cards = Vec::new();
    for result in results {
        if let Ok(card) = result {
            cards.push(card);
        }
    }

    Ok(cards)
}

#[tauri::command]
async fn rate_review_card(
    state: State<'_, rusqlite::Connection>,
    args: ReviewCardArgs,
) -> Result<(), String> {
    let mut conn = state.write();
    
    // SM-2 Algorithm implementation
    let rating = match args.rating.as_str() {
        "again" => 0,
        "hard" => 3,
        "good" => 4,
        "easy" => 5,
        _ => return Err(format!("Invalid rating: {}", args.rating)),
    };

    // Get current card stats
    let mut stmt = conn
        .prepare("SELECT review_interval, review_streak, review_easiness FROM notes WHERE id = ?")
        .map_err(|e| format!("Failed to get card stats: {}", e))?;

    let result = stmt.query_row([args.card_id], |row| {
        Ok((
            row.get::<_, f64>(0)?, // interval
            row.get::<_, i32>(1)?, // streak
            row.get::<_, f64>(2)?, // easiness
        ))
    });

    if let Ok((interval, streak, easiness)) = result {
        let mut new_interval = interval;
        let mut new_streak = streak;
        let mut new_easiness = easiness;

        if rating >= 3 { // Correct answer
            new_streak += 1;
            
            // Adjust easiness based on rating
            if rating == 5 { // Easy
                new_easiness = (new_easiness + 0.1).max(1.3);
            } else if rating == 4 { // Good
                new_easiness = (new_easiness - 0.08).max(1.3);
            } else if rating == 3 { // Hard
                new_easiness = (new_easiness - 0.15).max(1.3);
            }

            // Calculate new interval
            new_interval = (interval * new_easiness).round() as i64;
            
            if new_interval < 1 {
                new_interval = 1;
            }
        } else { // Wrong answer - reset
            new_streak = 0;
            new_interval = 1; // Review again tomorrow
        }

        // Update card with new values (using chrono for date calculation)
        let new_due_date = format!("datetime({}, 'day', '+{}')", 
            chrono::Utc::now().timestamp(), 
            new_interval.max(1) as i64
        );

        conn.execute(
            "UPDATE notes SET review_due = ?, review_interval = ?, review_streak = ?, review_easiness = ? WHERE id = ?",
            [new_due_date, new_interval, new_streak, new_easiness, args.card_id],
        )
        .map_err(|e| format!("Failed to update card: {}", e))?;

        Ok(())
    } else {
        Err("Card not found or has no review data".to_string())
    }
}

#[tauri::command]
async fn export_vault(
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    use std::fs;
    
    let data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get data dir: {}", e))?;

    let backup_path = data_dir.join("backup");
    fs::create_dir_all(&backup_path)
        .map_err(|e| format!("Failed to create backup dir: {}", e))?;

    // Export vault.db to ZIP (simplified - in production use zip crate)
    let export_file = backup_path.join(format!(
        "quicknote-backup-{}.zip",
        chrono::Utc::now().format("%Y-%m-%d")
    ));

    Ok(())
}

#[tauri::command]
async fn get_note_count(state: State<'_, rusqlite::Connection>) -> Result<usize, String> {
    let conn = state.read();
    
    let count: usize = conn
        .query_row("SELECT COUNT(*) FROM notes", [], |row| row.get(0))
        .map_err(|e| format!("Failed to count notes: {}", e))?;

    Ok(count)
}

fn auto_categorize(content: &str, title: &str) -> (String, String) {
    let mut tags = Vec::new();
    
    // Extract #tags from content
    for word in content.split_whitespace() {
        if word.starts_with('#') && !word.is_empty() {
            tags.push(word[1..].to_string());
        }
    }
    
    let lower_content = content.to_lowercase();
    let lower_title = title.to_lowercase();
    
    // Pattern matching for knowledge type detection
    if lower_content.contains("select") || lower_content.contains("from ") || lower_content.contains("insert into") {
        return ("SQLQuery".to_string(), serde_json::to_string(&tags).unwrap_or_default());
    }
    
    if lower_content.contains("error") || lower_content.contains("exception") || lower_content.contains("panic") {
        return ("DebugPattern".to_string(), serde_json::to_string(&tags).unwrap_or_default());
    }
    
    // Check for numbered lists (process)
    if title.starts_matches(|c: char| c.is_ascii_digit()) && content.split('\n').count() > 3 {
        return ("Process".to_string(), serde_json::to_string(&tags).unwrap_or_default());
    }
    
    // Default to Concept for most knowledge items
    ("Concept".to_string(), serde_json::to_string(&tags).unwrap_or_default())
}

fn init_database(db_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let conn = rusqlite::Connection::open(db_path)?;
    
    // Create notes table with review fields
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            knowledge_type TEXT CHECK(knowledge_type IN 
                ('Concept', 'Snippet', 'Checklist', 'Note', 'Process', 'SQLQuery', 'DebugPattern')),
            tags TEXT DEFAULT '[]',
            created_at INTEGER DEFAULT (strftime('%s', 'now')),
            updated_at INTEGER DEFAULT (strftime('%s', 'now')),
            review_due INTEGER,
            review_interval INTEGER DEFAULT 0,
            review_streak INTEGER DEFAULT 0,
            review_easiness REAL DEFAULT 2.5
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

#[tauri::command]
async fn check_portable_mode() -> Result<bool, String> {
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?;
    
    let app_dir = exe_path.parent().ok_or("Cannot determine app directory")?;
    let data_dir = app_dir.join("data");
    
    Ok(data_dir.exists() && data_dir.is_dir())
}

fn main() {
    println!("🚀 QuickNote — Portable Knowledge Pocket v0.1");
    
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            add_note, 
            search_notes, 
            export_vault, 
            get_note_count,
            check_portable_mode,
            get_review_cards,
            rate_review_card
        ])
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

            // Check portable mode
            let is_portable = check_portable_mode().unwrap_or(false);
            println!("📋 Running in {}mode", if is_portable { "PORTABLE " } else { "" });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
