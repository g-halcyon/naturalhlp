use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use log::{error, info, warn};
use std::path::PathBuf;

mod compiler;
mod gemini;

use compiler::Compiler;

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
    
    // Initialize the compiler
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
            Ok(())
        }
        Err(e) => {
            error!("Compilation or execution failed: {}", e);
            Err(e)
        }
    }
}
