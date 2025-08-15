use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use super::models::{NewPost, Post};
use crate::schema::posts;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection() -> SqliteConnection {
    let db_path: std::path::PathBuf = std::env::current_exe()
        .expect("Failed to get current exe path")
        .parent()
        .expect("Failed to get exe directory")
        .join("db.sqlite");

    let db_path_str = db_path.to_str().expect("Invalid db path");

    SqliteConnection::establish(db_path_str)
        .unwrap_or_else(|e| panic!("Failed to connect, error: {e}", e = e))
}

pub fn run_migrations() {
    let conn = &mut establish_connection();
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}

#[tauri::command]
pub fn create_post(title: &str, content: &str) -> String {
    let conn = &mut establish_connection();

    // Assuming you have a NewPost struct defined elsewhere
    let new_post = NewPost {
        title: title,
        body: content,
    };

    println!("Creating post: {} with content: {}", title, content);

    let post_created = diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post");

    serde_json::to_string(&post_created).expect("Failed to serialize post")
}
