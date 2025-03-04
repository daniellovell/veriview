// Prevents an additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri_plugin_opener::OpenerExt;

/// A submodule instantiation from within a module definition.
#[derive(Debug, Serialize, Deserialize)]
struct Submodule {
    instance_name: String,
    module_type: String,
}

/// An instance node for the design hierarchy.
#[derive(Debug, Serialize, Deserialize)]
struct InstanceNode {
    instance_name: String,
    module_type: String,
    complexity: u32,
    children: Vec<InstanceNode>,
}

/// The overall design: a list of top-level instances.
#[derive(Debug, Serialize, Deserialize)]
struct Design {
    top_instances: Vec<InstanceNode>,
}

/// Given a RefNode, unwrap a SimpleIdentifier or EscapedIdentifier to get the Locate.
fn get_identifier(node: sv_parser::RefNode) -> Option<sv_parser::Locate> {
    match sv_parser::unwrap_node!(node, SimpleIdentifier, EscapedIdentifier) {
        Some(sv_parser::RefNode::SimpleIdentifier(x)) => Some(x.nodes.0),
        Some(sv_parser::RefNode::EscapedIdentifier(x)) => Some(x.nodes.0),
        _ => None,
    }
}

/// Convert a node’s identifier into a String using the SyntaxTree.
fn get_identifier_str(
    syntax_tree: &sv_parser::SyntaxTree,
    node: sv_parser::RefNode,
) -> Option<String> {
    get_identifier(node).and_then(|loc| syntax_tree.get_str(&loc).map(|s| s.to_string()))
}

/// Extract the module name from a module declaration node (ANSI or non-ANSI).
fn extract_module_name(
    syntax_tree: &sv_parser::SyntaxTree,
    node: &sv_parser::RefNode,
) -> Option<String> {
    // The issue was that we were trying to bind the same variable name 'decl' to two different types
    // in a pattern match using the | (or) operator. When using |, all bindings must have the same type.
    //
    // The original code tried to bind 'decl' as:
    // &&ModuleDeclarationNonansi in the first case
    // &&ModuleDeclarationAnsi in the second case
    // These are different types, so Rust complained.
    //
    // To fix this, we need to handle each case separately rather than trying to combine them with |.
    // This way each match arm can work with its specific type.
    match node {
        sv_parser::RefNode::ModuleDeclarationNonansi(nonansi_decl) => {
            let mod_id = sv_parser::unwrap_node!(*nonansi_decl, ModuleIdentifier)?;
            get_identifier_str(syntax_tree, mod_id)
        }
        sv_parser::RefNode::ModuleDeclarationAnsi(ansi_decl) => {
            let mod_id = sv_parser::unwrap_node!(*ansi_decl, ModuleIdentifier)?;
            get_identifier_str(syntax_tree, mod_id)
        }
        _ => None,
    }
}

/// Extract a single module instantiation's instance name and module type.
///
/// # Arguments
/// * `syntax_tree` - Reference to the SystemVerilog syntax tree (like an AST)
/// * `node` - Reference to a node in the syntax tree we want to examine
///
/// # Returns
/// * `Option<(String, String)>` - If successful, returns (instance_name, module_type) as a tuple.
///                                Returns None if this isn't a module instantiation or parsing fails.
///
/// # How this works (for C++ devs):
/// This is similar to a function that would take an AST node and try to extract module instance info.
/// The '?' operator is like a try-catch that returns None on failure - similar to std::optional.
/// 'if let' is like a switch/case that also does pattern matching and destructuring.
fn extract_module_instantiation(
    syntax_tree: &sv_parser::SyntaxTree,
    node: &sv_parser::RefNode,
) -> Option<(String, String)> {
    // Pattern match on the node type - only proceed if it's a ModuleInstantiation
    // 'inst' is bound to the inner ModuleInstantiation value if matched
    if let sv_parser::RefNode::ModuleInstantiation(inst) = node {
        // Extract the module type identifier (what kind of module this is)
        // unwrap_node! is a macro that tries to extract a specific node type
        // The '?' means "return None if this fails" (like std::optional::value_or)
        let mod_id = sv_parser::unwrap_node!(*inst, ModuleIdentifier)?;

        // Convert the identifier node into an actual string
        let module_type = get_identifier_str(syntax_tree, mod_id)?;

        // Get the hierarchical instance node which contains the instance name
        let hi = sv_parser::unwrap_node!(*inst, HierarchicalInstance)?;

        // Try multiple ways to get the instance identifier, using or_else() for fallbacks
        // This is like: if (try_first()) use_that; else if (try_second()) use_that; etc.
        let inst_id = sv_parser::unwrap_node!(hi, InstanceIdentifier)?;

        // Convert the instance identifier to a string
        let instance_name = get_identifier_str(syntax_tree, inst_id)?;

        // Return both strings in a tuple wrapped in Some()
        // This is like returning std::optional<std::pair<std::string, std::string>>
        Some((instance_name, module_type))
    } else {
        // Not a module instantiation node, return None (like std::nullopt)
        None
    }
}

/// Process the syntax tree to build a mapping of module definitions.
/// Key: module name (the definition); Value: list of submodule instantiations inside that module.
fn process_syntax_tree(syntax_tree: &sv_parser::SyntaxTree) -> HashMap<String, Vec<Submodule>> {
    let mut definitions: HashMap<String, Vec<Submodule>> = HashMap::new();
    // Track the current module context.
    let mut module_stack: Vec<String> = Vec::new();

    for node in syntax_tree {
        match node {
            sv_parser::RefNode::ModuleDeclarationNonansi(_)
            | sv_parser::RefNode::ModuleDeclarationAnsi(_) => {
                if let Some(module_name) = extract_module_name(syntax_tree, &node) {
                    module_stack.push(module_name.clone());
                    definitions.entry(module_name).or_insert(Vec::new());
                }
            }
            sv_parser::RefNode::ModuleInstantiation(_) => {
                if let Some(parent_module) = module_stack.last() {
                    if let Some((instance_name, module_type)) =
                        extract_module_instantiation(syntax_tree, &node)
                    {
                        if let Some(subs) = definitions.get_mut(parent_module) {
                            subs.push(Submodule {
                                instance_name,
                                module_type,
                            });
                        }
                    }
                }
            }
            _ => {
                // When an "endmodule" is encountered, pop the current module.
                let node_str = format!("{:?}", node);
                if node_str.contains("Endmodule") {
                    module_stack.pop();
                }
            }
        }
    }
    definitions
}

/// Recursively build an instance node from a given module definition.
/// The instance's children come from the submodules defined in the module.
fn build_instance_node(
    defs: &HashMap<String, Vec<Submodule>>,
    instance_name: String,
    module_type: String,
) -> InstanceNode {
    let children = if let Some(subs) = defs.get(&module_type) {
        subs.iter()
            .map(|sub| {
                build_instance_node(defs, sub.instance_name.clone(), sub.module_type.clone())
            })
            .collect()
    } else {
        Vec::new()
    };
    InstanceNode {
        instance_name,
        module_type,
        complexity: (children.len() as u32) + 1,
        children,
    }
}

/// Find top-level module definitions – those that are never instantiated as submodules.
fn find_top_level_modules(defs: &HashMap<String, Vec<Submodule>>) -> Vec<String> {
    let mut instantiated = HashSet::new();
    for subs in defs.values() {
        for sub in subs {
            instantiated.insert(sub.module_type.clone());
        }
    }
    defs.keys()
        .filter(|k| !instantiated.contains(*k))
        .cloned()
        .collect()
}

/// Tauri command: parse Verilog files (or, if none provided, all .v/.sv files in the current directory)
/// and return the design as an instantiation tree.
#[tauri::command]
fn parse_verilog_files(file_paths: Vec<String>) -> Result<Design, String> {
    // Determine which files to parse.
    let paths_to_parse = if file_paths.is_empty() {
        let current_dir = std::env::current_dir().map_err(|e| e.to_string())?;
        let mut result = Vec::new();
        for entry in fs::read_dir(current_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext_lower = ext.to_string_lossy().to_lowercase();
                    if ext_lower == "v" || ext_lower == "sv" {
                        result.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
        result
    } else {
        file_paths
    };

    let mut definitions = HashMap::new();

    let defines = HashMap::new();
    let includes: Vec<PathBuf> = Vec::new();

    // Parse each file and merge the module definitions.
    for path_str in paths_to_parse {
        let path = PathBuf::from(&path_str);
        if !path.exists() {
            return Err(format!("File {} does not exist.", path.display()));
        }
        let parse_result = sv_parser::parse_sv(&path, &defines, &includes, false, false);
        match parse_result {
            Ok((syntax_tree, _)) => {
                let defs = process_syntax_tree(&syntax_tree);
                for (mod_name, subs) in defs {
                    definitions
                        .entry(mod_name)
                        .or_insert(Vec::new())
                        .extend(subs);
                }
            }
            Err(e) => {
                return Err(format!("Failed to parse file {}: {:?}", path.display(), e));
            }
        }
    }

    // Identify top-level modules (those never instantiated inside another module).
    let top_modules = find_top_level_modules(&definitions);
    let top_instances = top_modules
        .into_iter()
        .map(|mod_name| build_instance_node(&definitions, mod_name.clone(), mod_name))
        .collect();

    Ok(Design { top_instances })
}

/// (Optional) Command to list Verilog files in a given directory.
#[tauri::command]
fn list_verilog_files(directory: String) -> Result<Vec<String>, String> {
    let dir_path = PathBuf::from(directory);
    if !dir_path.is_dir() {
        return Err(format!("{} is not a directory.", dir_path.display()));
    }

    let mut result = Vec::new();
    for entry in fs::read_dir(dir_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_lower = ext.to_string_lossy().to_lowercase();
                if ext_lower == "v" || ext_lower == "sv" {
                    result.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(result)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            parse_verilog_files,
            list_verilog_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
