// Prevents an additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Import our library module
mod lib;

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

// Import our verilog module from lib.rs
use lib::verilog::{self, Design};

/// Deserialize a design from a list of Verilog file paths.
#[tauri::command]
fn parse_files(file_paths: Vec<String>) -> Result<Design, String> {
    verilog::parse_verilog_files(file_paths)
}

/// Find all Verilog files in a directory
#[tauri::command]
fn find_verilog_files(directory: String) -> Result<Vec<String>, String> {
    verilog::list_verilog_files(directory)
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Main function: create the Tauri application
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            parse_files,
            find_verilog_files,
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_example() {
        // Get the path to the example.v file
        let mut example_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        example_path.push("../example.v");
        
        let file_path = example_path.to_str().unwrap().to_string();
        let result = verilog::parse_single_file(&file_path);
        
        assert!(result.is_ok(), "Failed to parse example.v: {:?}", result.err());
        
        let design = result.unwrap();
        
        // Verify we found the top-level module
        assert_eq!(design.top_instances.len(), 1);
        assert_eq!(design.top_instances[0].module_type, "cpu");
        
        // Verify the children of the cpu module
        let cpu = &design.top_instances[0];
        assert_eq!(cpu.children.len(), 3);
        
        // Find the ALU child
        let alu = cpu.children.iter().find(|c| c.module_type == "alu").unwrap();
        assert_eq!(alu.children.len(), 2); // adder and multiplier
        
        // Find the control_unit child
        let control = cpu.children.iter().find(|c| c.module_type == "control_unit").unwrap();
        assert_eq!(control.children.len(), 1); // decoder
        
        // Find the register_file child
        let regs = cpu.children.iter().find(|c| c.module_type == "register_file").unwrap();
        assert_eq!(regs.children.len(), 0); // No children
    }
    
    #[test]
    fn test_minimal_verilog() {
        // Create a temporary file with minimal Verilog content
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("minimal.v");
        
        std::fs::write(&file_path, r#"
            module top();
                sub instance1();
            endmodule
            
            module sub();
            endmodule
        "#).unwrap();
        
        let result = verilog::parse_single_file(file_path.to_str().unwrap());
        
        assert!(result.is_ok(), "Failed to parse minimal Verilog: {:?}", result.err());
        
        let design = result.unwrap();
        assert_eq!(design.top_instances.len(), 1);
        assert_eq!(design.top_instances[0].module_type, "top");
        assert_eq!(design.top_instances[0].children.len(), 1);
        assert_eq!(design.top_instances[0].children[0].module_type, "sub");
    }
}
