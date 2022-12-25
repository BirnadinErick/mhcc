#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use dotenvy::dotenv;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};
use std::env;
use sqlx::postgres::PgPoolOptions;

#[derive(serde::Serialize, serde::Deserialize)]
struct StockGet {
    stock_id: i32,
    stock_name: String,
    uprice: f32,
    quantity: i32,
    dispensers_name: String,
    date_expiry: chrono::NaiveDate
}

#[tauri::command]
async fn search_stocks(term: String) -> Vec<StockGet> {
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(
            &env::var("DATABASE_URL").unwrap()
        )
        .await.unwrap();
    
    let stocks: Vec<StockGet> = sqlx::query_as!(
        StockGet,
        r#"
SELECT
    stock_id,
    stocks.name as stock_name,
    uprice, 
    quantity, 
    date_expiry,
    dispensers.name as dispensers_name
FROM stocks
LEFT JOIN dispensers
    ON stocks.dispenser_id = dispensers.dispenser_id
WHERE stocks.search_tokens @@ plainto_tsquery($1)
ORDER BY date_expiry ASC, stock_id ASC;
        "#,
        term
    ).fetch_all(&pool).await.unwrap();
    
    stocks.into()
}

#[tauri::command(async)]
async fn get_stocks(off_set: i64) -> Vec<StockGet>{
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(
            &env::var("DATABASE_URL").unwrap()
        )
        .await.unwrap();
    
    let stocks: Vec<StockGet> = sqlx::query_as!(
        StockGet,
        r#"
SELECT
    stock_id,
    stocks.name as stock_name,
    uprice,
    quantity,
    date_expiry,
    dispensers.name as dispensers_name
FROM stocks
LEFT JOIN dispensers
    ON stocks.dispenser_id = dispensers.dispenser_id
WHERE
    date_expiry >= CURRENT_DATE + interval '+7 day' * $1
        AND
    date_expiry < CURRENT_DATE + interval '+7 day' + interval '+7 day' * $1
ORDER BY date_expiry ASC, stock_id ASC;
        "#,
        off_set as f64
    )
        .fetch_all(&pool).await.unwrap();
   
    stocks.into()
}

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
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
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
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![get_stocks, search_stocks]) 
        .run(tauri::generate_context!())
        .expect("couldn't start MHCC!");
}
