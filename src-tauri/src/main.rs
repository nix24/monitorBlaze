// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//use chrono
use chrono::{Local, Timelike};
use sysinfo::{Disks, System};

#[tauri::command]
fn get_greeting() -> String {
    //based on the current time,  return morning, afternoon or evening
    //morning is 12am - 11:59am, afternoon is 12pm - 4:59pm, evening is 5pm - 11:59pm
    let current_time = Local::now();
    let current_hour = current_time.hour();
    let greeting = if current_hour < 12 {
        "morning"
    } else if current_hour < 17 {
        "afternoon"
    } else {
        "evening"
    };
    format!("Good {}!", greeting)
}

#[tauri::command]
fn get_sys_metrics() -> serde_json::Value {
    //getting cpu, ram, and storage usage as a percentage
    let mut system = System::new_all();
    let disks = Disks::new_with_refreshed_list(); // Get updated disk information
    system.refresh_all();
    let cpu_usage = system.global_cpu_info().cpu_usage();
    let total_memory = system.total_memory();
    let used_memory = system.used_memory();
    let ram_usage = (used_memory as f64 / total_memory as f64) * 100.0;

    let mut total_space: u64 = 0;
    let mut used_space: u64 = 0;

    for disk in &disks {
        total_space += disk.total_space();
        used_space += disk.total_space() - disk.available_space();
    }

    let storage_usage = (used_space as f64 / total_space as f64) * 100.0;

    serde_json::json!({
        "labels": ["CPU", "RAM", "Storage"],
        "datasets": [{
            "label": "Usage",
            "data": [cpu_usage, ram_usage, storage_usage],
            "backgroundColor": ["rgba(255, 99, 132, 0.2)", "rgba(54, 162, 235, 0.2)", "rgba(255, 206, 86, 0.2)"],
            "borderWidth": 1,
            "borderColor": ["rgba(255, 99, 132, 1)", "rgba(54, 162, 235, 1)", "rgba(255, 206, 86, 1)"],
        }]
    })
}

#[tauri::command]
fn calculate_sys_performance(
    cpu_usage: f64,
    ram_usage: f64,
    storage_usage: f64,
) -> serde_json::Value {
    //calcing avg
    let weight_cpu = 0.4;
    let weight_ram = 0.3;
    let weight_storage = 0.3;

    // Calculate the weighted sum
    let weighted_sum =
        (weight_cpu * cpu_usage) + (weight_ram * ram_usage) + (weight_storage * storage_usage);
    let mut avg = weighted_sum.round();
    if avg > 100.0 {
        avg = 100.0;
    }
    //if optimal, return percent and green, percent and yellow for warning and percent and red for danger
    if avg >= 75.0 {
        serde_json::json!({
            "percent": avg,
            "color": "green"
        })
    } else if avg >= 50.0 {
        serde_json::json!({
            "percent": avg,
            "color": "yellow"
        })
    } else {
        serde_json::json!({
            "percent": avg,
            "color": "red"
        })
    }
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_greeting,
            get_sys_metrics,
            calculate_sys_performance
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
