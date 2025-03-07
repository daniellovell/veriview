// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// Module for Verilog parsing functionality
pub mod verilog {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::path::Path;
    use sv_parser::parse_sv;

    /// Struct to represent a submodule instantiation
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Submodule {
        pub instance_name: String,
        pub module_type: String,
    }

    /// Struct to represent a node in the instance hierarchy
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InstanceNode {
        pub instance_name: String,
        pub module_type: String,
        pub complexity: u32,
        pub children: Vec<InstanceNode>,
    }

    /// Struct to represent the entire design
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Design {
        pub top_instances: Vec<InstanceNode>,
    }

    /// Given a RefNode, unwrap a SimpleIdentifier or EscapedIdentifier to get the Locate.
    pub fn get_identifier(node: sv_parser::RefNode) -> Option<sv_parser::Locate> {
        match sv_parser::unwrap_node!(node, SimpleIdentifier, EscapedIdentifier) {
            Some(sv_parser::RefNode::SimpleIdentifier(x)) => Some(x.nodes.0),
            Some(sv_parser::RefNode::EscapedIdentifier(x)) => Some(x.nodes.0),
            _ => None,
        }
    }

    /// Convert a node's identifier into a String using the SyntaxTree.
    pub fn get_identifier_str(
        syntax_tree: &sv_parser::SyntaxTree,
        node: sv_parser::RefNode,
    ) -> Option<String> {
        get_identifier(node).and_then(|loc| syntax_tree.get_str(&loc).map(|s| s.to_string()))
    }

    /// Extract the module name from a module declaration node
    pub fn extract_module_name(
        syntax_tree: &sv_parser::SyntaxTree,
        node: &sv_parser::RefNode,
    ) -> Option<String> {
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

    /// Extract a single module instantiation's instance name and module type
    pub fn extract_module_instantiation(
        syntax_tree: &sv_parser::SyntaxTree,
        node: &sv_parser::RefNode,
    ) -> Option<(String, String)> {
        if let sv_parser::RefNode::ModuleInstantiation(inst) = node {
            let mod_id = sv_parser::unwrap_node!(*inst, ModuleIdentifier)?;
            let module_type = get_identifier_str(syntax_tree, mod_id)?;

            let hi = sv_parser::unwrap_node!(*inst, HierarchicalInstance)?;
            let inst_id = sv_parser::unwrap_node!(hi, InstanceIdentifier)?;
            let instance_name = get_identifier_str(syntax_tree, inst_id)?;

            Some((instance_name, module_type))
        } else {
            None
        }
    }

    /// Process a syntax tree to extract module definitions and instantiations
    pub fn process_syntax_tree(
        syntax_tree: &sv_parser::SyntaxTree,
    ) -> HashMap<String, Vec<Submodule>> {
        let mut definitions: HashMap<String, Vec<Submodule>> = HashMap::new();
        let mut module_stack: Vec<String> = Vec::new();

        for node in syntax_tree {
            match node {
                sv_parser::RefNode::ModuleDeclarationNonansi(_)
                | sv_parser::RefNode::ModuleDeclarationAnsi(_) => {
                    if let Some(module_name) = extract_module_name(syntax_tree, &node) {
                        module_stack.push(module_name.clone());
                        definitions.entry(module_name).or_default();
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

    /// Build an instance node recursively
    pub fn build_instance_node(
        defs: &HashMap<String, Vec<Submodule>>,
        instance_name: String,
        module_type: String,
    ) -> InstanceNode {
        let children: Vec<InstanceNode> = if let Some(submodules) = defs.get(&module_type) {
            submodules
                .iter()
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

    /// Find top-level modules (those that are not instantiated by any other module)
    pub fn find_top_level_modules(defs: &HashMap<String, Vec<Submodule>>) -> Vec<String> {
        use std::collections::HashSet;

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

    /// Parse a list of Verilog files and build a design hierarchy
    pub fn parse_verilog_files(file_paths: Vec<String>) -> Result<Design, String> {
        let defines = HashMap::new();
        let includes = Vec::<String>::new();

        let mut all_defs = HashMap::new();

        for file_path in file_paths {
            let result = parse_sv(Path::new(&file_path), &defines, &includes, false, true);

            let syntax_tree = match result {
                Ok((syntax_tree, _)) => syntax_tree,
                Err(e) => return Err(format!("Failed to parse {}: {}", file_path, e)),
            };

            let defs = process_syntax_tree(&syntax_tree);

            for (module, submodules) in defs {
                all_defs
                    .entry(module)
                    .or_insert_with(Vec::new)
                    .extend(submodules);
            }
        }

        let top_modules = find_top_level_modules(&all_defs);

        let mut top_instances = Vec::new();
        for module_name in top_modules {
            top_instances.push(build_instance_node(
                &all_defs,
                module_name.clone(),
                module_name,
            ));
        }

        Ok(Design { top_instances })
    }

    /// List all Verilog files in a directory
    pub fn list_verilog_files(directory: String) -> Result<Vec<String>, String> {
        use std::fs;

        let entries = match fs::read_dir(directory) {
            Ok(entries) => entries,
            Err(e) => return Err(format!("Failed to read directory: {}", e)),
        };

        let mut verilog_files = Vec::new();

        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    if ext_str == "v" || ext_str == "sv" {
                        if let Some(path_str) = path.to_str() {
                            verilog_files.push(path_str.to_string());
                        }
                    }
                }
            }
        }

        Ok(verilog_files)
    }

    /// Function provided for testing purposes.
    /// This should only be used in tests and examples.
    #[doc(hidden)]
    pub fn parse_single_file(file_path: &str) -> Result<Design, String> {
        parse_verilog_files(vec![file_path.to_string()])
    }
}
