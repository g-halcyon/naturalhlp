use anyhow::{Context, Result};
use log::{debug, info, warn};
use std::fs;
use std::path::Path;

use crate::gemini::GeminiClient;

/// Represents the target language for compilation
#[derive(Debug, Clone, Copy)]
pub enum TargetLanguage {
    Cpp,
    Rust,
    Assembly,
}

impl TargetLanguage {
    /// Returns the file extension for this language
    pub fn extension(&self) -> &'static str {
        match self {
            TargetLanguage::Cpp => "cpp",
            TargetLanguage::Rust => "rs",
            TargetLanguage::Assembly => "asm",
        }
    }
    
    /// Returns the language name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            TargetLanguage::Cpp => "C++",
            TargetLanguage::Rust => "Rust",
            TargetLanguage::Assembly => "x86_64 Assembly",
        }
    }
    
    /// Try to detect the target language from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "cpp" | "cc" | "cxx" | "c++" | "c" | "h" | "hpp" => Some(TargetLanguage::Cpp),
            "rs" => Some(TargetLanguage::Rust),
            "asm" | "s" => Some(TargetLanguage::Assembly),
            _ => None,
        }
    }
}

/// Main compiler struct
pub struct Compiler {
    gemini_client: GeminiClient,
}

impl Compiler {
    /// Create a new compiler instance
    pub fn new() -> Result<Self> {
        let gemini_client = GeminiClient::new()?;
        Ok(Self { gemini_client })
    }

    /// Compile a .dshp file to the target language
    pub fn compile<P: AsRef<Path>>(
        &self,
        input_path: P,
        output_path: P,
        target: TargetLanguage,
    ) -> Result<()> {
        // Read the input file
        let input = fs::read_to_string(&input_path)
            .with_context(|| format!("Failed to read input file: {:?}", input_path.as_ref()))?;
        
        debug!("Read {} bytes from input file", input.len());
        
        // Process the input with Gemini API
        let prompt = self.build_prompt(&input, target);
        info!("Sending request to Gemini API for {} code generation", target.as_str());
        let generated_code = self.gemini_client.generate_code(&prompt)?;
        
        // Postprocess the code (syntax check, formatting, etc.)
        let processed_code = self.postprocess_code(&generated_code, target)?;
        
        // Write the output file
        fs::write(&output_path, processed_code)
            .with_context(|| format!("Failed to write output file: {:?}", output_path.as_ref()))?;
        
        info!("Successfully compiled to {:?}", output_path.as_ref());
        Ok(())
    }

    /// Build the prompt for Gemini API
    fn build_prompt(&self, input: &str, target: TargetLanguage) -> String {
        let target_lang = target.as_str();

        format!(
            "Convert the following natural language description into {target_lang} code.\n\
            Make sure the code is efficient, readable, and follows best practices.\n\
            Use appropriate error handling and optimizations.\n\n\
            Natural language description:\n{input}\n\n\
            {target_lang} code:"
        )
    }
    
    /// Process the generated code for syntax checking, formatting, etc.
    fn postprocess_code(&self, code: &str, target: TargetLanguage) -> Result<String> {
        // This is a placeholder for more advanced postprocessing
        // In a real implementation, we might:
        // 1. Run syntax checks
        // 2. Apply auto-formatting
        // 3. Optimize the code
        
        // For now, just add language-specific headers if they're missing
        let code = match target {
            TargetLanguage::Cpp => {
                if !code.contains("#include") {
                    // Add basic includes if they're missing
                    format!(
                        "#include <iostream>\n#include <vector>\n#include <string>\n\n{}",
                        code
                    )
                } else {
                    code.to_string()
                }
            }
            TargetLanguage::Rust => {
                if !code.contains("fn main()") {
                    // Add main function if it's missing
                    format!("fn main() {{\n    {}\n}}", code)
                } else {
                    code.to_string()
                }
            }
            TargetLanguage::Assembly => {
                if !code.contains("global") && !code.contains(".global") {
                    // Add basic x86_64 assembly structure if it's missing
                    format!(
                        "global _start\n\nsection .text\n_start:\n    {}\n    \n    ; Exit syscall\n    mov rax, 60\n    xor rdi, rdi\n    syscall\n",
                        code
                    )
                } else {
                    code.to_string()
                }
            }
        };
        
        Ok(code)
    }
} 