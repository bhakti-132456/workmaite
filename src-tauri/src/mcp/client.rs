use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::Path;

/// Basic MCP Tool Definition
#[derive(Debug, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

/// The MCP Client handles local tool execution
pub struct McpClient {
    pub tools: Vec<McpTool>,
}

impl McpClient {
    pub fn new() -> Self {
        Self {
            tools: vec![
                McpTool {
                    name: "read_file".to_string(),
                    description: "Read the contents of a local file at the given absolute path.".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "path": { "type": "string" }
                        },
                        "required": ["path"]
                    }),
                },
                McpTool {
                    name: "list_directory".to_string(),
                    description: "List files and folders in a local directory.".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "path": { "type": "string" }
                        },
                        "required": ["path"]
                    }),
                },
            ],
        }
    }

    /// Execute the requested tool locally
    pub fn execute_tool(&self, name: &str, args: serde_json::Value) -> Result<String, String> {
        match name {
            "read_file" => {
                let path_str = args.get("path")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing 'path' argument")?;
                
                fs::read_to_string(path_str).map_err(|e| format!("Failed to read file: {}", e))
            }
            "list_directory" => {
                let path_str = args.get("path")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing 'path' argument")?;
                
                let dir = Path::new(path_str);
                if !dir.is_dir() {
                    return Err("Path is not a directory".to_string());
                }

                let entries = fs::read_dir(dir).map_err(|e| e.to_string())?;
                let mut result = String::new();
                for entry in entries.flatten() {
                    let file_type = if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                        "[DIR]"
                    } else {
                        "[FILE]"
                    };
                    result.push_str(&format!("{} {}\n", file_type, entry.file_name().to_string_lossy()));
                }
                
                Ok(result.trim().to_string())
            }
            _ => Err(format!("Unknown tool: {}", name)),
        }
    }
}
