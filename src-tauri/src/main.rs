#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use dotenvy::dotenv;
use std::env;
use tauri::{
    CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use mhcc::stocks::{
    models::GetStock, models::AddStock,
};
use mhcc::patients::{
    models::GetPatient, models::AddPatient,
};
use mhcc::{
    ports::*,
    adapters::PgAdapter
};

// BEGIN: tauri commands ======================================================

#[tauri::command]
async fn insert_stocks<'m>(new_stock: AddStock, pool: State<'m, PgAdapter>) -> Result<u64, ()> {
    Ok(PgAdapter::add_stock(&pool, &new_stock).await)
}

#[tauri::command]
async fn get_stocks<'m>(off_set: i64, pool: State<'m, PgAdapter>) -> Result<Vec<GetStock>, ()> {
    Ok(PgAdapter::get_stocks(&pool, off_set as f64).await)
}

#[tauri::command]
async fn update_stocks<'m>(new_stock: GetStock, pool: State<'m, PgAdapter>) -> Result<u64, ()> {
    Ok(PgAdapter::update_stock(&pool, new_stock).await)
}

#[tauri::command]
async fn search_stocks<'m>(query: String, pool: State<'m, PgAdapter>) -> Result<Vec<GetStock>, ()> {
    Ok(PgAdapter::search_stock(&pool, query).await)
}

#[tauri::command]
async fn delete_stock<'m>(id: i64, pool: State<'m, PgAdapter>) -> Result<bool, ()> {
    Ok(PgAdapter::delete_stock(&pool, id).await)
}

// -----------------------------------------------------------------------------

#[tauri::command]
async fn insert_patient<'m>(new_patient: AddPatient, pool: State<'m, PgAdapter>) -> Result<u64, ()> {
    Ok(PgAdapter::add_patient(&pool, new_patient).await)
}

#[tauri::command]
async fn get_patients<'m>(off_set: i64, pool: State<'m, PgAdapter>) -> Result<Vec<GetPatient>, ()> {
    Ok(PgAdapter::get_patients(&pool, off_set as f64).await)
}

#[tauri::command]
async fn update_patient<'m>(new_patient: GetPatient, pool: State<'m, PgAdapter>) -> Result<u64, ()> {
    Ok(PgAdapter::update_patient(&pool, new_patient).await)
}

#[tauri::command]
async fn search_patients<'m>(query: String, pool: State<'m, PgAdapter>) -> Result<Vec<GetPatient>, ()> {
    Ok(PgAdapter::search_patient_by_name(&pool, query).await)
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

    tauri::Builder::default()
        .manage(PgAdapter::new(
			&env::var("DATABASE_URL")
				.expect("env var DATABASE_URL not specified"),
			env::var("MAX_CONN")
				.expect("env var MAX_CONN not specified")
				.parse::<u32>()
				.expect("MAX_CONN could not be parsed to u32")
		).await)
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
            insert_stocks,
			delete_stock,
			get_patients,
            search_patients,
            update_patient,
            insert_patient
        ])
        .run(tauri::generate_context!())
        .expect("couldn't start MHCC!");
}
