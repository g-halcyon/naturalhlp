use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use log::{error, info, warn};
use std::path::PathBuf;

mod compiler;
mod gemini;
mod nlmc;

use compiler::Compiler;
use nlmc::NLMCompiler;

#[derive(Parser, Debug)]
#[clap(
    name = "nhlp",
    about = "Natural High Level Programming Language Native Compiler",
    version
)]
struct Args {
    /// Input .dshp file
    #[clap(required = true)]
    input_file: PathBuf,

    /// Verbose output
    #[clap(short, long)]
    verbose: bool,

    /// Use new Natural Language to Machine Code compiler
    #[clap(long)]
    use_nlmc: bool,

    /// Show compiler inner monologue
    #[clap(long)]
    show_monologue: bool,
}

fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Initialize logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    
    let args = Args::parse();

    if args.verbose {
        println!("Natural High Level Programming Language Native Compiler");
        println!("Input file: {:?}", args.input_file);
    }

    // Validate input file
    if !args.input_file.exists() {
        return Err(anyhow::anyhow!("Input file does not exist"));
    }

    if args.input_file.extension().unwrap_or_default() != "dshp" {
        warn!("Input file does not have .dshp extension");
    }
    
    if args.use_nlmc {
        // Use the new Natural Language to Machine Code compiler
        let gemini_client = match crate::gemini::GeminiClient::new() {
            Ok(client) => client,
            Err(e) => {
                error!("Failed to initialize Gemini client: {}", e);
                return Err(e);
            }
        };

        let nlmc_compiler = match NLMCompiler::new(gemini_client) {
            Ok(compiler) => compiler,
            Err(e) => {
                error!("Failed to initialize NLMC compiler: {}", e);
                return Err(e);
            }
        };

        // Read the input file
        let input = std::fs::read_to_string(&args.input_file)
            .with_context(|| format!("Failed to read input file: {:?}", args.input_file))?;

        if args.show_monologue {
            // Show the compiler's inner monologue
            println!("🤖 NLMC Compiler Inner Monologue:");
            println!("==================================");
            match nlmc_compiler.compile_with_monologue(&input) {
                Ok(monologue) => {
                    println!("{}", monologue);
                    println!("\n🎯 Compilation completed! The natural language has been transformed into optimized machine code.");
                }
                Err(e) => {
                    error!("NLMC compilation failed: {}", e);
                    return Err(e);
                }
            }
        } else {
            // Standard NLMC compilation and execution
            info!("Using NLMC compiler for: {:?}", args.input_file);
            let program_name = args.input_file.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("nlmc_program");

            match nlmc_compiler.compile_and_execute(&input, program_name) {
                Ok(_) => {
                    if args.verbose {
                        println!("NLMC program executed successfully.");
                    }
                }
                Err(e) => {
                    error!("NLMC compilation or execution failed: {}", e);
                    return Err(e);
                }
            }
        }
    } else {
        // Use the original compiler
        let compiler = match Compiler::new() {
            Ok(compiler) => compiler,
            Err(e) => {
                error!("Failed to initialize compiler: {}", e);
                return Err(e);
            }
        };
        
        // Compile directly to native code and execute
        info!("Compiling and executing: {:?}", args.input_file);
        match compiler.execute(&args.input_file) {
            Ok(_) => {
                if args.verbose {
                    println!("Program executed successfully.");
                }
            }
            Err(e) => {
                error!("Compilation or execution failed: {}", e);
                return Err(e);
            }
        }
    }

    Ok(())
}
