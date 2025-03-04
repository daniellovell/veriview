// Service to handle Verilog file operations via Tauri API
import { invoke } from "@tauri-apps/api/core";
import { open as openDialog } from "@tauri-apps/plugin-dialog";

// Get design from current directory
export async function fetchDesignFromCurrentDir() {
  try {
    console.log("Fetching design from current directory...");
    const data = await invoke("parse_verilog_files", { filePaths: [] });
    console.log("Received design:", data);
    return data;
  } catch (err) {
    console.error("Error invoking parse_verilog_files:", err);
    throw err;
  }
}

// Open file dialog and parse selected Verilog files
export async function openAndParseVerilogFiles() {
  try {
    // Open the Tauri v2 dialog
    const selected = await openDialog({
      multiple: true,
      filters: [
        { name: "Verilog Files", extensions: ["v", "sv"] },
      ],
    });

    // Check if user canceled or not
    if (!selected) {
      return null;
    }
    
    // The dialog can return a single path or array of paths
    const filePaths = Array.isArray(selected) ? selected : [selected];
    const data = await invoke("parse_verilog_files", { filePaths });
    return data;
  } catch (err) {
    console.error("Error opening files:", err);
    throw err;
  }
} 