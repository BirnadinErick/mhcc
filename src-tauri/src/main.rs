#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use directories::ProjectDirs;
use surrealdb::{Datastore, Session};

#[tauri::command]
async fn test_db() -> Result<(), String> {
    // get local datadir

    let db = Datastore::new("http://localhost:2007").await?;
    let ses = Session::for_kv();
    let ast = "USE NS mhcc DB main; SELECT * FROM staff;";
    let res = db.execute(ast, &ses, None, false).await?;
    println!("{:?}", res);

    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hey there, {} how are you?", name)
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, test_db])
        .run(tauri::generate_context!())
        .expect("couldn't start MHCC!");
}
