use anyhow::{Context, Result};
use log::{debug, info, warn};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::{Builder, NamedTempFile};
use std::time::Instant;
use std::env;

use crate::gemini::GeminiClient;

/// Represents available compilers
struct CompilerInfo {
    gcc: bool,
    clang: bool,
    rustc: bool,
}

impl CompilerInfo {
    fn new() -> Self {
        // Check for gcc
        let gcc = Command::new("gcc")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok();
        
        // Check for clang
        let clang = Command::new("clang")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok();
        
        // Check for rustc
        let rustc = Command::new("rustc")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok();
        
        Self { gcc, clang, rustc }
    }
    
    fn has_c_compiler(&self) -> bool {
        self.gcc || self.clang
    }
}

/// The NHLP native compiler
pub struct Compiler {
    gemini_client: GeminiClient,
    compilers: CompilerInfo,
}

impl Compiler {
    /// Create a new compiler instance
    pub fn new() -> Result<Self> {
        let gemini_client = GeminiClient::new()?;
        let compilers = CompilerInfo::new();
        
        // Log available compilers
        if compilers.gcc {
            info!("Found GCC compiler for machine code generation");
        }
        if compilers.clang {
            info!("Found Clang compiler for machine code generation");
        }
        if compilers.rustc {
            info!("Found Rust compiler for machine code generation");
        }
        
        if !compilers.has_c_compiler() && compilers.rustc {
            info!("No C compiler found, will use Rust for machine code generation");
        } else if !compilers.has_c_compiler() && !compilers.rustc {
            warn!("No compilers found - unable to generate machine code directly");
        }
        
        Ok(Self { gemini_client, compilers })
    }

    /// Compile a .dshp file directly to native machine code and execute it
    pub fn execute<P: AsRef<Path>>(&self, input_path: P) -> Result<()> {
        info!("Compiling NHLP directly to machine code");

        // Read the input file
        let input = fs::read_to_string(&input_path)
            .with_context(|| format!("Failed to read input file: {:?}", input_path.as_ref()))?;
        
        debug!("Read {} bytes from input file", input.len());
        
        // Extract program name for the output binary
        let program_name = input_path.as_ref()
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("nhlp_program");
        
        let start_time = Instant::now();
        
        // Determine which language to target based on available compilers
        let use_rust = !self.compilers.has_c_compiler() && self.compilers.rustc;
        
        // Send to Neural Compiler Engine for direct translation to machine code
        info!("Neural Compiler Engine: analyzing natural language semantics");
        let (binary_instructions, language) = if use_rust {
            (self.translate_to_rust_code(&input)?, "rust")
        } else {
            (self.translate_to_c_code(&input)?, "c")
        };
        
        // Create temporary source file with appropriate extension
        let source_file = create_temp_source_file(&binary_instructions, language, program_name)?;
        let source_path = source_file.path().to_path_buf();
        
        // Generate final executable
        info!("Generating native machine code");
        let executable_path = self.generate_executable(&source_path, program_name, language)?;
        
        let elapsed = start_time.elapsed();
        info!("Compilation complete in {:.2?}", elapsed);
        
        // Run the compiled binary
        info!("Running native executable: {:?}", executable_path);
        self.run_binary(&executable_path)?;
        
        Ok(())
    }
    
    /// Translate the natural language program directly to C code
    fn translate_to_c_code(&self, program_description: &str) -> Result<String> {
        let prompt = format!(
            r#"You are the NHLP compiler that translates natural language directly to machine code.

Your task is to translate the following NHLP (Natural High Level Programming Language) program:

---
NHLP PROGRAM:
{}
---

IMPORTANT: Generate complete, compilable C code that implements this program exactly as described.
Include all necessary headers and implement full interactive capabilities.
The code must be surrounded by triple backticks with the language identifier.

RESPOND ONLY WITH THE COMPLETE CODE.
"#,
            program_description
        );
        
        // Get the translated code from Gemini
        let response = self.gemini_client.execute_code(&prompt)?;
        
        // Extract the machine code instructions
        let code = extract_code_from_response(&response);
        
        Ok(code)
    }
    
    /// Translate the natural language program directly to Rust code
    fn translate_to_rust_code(&self, program_description: &str) -> Result<String> {
        let prompt = format!(
            r#"You are the NHLP compiler that translates natural language directly to machine code.

Your task is to translate the following NHLP (Natural High Level Programming Language) program:

---
NHLP PROGRAM:
{}
---

IMPORTANT: Generate complete, compilable Rust code that implements this program exactly as described.
Include all necessary crates and implement full interactive capabilities.
The code must be surrounded by triple backticks with the language identifier.
Be sure to handle user input properly and make the code robust.
Make sure the code is valid Rust that can be compiled with rustc directly.
Do not use any external crates that need to be added to Cargo.toml - use only the standard library.

RESPOND ONLY WITH THE COMPLETE RUST CODE.
"#,
            program_description
        );
        
        // Get the translated code from Gemini
        let response = self.gemini_client.execute_code(&prompt)?;
        
        // Extract the machine code instructions
        let code = extract_code_from_response(&response);
        
        Ok(code)
    }
    
    /// Generate an executable from the machine code
    fn generate_executable(&self, source_path: &Path, program_name: &str, language: &str) -> Result<String> {
        // Check if we have any compilers available
        if !self.compilers.has_c_compiler() && !self.compilers.rustc {
            return Err(anyhow::anyhow!(
                "No compilers found. Please install gcc, clang, or rustc to compile NHLP programs."
            ));
        }
        
        // Get current directory for output path
        let current_dir = env::current_dir()?;
        let output_path = current_dir.join(if cfg!(windows) {
            format!("{}.exe", program_name)
        } else {
            program_name.to_string()
        });
        
        let output_path_str = output_path.to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid output path"))?;
        
        // Compile based on language
        let compiler_result = match language {
            "rust" => {
                // Rust code
                if !self.compilers.rustc {
                    return Err(anyhow::anyhow!("Rust compiler not found"));
                }
                
                info!("Compiling Rust code to native machine code");
                Command::new("rustc")
                    .arg(source_path)
                    .arg("--crate-name")
                    .arg(program_name)
                    .arg("-o")
                    .arg(output_path_str)
                    .status()
                    .map_err(|e| anyhow::anyhow!("Rustc compiler error: {}", e))
            },
            "c" => {
                // C code
                info!("Compiling C code to native machine code");
                if self.compilers.gcc {
                    Command::new("gcc")
                        .arg(source_path)
                        .arg("-o")
                        .arg(output_path_str)
                        .status()
                        .map_err(|e| anyhow::anyhow!("GCC compiler error: {}", e))
                } else if self.compilers.clang {
                    Command::new("clang")
                        .arg(source_path)
                        .arg("-o")
                        .arg(output_path_str)
                        .status()
                        .map_err(|e| anyhow::anyhow!("Clang compiler error: {}", e))
                } else {
                    Err(anyhow::anyhow!("No C compiler found"))
                }
            },
            _ => Err(anyhow::anyhow!("Unsupported language: {}", language)),
        };
        
        // Check compilation result
        match compiler_result {
            Ok(status) if status.success() => Ok(output_path_str.to_string()),
            Ok(status) => Err(anyhow::anyhow!("Machine code compilation failed with status: {}", status)),
            Err(e) => Err(e),
        }
    }
    
    /// Run the binary executable
    fn run_binary(&self, path: &str) -> Result<()> {
        let status = Command::new(path)
            .status()
            .with_context(|| format!("Failed to execute the compiled program: {}", path))?;
        
        if !status.success() {
            warn!("Program exited with non-zero status: {}", status);
        }
        
        Ok(())
    }
}

/// Create a temporary source file with the appropriate extension
fn create_temp_source_file(code: &str, language: &str, program_name: &str) -> Result<NamedTempFile> {
    let extension = match language {
        "c" => ".c",
        "rust" => ".rs",
        _ => ".c",  // Default to C
    };
    
    // Create a temporary file with the right extension
    let file = Builder::new()
        .prefix(&format!("{}_", program_name))
        .suffix(extension)
        .tempfile()?;
    
    // Write the code to the file
    file.as_file().write_all(code.as_bytes())?;
    
    Ok(file)
}

/// Extract machine code from the neural compiler response
fn extract_code_from_response(response: &str) -> String {
    // Find code block between triple backticks
    if let Some(start) = response.find("```") {
        if let Some(end) = response[start + 3..].find("```") {
            let block = &response[start + 3..start + 3 + end];
            
            // Remove language identifier if present (e.g., ```c)
            if let Some(newline) = block.find('\n') {
                return block[newline + 1..].trim().to_string();
            }
            return block.trim().to_string();
        }
    }
    
    // If no triple backticks, return the whole response
    response.to_string()
} 