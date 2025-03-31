use anyhow::Result;
use clap::{Parser, ValueEnum};
use dotenv::dotenv;
use log::{error, info, warn};
use std::path::PathBuf;

mod compiler;
mod gemini;

use compiler::{Compiler, TargetLanguage};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputLanguage {
    Cpp,
    Rust,
    Assembly,
}

impl From<OutputLanguage> for TargetLanguage {
    fn from(lang: OutputLanguage) -> Self {
        match lang {
            OutputLanguage::Cpp => TargetLanguage::Cpp,
            OutputLanguage::Rust => TargetLanguage::Rust,
            OutputLanguage::Assembly => TargetLanguage::Assembly,
        }
    }
}

#[derive(Parser, Debug)]
#[clap(
    name = "dshpc",
    about = "Superior High-Level Programming Language Compiler",
    version
)]
struct Args {
    /// Input .dshp file
    #[clap(required = true)]
    input_file: PathBuf,

    /// Output file path
    #[clap(short, long)]
    output: Option<PathBuf>,

    /// Target language
    #[clap(short, long, value_enum)]
    language: Option<OutputLanguage>,

    /// Verbose output
    #[clap(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Initialize logging
    env_logger::init();
    let args = Args::parse();

    if args.verbose {
        println!("Superior High-Level Programming Language Compiler");
        println!("Input file: {:?}", args.input_file);
        println!("Output file: {:?}", args.output);
        println!("Target language: {:?}", args.language);
    }

    // Validate input file
    if !args.input_file.exists() {
        return Err(anyhow::anyhow!("Input file does not exist"));
    }

    if args.input_file.extension().unwrap_or_default() != "dshp" {
        warn!("Input file does not have .dshp extension");
    }

    // Determine target language and output path
    let (target_language, output_path) = determine_language_and_output(&args);
    
    info!("Compiling {:?} to {:?} ({})", 
        args.input_file, 
        output_path,
        target_language.as_str()
    );

    // Initialize the compiler
    let compiler = match Compiler::new() {
        Ok(compiler) => compiler,
        Err(e) => {
            error!("Failed to initialize compiler: {}", e);
            return Err(e);
        }
    };

    // Compile the code
    match compiler.compile(&args.input_file, &output_path, target_language) {
        Ok(_) => {
            println!("Successfully compiled to {:?}", output_path);
            Ok(())
        }
        Err(e) => {
            error!("Compilation failed: {}", e);
            Err(e)
        }
    }
}

/// Determine the target language and output file path based on CLI arguments
fn determine_language_and_output(args: &Args) -> (TargetLanguage, PathBuf) {
    // Start with the provided language or default to C++
    let target_language = if let Some(lang) = args.language {
        lang.into()
    } else {
        // If language not specified but output file is, try to detect from extension
        if let Some(ref output) = args.output {
            if let Some(ext) = output.extension() {
                if let Some(lang) = TargetLanguage::from_extension(ext.to_string_lossy().as_ref()) {
                    info!("Auto-detected target language: {} from output file extension", lang.as_str());
                    lang
                } else {
                    warn!("Could not detect language from output file extension, defaulting to C++");
                    TargetLanguage::Cpp
                }
            } else {
                TargetLanguage::Cpp
            }
        } else {
            TargetLanguage::Cpp
        }
    };

    // Determine output path
    let output_path = match &args.output {
        Some(path) => path.clone(),
        None => {
            let file_stem = args.input_file.file_stem().unwrap_or_default();
            PathBuf::from(format!("{}.{}", file_stem.to_string_lossy(), target_language.extension()))
        }
    };

    (target_language, output_path)
}
