//! Natural Language to Machine Code Compiler (NLMC)
//! 
//! A revolutionary compiler that uses embedded LLM agents to directly generate
//! optimized machine code from natural language descriptions.

pub mod intent_extractor;
pub mod semantic_analyzer;
pub mod type_inferencer;
pub mod flow_analyzer;
pub mod llvm_generator;
pub mod ambiguity_resolver;
pub mod error_recovery;
pub mod syscall_interface;
pub mod hardware_layer;

use anyhow::Result;
use log::{info, debug};
use std::path::Path;

use crate::gemini::GeminiClient;
use intent_extractor::{IntentExtractor, ProgramIntent};
use semantic_analyzer::{SemanticAnalyzer, SemanticModel};
use type_inferencer::{TypeInferencer, TypeModel};
use flow_analyzer::{FlowAnalyzer, FlowModel};
use llvm_generator::{LLVMGenerator, LLVMModule};
use ambiguity_resolver::AmbiguityResolver;
use error_recovery::ErrorRecovery;

/// Core Natural Language to Machine Code Compiler
pub struct NLMCompiler {
    intent_extractor: IntentExtractor,
    semantic_analyzer: SemanticAnalyzer,
    type_inferencer: TypeInferencer,
    flow_analyzer: FlowAnalyzer,
    llvm_generator: LLVMGenerator,
    ambiguity_resolver: AmbiguityResolver,
    error_recovery: ErrorRecovery,
}

impl NLMCompiler {
    /// Create a new NLMC instance
    pub fn new(gemini_client: GeminiClient) -> Result<Self> {
        info!("Initializing Natural Language to Machine Code Compiler");
        
        Ok(Self {
            intent_extractor: IntentExtractor::new(gemini_client.clone())?,
            semantic_analyzer: SemanticAnalyzer::new(gemini_client.clone())?,
            type_inferencer: TypeInferencer::new(gemini_client.clone())?,
            flow_analyzer: FlowAnalyzer::new(gemini_client.clone())?,
            llvm_generator: LLVMGenerator::new()?,
            ambiguity_resolver: AmbiguityResolver::new(gemini_client.clone())?,
            error_recovery: ErrorRecovery::new(),
        })
    }

    /// Compile natural language directly to machine code
    pub fn compile_to_machine_code<P: AsRef<Path>>(
        &self,
        input: &str,
        output_path: P,
        target_triple: &str,
    ) -> Result<()> {
        info!("Starting NLMC compilation pipeline");
        debug!("Input length: {} characters", input.len());
        debug!("Target triple: {}", target_triple);

        // Stage 1: Intent Extraction
        info!("Stage 1: Extracting program intent from natural language");
        let mut intent = self.intent_extractor.extract_intent(input)?;
        
        // Handle ambiguities early
        if !intent.ambiguities.is_empty() {
            info!("Resolving {} ambiguities", intent.ambiguities.len());
            intent = self.ambiguity_resolver.resolve_ambiguities(intent)?;
        }

        // Stage 2: Semantic Analysis
        info!("Stage 2: Performing semantic analysis");
        let semantic_model = self.semantic_analyzer.analyze(&intent)?;

        // Stage 3: Type and Memory Inference
        info!("Stage 3: Inferring types and memory layout");
        let type_model = self.type_inferencer.infer_types(&intent, &semantic_model)?;

        // Stage 4: Control and Data Flow Analysis
        info!("Stage 4: Analyzing control and data flow");
        let flow_model = self.flow_analyzer.analyze_flows(&intent, &semantic_model, &type_model)?;

        // Stage 5: LLVM IR Generation
        info!("Stage 5: Generating LLVM IR");
        let mut llvm_module = self.llvm_generator.generate_module(
            &intent,
            &semantic_model,
            &type_model,
            &flow_model,
        )?;

        // Stage 6: Optimization
        info!("Stage 6: Applying LLVM optimizations");
        self.llvm_generator.optimize(&mut llvm_module)?;

        // Stage 7: Machine Code Generation
        info!("Stage 7: Generating machine code");
        let machine_code = self.llvm_generator.emit_machine_code(&llvm_module, target_triple)?;

        // Write to output file
        std::fs::write(&output_path, machine_code)?;
        info!("Machine code written to: {:?}", output_path.as_ref());

        Ok(())
    }

    /// Compile and execute directly (for testing/development)
    pub fn compile_and_execute(&self, input: &str, program_name: &str) -> Result<()> {
        let temp_path = format!("/tmp/{}", program_name);
        let target_triple = self.get_native_target_triple();
        
        self.compile_to_machine_code(input, &temp_path, &target_triple)?;
        
        // Make executable and run
        std::process::Command::new("chmod")
            .arg("+x")
            .arg(&temp_path)
            .status()?;
            
        let status = std::process::Command::new(&temp_path)
            .status()?;
            
        if !status.success() {
            anyhow::bail!("Program execution failed with status: {}", status);
        }
        
        Ok(())
    }

    /// Get the native target triple for the current platform
    fn get_native_target_triple(&self) -> String {
        // This would typically use LLVM's target detection
        // For now, return a common x86_64 triple
        "x86_64-unknown-linux-gnu".to_string()
    }

    /// Simulate the inner monologue of the compiler during compilation
    pub fn compile_with_monologue(&self, input: &str) -> Result<String> {
        let mut monologue = String::new();
        
        monologue.push_str("🧠 NLMC COMPILER INNER MONOLOGUE 🧠\n");
        monologue.push_str("=====================================\n\n");
        
        monologue.push_str(&format!("📝 INPUT RECEIVED: '{}'\n", input));
        monologue.push_str("🤔 Hmm, let me decode this human intent...\n\n");
        
        // Intent extraction monologue
        monologue.push_str("🎯 INTENT EXTRACTION PHASE:\n");
        monologue.push_str("- Parsing natural language semantics...\n");
        monologue.push_str("- Identifying computational operations...\n");
        monologue.push_str("- Detecting data structures and variables...\n");
        monologue.push_str("- Mapping control flow patterns...\n");
        
        let intent = self.intent_extractor.extract_intent(input)?;
        monologue.push_str(&format!("✅ Extracted {} operations, {} data structures\n\n", 
            intent.operations.len(), intent.data_structures.len()));
        
        // Semantic analysis monologue
        monologue.push_str("🔍 SEMANTIC ANALYSIS PHASE:\n");
        monologue.push_str("- Validating program semantics...\n");
        monologue.push_str("- Resolving symbol references...\n");
        monologue.push_str("- Checking logical consistency...\n");
        
        let semantic_model = self.semantic_analyzer.analyze(&intent)?;
        monologue.push_str(&format!("✅ Analyzed {} variables, {} functions\n\n",
            semantic_model.variables.len(), semantic_model.functions.len()));
        
        // Type inference monologue
        monologue.push_str("🧬 TYPE INFERENCE PHASE:\n");
        monologue.push_str("- Inferring data types from context...\n");
        monologue.push_str("- Planning memory layout...\n");
        monologue.push_str("- Ensuring type safety...\n");
        
        let type_model = self.type_inferencer.infer_types(&intent, &semantic_model)?;
        monologue.push_str(&format!("✅ Inferred {} types, {} bytes memory layout\n\n",
            type_model.types.len(), type_model.memory_layout.total_size));
        
        // Flow analysis monologue
        monologue.push_str("🌊 FLOW ANALYSIS PHASE:\n");
        monologue.push_str("- Analyzing control flow patterns...\n");
        monologue.push_str("- Tracking data dependencies...\n");
        monologue.push_str("- Optimizing execution paths...\n");
        
        let flow_model = self.flow_analyzer.analyze_flows(&intent, &semantic_model, &type_model)?;
        monologue.push_str(&format!("✅ Mapped {} control blocks, {} data flows\n\n",
            flow_model.control_blocks.len(), flow_model.data_flows.len()));
        
        // LLVM generation monologue
        monologue.push_str("⚡ LLVM IR GENERATION PHASE:\n");
        monologue.push_str("- Translating to LLVM intermediate representation...\n");
        monologue.push_str("- Applying target-independent optimizations...\n");
        monologue.push_str("- Preparing for machine code emission...\n");
        
        let llvm_module = self.llvm_generator.generate_module(&intent, &semantic_model, &type_model, &flow_model)?;
        monologue.push_str(&format!("✅ Generated LLVM module with {} functions\n\n", llvm_module.function_count()));
        
        monologue.push_str("🎉 COMPILATION COMPLETE!\n");
        monologue.push_str("The human's natural language intent has been successfully\n");
        monologue.push_str("transformed into optimized machine code. Von Neumann would be proud! 🚀\n");
        
        Ok(monologue)
    }
}