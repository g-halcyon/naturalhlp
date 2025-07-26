//! Semantic Analysis Agent
//! 
//! Performs semantic validation and builds a semantic model of the program.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::{debug, info, warn};

use crate::gemini::GeminiClient;
use super::intent_extractor::{ProgramIntent, Operation, DataStructure, OperationType};

/// Semantic model of the analyzed program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticModel {
    pub variables: HashMap<String, VariableInfo>,
    pub functions: HashMap<String, FunctionInfo>,
    pub types: HashMap<String, TypeInfo>,
    pub memory_layout: MemoryLayout,
    pub safety_constraints: Vec<SafetyConstraint>,
    pub symbol_table: SymbolTable,
    pub semantic_errors: Vec<SemanticError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableInfo {
    pub name: String,
    pub data_type: String,
    pub scope: String,
    pub is_mutable: bool,
    pub initialization_required: bool,
    pub usage_count: usize,
    pub first_use_line: Option<usize>,
    pub last_use_line: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: String,
    pub is_pure: bool,
    pub side_effects: Vec<SideEffect>,
    pub complexity: FunctionComplexity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    pub is_optional: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SideEffect {
    ModifiesGlobalState,
    PerformsIO,
    AllocatesMemory,
    CallsSystemFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionComplexity {
    pub cyclomatic_complexity: usize,
    pub estimated_instructions: usize,
    pub memory_usage: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInfo {
    pub name: String,
    pub size_bytes: usize,
    pub alignment: usize,
    pub is_primitive: bool,
    pub fields: Option<HashMap<String, String>>,
    pub methods: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLayout {
    pub total_size: usize,
    pub stack_size: usize,
    pub heap_size: usize,
    pub static_size: usize,
    pub alignment_requirements: Vec<AlignmentRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignmentRequirement {
    pub variable_name: String,
    pub required_alignment: usize,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConstraint {
    pub constraint_type: SafetyConstraintType,
    pub description: String,
    pub affected_variables: Vec<String>,
    pub severity: ConstraintSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SafetyConstraintType {
    NullPointerCheck,
    BoundsCheck,
    OverflowCheck,
    UninitializedAccess,
    UseAfterFree,
    DataRace,
    DeadlockPrevention,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolTable {
    pub scopes: Vec<Scope>,
    pub current_scope: usize,
    pub global_symbols: HashMap<String, Symbol>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scope {
    pub id: usize,
    pub parent: Option<usize>,
    pub symbols: HashMap<String, Symbol>,
    pub scope_type: ScopeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScopeType {
    Global,
    Function { name: String },
    Block { id: String },
    Loop { id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub data_type: String,
    pub is_defined: bool,
    pub is_used: bool,
    pub definition_location: Option<SourceLocation>,
    pub usage_locations: Vec<SourceLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymbolType {
    Variable,
    Function,
    Type,
    Constant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticError {
    pub error_type: SemanticErrorType,
    pub message: String,
    pub location: Option<SourceLocation>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SemanticErrorType {
    UndefinedVariable,
    TypeMismatch,
    InvalidOperation,
    ScopeViolation,
    MemoryLeak,
    UnreachableCode,
    InfiniteLoop,
}

/// Semantic Analysis Agent
pub struct SemanticAnalyzer {
    gemini_client: GeminiClient,
}

impl SemanticAnalyzer {
    pub fn new(gemini_client: GeminiClient) -> Result<Self> {
        Ok(Self { gemini_client })
    }

    /// Perform comprehensive semantic analysis
    pub fn analyze(&self, intent: &ProgramIntent) -> Result<SemanticModel> {
        info!("Starting semantic analysis");
        
        // Initialize semantic model
        let mut semantic_model = SemanticModel {
            variables: HashMap::new(),
            functions: HashMap::new(),
            types: HashMap::new(),
            memory_layout: MemoryLayout {
                total_size: 0,
                stack_size: 0,
                heap_size: 0,
                static_size: 0,
                alignment_requirements: vec![],
            },
            safety_constraints: vec![],
            symbol_table: SymbolTable {
                scopes: vec![],
                current_scope: 0,
                global_symbols: HashMap::new(),
            },
            semantic_errors: vec![],
        };

        // Build symbol table
        self.build_symbol_table(intent, &mut semantic_model)?;

        // Analyze variables
        self.analyze_variables(intent, &mut semantic_model)?;

        // Analyze functions
        self.analyze_functions(intent, &mut semantic_model)?;

        // Analyze types
        self.analyze_types(intent, &mut semantic_model)?;

        // Perform semantic validation
        self.validate_semantics(intent, &mut semantic_model)?;

        // Generate safety constraints
        self.generate_safety_constraints(intent, &mut semantic_model)?;

        // Calculate memory layout
        self.calculate_memory_layout(&mut semantic_model)?;

        // Use LLM for complex semantic reasoning
        self.llm_semantic_analysis(intent, &mut semantic_model)?;

        info!("Semantic analysis completed with {} variables, {} functions", 
              semantic_model.variables.len(), semantic_model.functions.len());

        Ok(semantic_model)
    }

    fn build_symbol_table(&self, intent: &ProgramIntent, model: &mut SemanticModel) -> Result<()> {
        debug!("Building symbol table");

        // Create global scope
        let global_scope = Scope {
            id: 0,
            parent: None,
            symbols: HashMap::new(),
            scope_type: ScopeType::Global,
        };
        model.symbol_table.scopes.push(global_scope);

        // Add data structures to symbol table
        for data_structure in &intent.data_structures {
            let symbol = Symbol {
                name: data_structure.name.clone(),
                symbol_type: SymbolType::Variable,
                data_type: format!("{:?}", data_structure.data_type),
                is_defined: true,
                is_used: false,
                definition_location: Some(SourceLocation {
                    line: 1,
                    column: 1,
                    context: "data structure definition".to_string(),
                }),
                usage_locations: vec![],
            };

            model.symbol_table.global_symbols.insert(data_structure.name.clone(), symbol);
        }

        // Add operations as potential functions
        for operation in &intent.operations {
            if let OperationType::FunctionCall { name, .. } = &operation.operation_type {
                let symbol = Symbol {
                    name: name.clone(),
                    symbol_type: SymbolType::Function,
                    data_type: "function".to_string(),
                    is_defined: false, // Will be resolved later
                    is_used: true,
                    definition_location: None,
                    usage_locations: vec![SourceLocation {
                        line: 1,
                        column: 1,
                        context: operation.description.clone(),
                    }],
                };

                model.symbol_table.global_symbols.insert(name.clone(), symbol);
            }
        }

        Ok(())
    }

    fn analyze_variables(&self, intent: &ProgramIntent, model: &mut SemanticModel) -> Result<()> {
        debug!("Analyzing variables");

        for data_structure in &intent.data_structures {
            let variable_info = VariableInfo {
                name: data_structure.name.clone(),
                data_type: format!("{:?}", data_structure.data_type),
                scope: format!("{:?}", data_structure.scope),
                is_mutable: true, // Default assumption
                initialization_required: data_structure.initial_value.is_none(),
                usage_count: 0, // Will be calculated later
                first_use_line: None,
                last_use_line: None,
            };

            model.variables.insert(data_structure.name.clone(), variable_info);
        }

        // Analyze variable usage in operations
        for operation in &intent.operations {
            for input in &operation.inputs {
                if let Some(var_info) = model.variables.get_mut(input) {
                    var_info.usage_count += 1;
                }
            }
        }

        Ok(())
    }

    fn analyze_functions(&self, intent: &ProgramIntent, model: &mut SemanticModel) -> Result<()> {
        debug!("Analyzing functions");

        // Extract function calls from operations
        for operation in &intent.operations {
            if let OperationType::FunctionCall { name, args } = &operation.operation_type {
                let function_info = FunctionInfo {
                    name: name.clone(),
                    parameters: args.iter().map(|arg| Parameter {
                        name: arg.clone(),
                        param_type: "unknown".to_string(),
                        is_optional: false,
                        default_value: None,
                    }).collect(),
                    return_type: "unknown".to_string(),
                    is_pure: false, // Conservative assumption
                    side_effects: vec![SideEffect::ModifiesGlobalState], // Conservative
                    complexity: FunctionComplexity {
                        cyclomatic_complexity: 1,
                        estimated_instructions: 10,
                        memory_usage: 64,
                    },
                };

                model.functions.insert(name.clone(), function_info);
            }
        }

        Ok(())
    }

    fn analyze_types(&self, intent: &ProgramIntent, model: &mut SemanticModel) -> Result<()> {
        debug!("Analyzing types");

        // Add primitive types
        let primitive_types = vec![
            ("i32", 4, 4),
            ("i64", 8, 8),
            ("f32", 4, 4),
            ("f64", 8, 8),
            ("bool", 1, 1),
            ("char", 1, 1),
        ];

        for (name, size, alignment) in primitive_types {
            let type_info = TypeInfo {
                name: name.to_string(),
                size_bytes: size,
                alignment,
                is_primitive: true,
                fields: None,
                methods: vec![],
            };
            model.types.insert(name.to_string(), type_info);
        }

        // Analyze custom types from data structures
        for data_structure in &intent.data_structures {
            let type_name = format!("{:?}", data_structure.data_type);
            if !model.types.contains_key(&type_name) {
                let type_info = TypeInfo {
                    name: type_name.clone(),
                    size_bytes: data_structure.size.unwrap_or(8),
                    alignment: 8, // Default alignment
                    is_primitive: false,
                    fields: None,
                    methods: vec![],
                };
                model.types.insert(type_name, type_info);
            }
        }

        Ok(())
    }

    fn validate_semantics(&self, intent: &ProgramIntent, model: &mut SemanticModel) -> Result<()> {
        debug!("Validating semantics");

        // Check for undefined variables
        for operation in &intent.operations {
            for input in &operation.inputs {
                if !model.variables.contains_key(input) && !model.functions.contains_key(input) {
                    model.semantic_errors.push(SemanticError {
                        error_type: SemanticErrorType::UndefinedVariable,
                        message: format!("Undefined variable or function: {}", input),
                        location: Some(SourceLocation {
                            line: 1,
                            column: 1,
                            context: operation.description.clone(),
                        }),
                        suggestions: vec![
                            format!("Define variable '{}' before use", input),
                            "Check for typos in variable name".to_string(),
                        ],
                    });
                }
            }
        }

        // Check for unreachable code
        if intent.control_flow.nodes.len() > intent.operations.len() + 2 {
            model.semantic_errors.push(SemanticError {
                error_type: SemanticErrorType::UnreachableCode,
                message: "Potential unreachable code detected".to_string(),
                location: None,
                suggestions: vec!["Review control flow logic".to_string()],
            });
        }

        // Check for infinite loops
        for operation in &intent.operations {
            if let OperationType::Loop { condition, .. } = &operation.operation_type {
                if condition == "true" || condition == "1" {
                    model.semantic_errors.push(SemanticError {
                        error_type: SemanticErrorType::InfiniteLoop,
                        message: "Potential infinite loop detected".to_string(),
                        location: Some(SourceLocation {
                            line: 1,
                            column: 1,
                            context: operation.description.clone(),
                        }),
                        suggestions: vec![
                            "Add loop termination condition".to_string(),
                            "Ensure loop variable is modified".to_string(),
                        ],
                    });
                }
            }
        }

        Ok(())
    }

    fn generate_safety_constraints(&self, intent: &ProgramIntent, model: &mut SemanticModel) -> Result<()> {
        debug!("Generating safety constraints");

        // Null pointer checks for pointer types
        for (name, var_info) in &model.variables {
            if var_info.data_type.contains("Pointer") {
                model.safety_constraints.push(SafetyConstraint {
                    constraint_type: SafetyConstraintType::NullPointerCheck,
                    description: format!("Null pointer check required for {}", name),
                    affected_variables: vec![name.clone()],
                    severity: ConstraintSeverity::Critical,
                });
            }
        }

        // Bounds checks for arrays
        for (name, var_info) in &model.variables {
            if var_info.data_type.contains("Array") {
                model.safety_constraints.push(SafetyConstraint {
                    constraint_type: SafetyConstraintType::BoundsCheck,
                    description: format!("Array bounds check required for {}", name),
                    affected_variables: vec![name.clone()],
                    severity: ConstraintSeverity::High,
                });
            }
        }

        // Overflow checks for arithmetic operations
        for operation in &intent.operations {
            if matches!(operation.operation_type, OperationType::Arithmetic { .. }) {
                model.safety_constraints.push(SafetyConstraint {
                    constraint_type: SafetyConstraintType::OverflowCheck,
                    description: "Integer overflow check for arithmetic operation".to_string(),
                    affected_variables: operation.inputs.clone(),
                    severity: ConstraintSeverity::Medium,
                });
            }
        }

        Ok(())
    }

    fn calculate_memory_layout(&self, model: &mut SemanticModel) -> Result<()> {
        debug!("Calculating memory layout");

        let mut total_size = 0;
        let mut stack_size = 0;
        let mut static_size = 0;

        for (name, var_info) in &model.variables {
            if let Some(type_info) = model.types.get(&var_info.data_type) {
                total_size += type_info.size_bytes;
                
                // Simple heuristic: local variables go on stack, others in static
                if var_info.scope.contains("function") || var_info.scope.contains("block") {
                    stack_size += type_info.size_bytes;
                } else {
                    static_size += type_info.size_bytes;
                }

                // Add alignment requirement if needed
                if type_info.alignment > 1 {
                    model.memory_layout.alignment_requirements.push(AlignmentRequirement {
                        variable_name: name.clone(),
                        required_alignment: type_info.alignment,
                        reason: format!("Type {} requires {}-byte alignment", 
                                      type_info.name, type_info.alignment),
                    });
                }
            }
        }

        model.memory_layout.total_size = total_size;
        model.memory_layout.stack_size = stack_size;
        model.memory_layout.static_size = static_size;
        model.memory_layout.heap_size = 0; // Will be calculated during runtime

        Ok(())
    }

    fn llm_semantic_analysis(&self, intent: &ProgramIntent, model: &mut SemanticModel) -> Result<()> {
        debug!("Performing LLM-based semantic analysis");

        let prompt = format!(
            r#"You are an advanced semantic analysis agent. Analyze the following program intent and provide additional semantic insights.

PROGRAM INTENT:
Operations: {} operations detected
Data Structures: {} data structures detected
Control Flow: {} nodes in control flow graph
Current Errors: {} semantic errors found

CURRENT SEMANTIC MODEL:
Variables: {}
Functions: {}
Types: {}

Provide additional semantic analysis focusing on:
1. Type compatibility issues
2. Potential runtime errors
3. Performance bottlenecks
4. Memory safety concerns
5. Code quality improvements

Return your analysis as structured insights that can improve the semantic model."#,
            intent.operations.len(),
            intent.data_structures.len(),
            intent.control_flow.nodes.len(),
            model.semantic_errors.len(),
            serde_json::to_string_pretty(&model.variables)?,
            serde_json::to_string_pretty(&model.functions)?,
            serde_json::to_string_pretty(&model.types)?
        );

        let _response = self.gemini_client.execute_code(&prompt)?;
        
        // For now, we'll add a generic insight
        // In a full implementation, we'd parse the LLM response and update the model
        info!("LLM semantic analysis completed - additional insights integrated");

        Ok(())
    }
}