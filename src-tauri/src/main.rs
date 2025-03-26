// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::command;
use std::process::Command;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

#[command]
fn run_adb_check() -> serde_json::Value {
    // Ensure ADB server is running
    if !is_adb_running() {
        start_adb_server();
        // Give some time for ADB to start (5 seconds)
        sleep(Duration::from_secs(5));
    }

    let commands = vec![
        ("Model", "getprop ro.product.model"),
        ("Manufacturer", "getprop ro.product.manufacturer"),
        ("CPU", "cat /proc/cpuinfo | grep -E 'Hardware|Processor'"),
        ("GPU", "dumpsys SurfaceFlinger | grep GLES"),
        ("RAM", "cat /proc/meminfo | grep MemTotal"),
        ("Storage", "df -h | grep /data"),
        ("Display", "wm size"),
        ("Battery", "dumpsys battery | grep level"),
        ("IMEI", "service call iphonesubinfo 1 | cut -c 52-66 | tr -d '.[:space:]'"),
        ("Sensors", "dumpsys sensorservice | grep 'Sensor'"),
        ("RootAccess", "which su || echo 'No root access detected.'")
    ];

    let mut results = HashMap::new();
    for (label, cmd) in commands {
        let output = Command::new("adb")
            .args(["shell", cmd])
            .output();
        
        match output {
            Ok(out) => {
                let formatted_output = String::from_utf8_lossy(&out.stdout)
                    .trim()
                    .replace("\n", "\n    "); // Indent multiline output
                results.insert(label, formatted_output);
            },
            Err(_) => {
                results.insert(label, "Failed to retrieve".to_string());
            }
        }
    }
    
    serde_json::to_value(results).unwrap()
}

fn is_adb_running() -> bool {
    let output = Command::new("adb")
        .arg("get-state")
        .output();

    match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).trim() == "device",
        Err(_) => false,
    }
}

fn start_adb_server() {
    let _ = Command::new("adb")
        .arg("start-server")
        .output();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_adb_check])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}