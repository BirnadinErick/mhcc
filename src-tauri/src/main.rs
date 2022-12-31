#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool as Pool;
use std::env;
use tauri::{
    CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use mhcc::stocks::{
    state::StocksState, 
    models::GetStock, models::AddStock,
};
use mhcc::{
    ports::*,
    adapters::PgAdapter
};

// BEGIN: tauri commands ======================================================

#[tauri::command]
async fn insert_stocks<'m>(new_stock: AddStock, pool: State<'m, Pool>) -> Result<u64, ()> {
    Ok(StocksState::insert(new_stock, &*pool).await)
}

#[tauri::command]
async fn get_stocks<'m>(off_set: i64, pool: State<'m, PgAdapter>) -> Result<Vec<GetStock>, ()> {
    Ok(PgAdapter::get_stock(&pool, off_set as f64).await)
}

#[tauri::command]
async fn update_stocks<'m>(new_stock: GetStock, pool: State<'m, Pool>) -> Result<u64, ()> {
    Ok(StocksState::update(new_stock, &*pool).await)
}

#[tauri::command]
async fn search_stocks<'m>(term: String, pool: State<'m, Pool>) -> Result<Vec<GetStock>, ()> {
    Ok(StocksState::search(term, &*pool).await)
}

// END: Tauri commands ========================================================

#[tokio::main]
async fn main() {
    dotenv().ok();

    // system try init
    let open = CustomMenuItem::new("open".to_string(), "Open App");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit App");
    let minimize = CustomMenuItem::new("minimize".to_string(), "Minimize Window");
    let maximize = CustomMenuItem::new("maximize".to_string(), "Maximize Window");
    let tray_menu = SystemTrayMenu::new()
        .add_item(open)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(minimize)
        .add_item(maximize)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await
        .expect("Failed to initiate database pool");

    tauri::Builder::default()
        .manage(pool.clone())
        .manage(PgAdapter {pool})
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "open" => {
                        let window = app.get_window("main").unwrap();
                        window.show().unwrap();
                    }
                    "maximize" => {
                        let window = app.get_window("main").unwrap();
                        window.maximize().unwrap();
                    }
                    "minimize" => {
                        let window = app.get_window("main").unwrap();
                        window.minimize().unwrap();
                    }
                    _ => {}
                };
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_stocks,
            search_stocks,
            update_stocks,
            insert_stocks
        ])
        .run(tauri::generate_context!())
        .expect("couldn't start MHCC!");
}
