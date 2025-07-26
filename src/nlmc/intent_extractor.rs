//! Intent Extraction Agent
//! 
//! Extracts computational intent and program structure from natural language descriptions.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::{debug, info};
use regex::Regex;

use crate::gemini::GeminiClient;

/// Represents the extracted intent from natural language
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramIntent {
    pub operations: Vec<Operation>,
    pub data_structures: Vec<DataStructure>,
    pub control_flow: ControlFlowGraph,
    pub constraints: Vec<Constraint>,
    pub ambiguities: Vec<Ambiguity>,
    pub metadata: IntentMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub id: String,
    pub operation_type: OperationType,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub description: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    Arithmetic { operator: ArithmeticOp },
    Comparison { operator: ComparisonOp },
    Assignment { target: String },
    FunctionCall { name: String, args: Vec<String> },
    Loop { condition: String, body: Vec<String> },
    Conditional { condition: String, then_branch: Vec<String>, else_branch: Option<Vec<String>> },
    Input { prompt: Option<String> },
    Output { format: Option<String> },
    MemoryAllocation { size: Option<usize>, type_hint: Option<String> },
    SystemCall { call_type: String, parameters: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArithmeticOp {
    Add, Subtract, Multiply, Divide, Modulo, Power,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOp {
    Equal, NotEqual, LessThan, LessEqual, GreaterThan, GreaterEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataStructure {
    pub name: String,
    pub data_type: DataType,
    pub size: Option<usize>,
    pub initial_value: Option<String>,
    pub scope: Scope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    Integer { bits: u8, signed: bool },
    Float { precision: FloatPrecision },
    String { max_length: Option<usize> },
    Boolean,
    Array { element_type: Box<DataType>, length: Option<usize> },
    Struct { fields: HashMap<String, DataType> },
    Pointer { target_type: Box<DataType> },
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FloatPrecision {
    Single, // f32
    Double, // f64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Scope {
    Global,
    Function { name: String },
    Block { id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlFlowGraph {
    pub nodes: Vec<ControlFlowNode>,
    pub edges: Vec<ControlFlowEdge>,
    pub entry_point: String,
    pub exit_points: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlFlowNode {
    pub id: String,
    pub node_type: ControlFlowNodeType,
    pub operations: Vec<String>, // References to Operation IDs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlFlowNodeType {
    Entry,
    Exit,
    BasicBlock,
    Conditional,
    Loop,
    FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlFlowEdge {
    pub from: String,
    pub to: String,
    pub condition: Option<String>,
    pub edge_type: EdgeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeType {
    Unconditional,
    ConditionalTrue,
    ConditionalFalse,
    LoopBack,
    FunctionCall,
    FunctionReturn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub constraint_type: ConstraintType,
    pub description: String,
    pub severity: ConstraintSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    TypeSafety,
    MemoryBounds,
    NullPointer,
    ResourceLeak,
    DeadCode,
    Performance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ambiguity {
    pub id: String,
    pub description: String,
    pub possible_interpretations: Vec<String>,
    pub context: String,
    pub confidence_scores: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentMetadata {
    pub complexity_score: f32,
    pub estimated_runtime: Option<String>,
    pub memory_requirements: Option<usize>,
    pub system_dependencies: Vec<String>,
    pub optimization_hints: Vec<String>,
}

/// Intent Extraction Agent using LLM reasoning
pub struct IntentExtractor {
    gemini_client: GeminiClient,
    pattern_matchers: Vec<PatternMatcher>,
}

struct PatternMatcher {
    name: String,
    regex: Regex,
    operation_type: OperationType,
}

impl IntentExtractor {
    pub fn new(gemini_client: GeminiClient) -> Result<Self> {
        let pattern_matchers = Self::initialize_pattern_matchers()?;
        
        Ok(Self {
            gemini_client,
            pattern_matchers,
        })
    }

    /// Extract program intent from natural language
    pub fn extract_intent(&self, input: &str) -> Result<ProgramIntent> {
        info!("Extracting intent from natural language input");
        debug!("Input: {}", input);

        // First pass: Use pattern matching for common constructs
        let mut operations = self.extract_operations_with_patterns(input)?;
        let mut data_structures = self.extract_data_structures_with_patterns(input)?;

        // Second pass: Use LLM for complex reasoning
        let llm_analysis = self.analyze_with_llm(input)?;
        
        // Merge pattern-based and LLM-based results
        operations.extend(llm_analysis.operations);
        data_structures.extend(llm_analysis.data_structures);

        // Build control flow graph
        let control_flow = self.build_control_flow_graph(&operations)?;

        // Extract constraints and ambiguities
        let constraints = self.extract_constraints(input, &operations)?;
        let ambiguities = self.detect_ambiguities(input)?;

        // Generate metadata
        let metadata = self.generate_metadata(input, &operations, &data_structures)?;

        Ok(ProgramIntent {
            operations,
            data_structures,
            control_flow,
            constraints,
            ambiguities,
            metadata,
        })
    }

    fn initialize_pattern_matchers() -> Result<Vec<PatternMatcher>> {
        let mut matchers = Vec::new();

        // Arithmetic operations
        matchers.push(PatternMatcher {
            name: "addition".to_string(),
            regex: Regex::new(r"(?i)(add|sum|plus|\+|calculate.*sum)")?,
            operation_type: OperationType::Arithmetic { operator: ArithmeticOp::Add },
        });

        matchers.push(PatternMatcher {
            name: "multiplication".to_string(),
            regex: Regex::new(r"(?i)(multiply|times|\*|product)")?,
            operation_type: OperationType::Arithmetic { operator: ArithmeticOp::Multiply },
        });

        // I/O operations
        matchers.push(PatternMatcher {
            name: "input".to_string(),
            regex: Regex::new(r"(?i)(ask|input|read|get.*from.*user)")?,
            operation_type: OperationType::Input { prompt: None },
        });

        matchers.push(PatternMatcher {
            name: "output".to_string(),
            regex: Regex::new(r"(?i)(print|display|show|output|write)")?,
            operation_type: OperationType::Output { format: None },
        });

        // Control flow
        matchers.push(PatternMatcher {
            name: "loop".to_string(),
            regex: Regex::new(r"(?i)(loop|repeat|while|for|iterate)")?,
            operation_type: OperationType::Loop { condition: "unknown".to_string(), body: vec![] },
        });

        Ok(matchers)
    }

    fn extract_operations_with_patterns(&self, input: &str) -> Result<Vec<Operation>> {
        let mut operations = Vec::new();
        let mut operation_id = 0;

        for matcher in &self.pattern_matchers {
            if matcher.regex.is_match(input) {
                operations.push(Operation {
                    id: format!("op_{}", operation_id),
                    operation_type: matcher.operation_type.clone(),
                    inputs: vec![],
                    outputs: vec![],
                    description: format!("Pattern matched: {}", matcher.name),
                    confidence: 0.8,
                });
                operation_id += 1;
            }
        }

        Ok(operations)
    }

    fn extract_data_structures_with_patterns(&self, input: &str) -> Result<Vec<DataStructure>> {
        let mut data_structures = Vec::new();

        // Pattern for numbers/variables
        let number_regex = Regex::new(r"(?i)(number|integer|value|variable)\s+(\w+)")?;
        for cap in number_regex.captures_iter(input) {
            if let Some(name) = cap.get(2) {
                data_structures.push(DataStructure {
                    name: name.as_str().to_string(),
                    data_type: DataType::Integer { bits: 32, signed: true },
                    size: None,
                    initial_value: None,
                    scope: Scope::Global,
                });
            }
        }

        // Pattern for arrays
        let array_regex = Regex::new(r"(?i)(array|list)\s+of\s+(\w+)")?;
        for cap in array_regex.captures_iter(input) {
            if let Some(name) = cap.get(2) {
                data_structures.push(DataStructure {
                    name: format!("{}_array", name.as_str()),
                    data_type: DataType::Array { 
                        element_type: Box::new(DataType::Integer { bits: 32, signed: true }),
                        length: None 
                    },
                    size: None,
                    initial_value: None,
                    scope: Scope::Global,
                });
            }
        }

        Ok(data_structures)
    }

    fn analyze_with_llm(&self, input: &str) -> Result<ProgramIntent> {
        let prompt = format!(
            r#"You are an advanced compiler intent extraction agent. Analyze the following natural language program description and extract the computational intent in JSON format.

NATURAL LANGUAGE PROGRAM:
{}

Extract and return a JSON object with the following structure:
{{
    "operations": [
        {{
            "id": "unique_id",
            "operation_type": "type_name",
            "inputs": ["input1", "input2"],
            "outputs": ["output1"],
            "description": "what this operation does",
            "confidence": 0.95
        }}
    ],
    "data_structures": [
        {{
            "name": "variable_name",
            "data_type": "inferred_type",
            "scope": "global|function|block"
        }}
    ]
}}

Focus on:
1. Computational operations (arithmetic, logic, I/O)
2. Data structures and variables
3. Control flow patterns
4. System interactions

Return ONLY the JSON object, no additional text."#,
            input
        );

        let response = self.gemini_client.execute_code(&prompt)?;
        
        // Parse the JSON response
        match serde_json::from_str::<ProgramIntent>(&response) {
            Ok(intent) => Ok(intent),
            Err(_) => {
                // Fallback: create a basic intent structure
                Ok(ProgramIntent {
                    operations: vec![],
                    data_structures: vec![],
                    control_flow: ControlFlowGraph {
                        nodes: vec![],
                        edges: vec![],
                        entry_point: "entry".to_string(),
                        exit_points: vec!["exit".to_string()],
                    },
                    constraints: vec![],
                    ambiguities: vec![],
                    metadata: IntentMetadata {
                        complexity_score: 0.5,
                        estimated_runtime: None,
                        memory_requirements: None,
                        system_dependencies: vec![],
                        optimization_hints: vec![],
                    },
                })
            }
        }
    }

    fn build_control_flow_graph(&self, operations: &[Operation]) -> Result<ControlFlowGraph> {
        let mut nodes = vec![
            ControlFlowNode {
                id: "entry".to_string(),
                node_type: ControlFlowNodeType::Entry,
                operations: vec![],
            }
        ];

        let mut edges = vec![];
        let mut current_node_id = "entry".to_string();

        // Create basic blocks for operations
        for (i, operation) in operations.iter().enumerate() {
            let node_id = format!("block_{}", i);
            
            nodes.push(ControlFlowNode {
                id: node_id.clone(),
                node_type: ControlFlowNodeType::BasicBlock,
                operations: vec![operation.id.clone()],
            });

            // Connect to previous node
            edges.push(ControlFlowEdge {
                from: current_node_id.clone(),
                to: node_id.clone(),
                condition: None,
                edge_type: EdgeType::Unconditional,
            });

            current_node_id = node_id;
        }

        // Add exit node
        let exit_node_id = "exit".to_string();
        nodes.push(ControlFlowNode {
            id: exit_node_id.clone(),
            node_type: ControlFlowNodeType::Exit,
            operations: vec![],
        });

        edges.push(ControlFlowEdge {
            from: current_node_id,
            to: exit_node_id.clone(),
            condition: None,
            edge_type: EdgeType::Unconditional,
        });

        Ok(ControlFlowGraph {
            nodes,
            edges,
            entry_point: "entry".to_string(),
            exit_points: vec![exit_node_id],
        })
    }

    fn extract_constraints(&self, input: &str, operations: &[Operation]) -> Result<Vec<Constraint>> {
        let mut constraints = Vec::new();

        // Check for potential null pointer issues
        if input.to_lowercase().contains("pointer") || input.to_lowercase().contains("reference") {
            constraints.push(Constraint {
                constraint_type: ConstraintType::NullPointer,
                description: "Potential null pointer dereference detected".to_string(),
                severity: ConstraintSeverity::Warning,
            });
        }

        // Check for array bounds
        if input.to_lowercase().contains("array") || input.to_lowercase().contains("index") {
            constraints.push(Constraint {
                constraint_type: ConstraintType::MemoryBounds,
                description: "Array bounds checking required".to_string(),
                severity: ConstraintSeverity::Error,
            });
        }

        // Performance constraints for loops
        for operation in operations {
            if matches!(operation.operation_type, OperationType::Loop { .. }) {
                constraints.push(Constraint {
                    constraint_type: ConstraintType::Performance,
                    description: "Loop optimization opportunity".to_string(),
                    severity: ConstraintSeverity::Info,
                });
            }
        }

        Ok(constraints)
    }

    fn detect_ambiguities(&self, input: &str) -> Result<Vec<Ambiguity>> {
        let mut ambiguities = Vec::new();

        // Detect ambiguous pronouns
        let pronoun_regex = Regex::new(r"(?i)\b(it|this|that|them|they)\b")?;
        if pronoun_regex.is_match(input) {
            ambiguities.push(Ambiguity {
                id: "pronoun_ambiguity".to_string(),
                description: "Ambiguous pronoun reference detected".to_string(),
                possible_interpretations: vec![
                    "Refers to previously mentioned variable".to_string(),
                    "Refers to result of previous operation".to_string(),
                ],
                context: input.to_string(),
                confidence_scores: vec![0.6, 0.4],
            });
        }

        // Detect ambiguous operations
        if input.to_lowercase().contains("calculate") && !input.to_lowercase().contains("add") 
            && !input.to_lowercase().contains("multiply") {
            ambiguities.push(Ambiguity {
                id: "operation_ambiguity".to_string(),
                description: "Ambiguous calculation operation".to_string(),
                possible_interpretations: vec![
                    "Addition operation".to_string(),
                    "Multiplication operation".to_string(),
                    "Complex mathematical formula".to_string(),
                ],
                context: input.to_string(),
                confidence_scores: vec![0.4, 0.3, 0.3],
            });
        }

        Ok(ambiguities)
    }

    fn generate_metadata(&self, input: &str, operations: &[Operation], data_structures: &[DataStructure]) -> Result<IntentMetadata> {
        let complexity_score = self.calculate_complexity_score(operations);
        let memory_requirements = self.estimate_memory_requirements(data_structures);
        let system_dependencies = self.detect_system_dependencies(input);
        let optimization_hints = self.generate_optimization_hints(operations);

        Ok(IntentMetadata {
            complexity_score,
            estimated_runtime: Some("O(n)".to_string()), // Simplified
            memory_requirements: Some(memory_requirements),
            system_dependencies,
            optimization_hints,
        })
    }

    fn calculate_complexity_score(&self, operations: &[Operation]) -> f32 {
        let base_score = operations.len() as f32 * 0.1;
        let loop_penalty = operations.iter()
            .filter(|op| matches!(op.operation_type, OperationType::Loop { .. }))
            .count() as f32 * 0.3;
        
        (base_score + loop_penalty).min(1.0)
    }

    fn estimate_memory_requirements(&self, data_structures: &[DataStructure]) -> usize {
        data_structures.iter()
            .map(|ds| match &ds.data_type {
                DataType::Integer { bits, .. } => (*bits as usize) / 8,
                DataType::Float { precision } => match precision {
                    FloatPrecision::Single => 4,
                    FloatPrecision::Double => 8,
                },
                DataType::String { max_length } => max_length.unwrap_or(256),
                DataType::Boolean => 1,
                DataType::Array { element_type: _, length } => length.unwrap_or(10) * 8, // Estimate
                _ => 8, // Default pointer size
            })
            .sum()
    }

    fn detect_system_dependencies(&self, input: &str) -> Vec<String> {
        let mut dependencies = Vec::new();

        if input.to_lowercase().contains("file") || input.to_lowercase().contains("read") {
            dependencies.push("filesystem".to_string());
        }
        if input.to_lowercase().contains("network") || input.to_lowercase().contains("http") {
            dependencies.push("network".to_string());
        }
        if input.to_lowercase().contains("time") || input.to_lowercase().contains("date") {
            dependencies.push("time".to_string());
        }

        dependencies
    }

    fn generate_optimization_hints(&self, operations: &[Operation]) -> Vec<String> {
        let mut hints = Vec::new();

        let loop_count = operations.iter()
            .filter(|op| matches!(op.operation_type, OperationType::Loop { .. }))
            .count();

        if loop_count > 0 {
            hints.push("Consider loop unrolling for performance".to_string());
        }

        let arithmetic_count = operations.iter()
            .filter(|op| matches!(op.operation_type, OperationType::Arithmetic { .. }))
            .count();

        if arithmetic_count > 3 {
            hints.push("Vectorization opportunity detected".to_string());
        }

        hints
    }
}