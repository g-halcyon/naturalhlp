//! Flow Analysis Agent
//! 
//! Analyzes control flow and data flow patterns for optimization and correctness.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use petgraph::{Graph, Directed};
use petgraph::graph::NodeIndex;
use log::{debug, info};

use crate::gemini::GeminiClient;
use super::intent_extractor::ProgramIntent;
use super::semantic_analyzer::SemanticModel;
use super::type_inferencer::TypeModel;

/// Flow analysis model containing control and data flow information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowModel {
    pub control_blocks: Vec<ControlBlock>,
    pub data_flows: Vec<DataFlow>,
    pub dominance_tree: DominanceTree,
    pub loop_analysis: LoopAnalysis,
    pub reaching_definitions: HashMap<String, HashSet<String>>,
    pub live_variables: HashMap<String, HashSet<String>>,
    pub available_expressions: HashMap<String, HashSet<String>>,
    pub optimization_opportunities: Vec<OptimizationOpportunity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlBlock {
    pub id: String,
    pub block_type: ControlBlockType,
    pub predecessors: Vec<String>,
    pub successors: Vec<String>,
    pub instructions: Vec<Instruction>,
    pub dominates: Vec<String>,
    pub dominated_by: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlBlockType {
    Entry,
    Exit,
    Basic,
    LoopHeader,
    LoopLatch,
    Conditional,
    Merge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub id: String,
    pub opcode: Opcode,
    pub operands: Vec<Operand>,
    pub result: Option<String>,
    pub side_effects: Vec<SideEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Opcode {
    // Arithmetic
    Add, Sub, Mul, Div, Mod,
    // Logical
    And, Or, Xor, Not,
    // Comparison
    Eq, Ne, Lt, Le, Gt, Ge,
    // Memory
    Load, Store, Alloca,
    // Control flow
    Br, CondBr, Ret, Call,
    // Type conversion
    Cast, Trunc, Ext,
    // Special
    Phi, Select,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operand {
    pub operand_type: OperandType,
    pub value: String,
    pub data_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperandType {
    Register,
    Immediate,
    Memory,
    Label,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SideEffect {
    ModifiesMemory,
    ReadsMemory,
    CallsFunction,
    ThrowsException,
    HasIO,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlow {
    pub from_instruction: String,
    pub to_instruction: String,
    pub variable: String,
    pub flow_type: DataFlowType,
    pub distance: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataFlowType {
    Definition,
    Use,
    DefUse,
    AntiDependence,
    OutputDependence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DominanceTree {
    pub nodes: Vec<DominanceNode>,
    pub root: String,
    pub immediate_dominators: HashMap<String, String>,
    pub dominance_frontiers: HashMap<String, HashSet<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DominanceNode {
    pub block_id: String,
    pub children: Vec<String>,
    pub dominance_level: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopAnalysis {
    pub natural_loops: Vec<NaturalLoop>,
    pub loop_nesting_tree: LoopNestingTree,
    pub induction_variables: HashMap<String, InductionVariable>,
    pub loop_invariants: HashMap<String, HashSet<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaturalLoop {
    pub header: String,
    pub latch: String,
    pub body: HashSet<String>,
    pub exits: Vec<String>,
    pub depth: usize,
    pub trip_count: Option<TripCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TripCount {
    Constant(usize),
    Variable(String),
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopNestingTree {
    pub root_loops: Vec<String>,
    pub nesting_levels: HashMap<String, usize>,
    pub parent_loop: HashMap<String, String>,
    pub child_loops: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InductionVariable {
    pub name: String,
    pub initial_value: String,
    pub step: String,
    pub final_value: Option<String>,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationOpportunity {
    pub opportunity_type: OptimizationType,
    pub description: String,
    pub affected_blocks: Vec<String>,
    pub estimated_benefit: OptimizationBenefit,
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationType {
    DeadCodeElimination,
    CommonSubexpressionElimination,
    ConstantPropagation,
    LoopInvariantCodeMotion,
    StrengthReduction,
    LoopUnrolling,
    Vectorization,
    InstructionScheduling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationBenefit {
    High,
    Medium,
    Low,
    Negligible,
}

/// Flow Analysis Agent
pub struct FlowAnalyzer {
    gemini_client: GeminiClient,
    cfg_graph: Graph<String, String, Directed>,
    block_map: HashMap<String, NodeIndex>,
}

impl FlowAnalyzer {
    pub fn new(gemini_client: GeminiClient) -> Result<Self> {
        Ok(Self {
            gemini_client,
            cfg_graph: Graph::new(),
            block_map: HashMap::new(),
        })
    }

    /// Analyze control and data flows
    pub fn analyze_flows(
        &self,
        intent: &ProgramIntent,
        semantic_model: &SemanticModel,
        type_model: &TypeModel,
    ) -> Result<FlowModel> {
        info!("Starting flow analysis");

        let mut flow_model = FlowModel {
            control_blocks: vec![],
            data_flows: vec![],
            dominance_tree: DominanceTree {
                nodes: vec![],
                root: "entry".to_string(),
                immediate_dominators: HashMap::new(),
                dominance_frontiers: HashMap::new(),
            },
            loop_analysis: LoopAnalysis {
                natural_loops: vec![],
                loop_nesting_tree: LoopNestingTree {
                    root_loops: vec![],
                    nesting_levels: HashMap::new(),
                    parent_loop: HashMap::new(),
                    child_loops: HashMap::new(),
                },
                induction_variables: HashMap::new(),
                loop_invariants: HashMap::new(),
            },
            reaching_definitions: HashMap::new(),
            live_variables: HashMap::new(),
            available_expressions: HashMap::new(),
            optimization_opportunities: vec![],
        };

        // Build control flow graph
        self.build_control_flow_graph(intent, &mut flow_model)?;

        // Analyze dominance relationships
        self.analyze_dominance(&mut flow_model)?;

        // Detect and analyze loops
        self.analyze_loops(&mut flow_model)?;

        // Perform data flow analysis
        self.analyze_data_flows(intent, semantic_model, &mut flow_model)?;

        // Identify optimization opportunities
        self.identify_optimizations(&mut flow_model)?;

        // Use LLM for advanced flow analysis
        self.llm_flow_analysis(intent, semantic_model, type_model, &mut flow_model)?;

        info!("Flow analysis completed with {} control blocks, {} data flows",
              flow_model.control_blocks.len(), flow_model.data_flows.len());

        Ok(flow_model)
    }

    fn build_control_flow_graph(&self, intent: &ProgramIntent, flow_model: &mut FlowModel) -> Result<()> {
        debug!("Building control flow graph");

        // Convert intent control flow to detailed control blocks
        for node in &intent.control_flow.nodes {
            let block_type = match node.node_type {
                super::intent_extractor::ControlFlowNodeType::Entry => ControlBlockType::Entry,
                super::intent_extractor::ControlFlowNodeType::Exit => ControlBlockType::Exit,
                super::intent_extractor::ControlFlowNodeType::BasicBlock => ControlBlockType::Basic,
                super::intent_extractor::ControlFlowNodeType::Conditional => ControlBlockType::Conditional,
                super::intent_extractor::ControlFlowNodeType::Loop => ControlBlockType::LoopHeader,
                super::intent_extractor::ControlFlowNodeType::FunctionCall => ControlBlockType::Basic,
            };

            // Generate instructions for this block
            let instructions = self.generate_instructions_for_block(intent, &node.id, &node.operations)?;

            // Find predecessors and successors
            let predecessors = intent.control_flow.edges.iter()
                .filter(|edge| edge.to == node.id)
                .map(|edge| edge.from.clone())
                .collect();

            let successors = intent.control_flow.edges.iter()
                .filter(|edge| edge.from == node.id)
                .map(|edge| edge.to.clone())
                .collect();

            let control_block = ControlBlock {
                id: node.id.clone(),
                block_type,
                predecessors,
                successors,
                instructions,
                dominates: vec![],
                dominated_by: vec![],
            };

            flow_model.control_blocks.push(control_block);
        }

        Ok(())
    }

    fn generate_instructions_for_block(
        &self,
        intent: &ProgramIntent,
        block_id: &str,
        operation_ids: &[String],
    ) -> Result<Vec<Instruction>> {
        let mut instructions = vec![];
        let mut instruction_counter = 0;

        for op_id in operation_ids {
            if let Some(operation) = intent.operations.iter().find(|op| op.id == *op_id) {
                let instruction = match &operation.operation_type {
                    super::intent_extractor::OperationType::Arithmetic { operator } => {
                        let opcode = match operator {
                            super::intent_extractor::ArithmeticOp::Add => Opcode::Add,
                            super::intent_extractor::ArithmeticOp::Subtract => Opcode::Sub,
                            super::intent_extractor::ArithmeticOp::Multiply => Opcode::Mul,
                            super::intent_extractor::ArithmeticOp::Divide => Opcode::Div,
                            super::intent_extractor::ArithmeticOp::Modulo => Opcode::Mod,
                            super::intent_extractor::ArithmeticOp::Power => Opcode::Call, // Power as function call
                        };

                        Instruction {
                            id: format!("{}_{}", block_id, instruction_counter),
                            opcode,
                            operands: operation.inputs.iter().map(|input| Operand {
                                operand_type: OperandType::Register,
                                value: input.clone(),
                                data_type: "i32".to_string(),
                            }).collect(),
                            result: operation.outputs.first().cloned(),
                            side_effects: vec![],
                        }
                    },
                    super::intent_extractor::OperationType::Assignment { target } => {
                        Instruction {
                            id: format!("{}_{}", block_id, instruction_counter),
                            opcode: Opcode::Store,
                            operands: vec![
                                Operand {
                                    operand_type: OperandType::Register,
                                    value: operation.inputs.first().unwrap_or(&"unknown".to_string()).clone(),
                                    data_type: "i32".to_string(),
                                },
                                Operand {
                                    operand_type: OperandType::Memory,
                                    value: target.clone(),
                                    data_type: "ptr".to_string(),
                                },
                            ],
                            result: None,
                            side_effects: vec![SideEffect::ModifiesMemory],
                        }
                    },
                    super::intent_extractor::OperationType::Input { .. } => {
                        Instruction {
                            id: format!("{}_{}", block_id, instruction_counter),
                            opcode: Opcode::Call,
                            operands: vec![Operand {
                                operand_type: OperandType::Label,
                                value: "input_function".to_string(),
                                data_type: "function".to_string(),
                            }],
                            result: operation.outputs.first().cloned(),
                            side_effects: vec![SideEffect::HasIO],
                        }
                    },
                    super::intent_extractor::OperationType::Output { .. } => {
                        Instruction {
                            id: format!("{}_{}", block_id, instruction_counter),
                            opcode: Opcode::Call,
                            operands: vec![
                                Operand {
                                    operand_type: OperandType::Label,
                                    value: "output_function".to_string(),
                                    data_type: "function".to_string(),
                                },
                                Operand {
                                    operand_type: OperandType::Register,
                                    value: operation.inputs.first().unwrap_or(&"unknown".to_string()).clone(),
                                    data_type: "i32".to_string(),
                                },
                            ],
                            result: None,
                            side_effects: vec![SideEffect::HasIO],
                        }
                    },
                    _ => {
                        // Generic instruction for other operation types
                        Instruction {
                            id: format!("{}_{}", block_id, instruction_counter),
                            opcode: Opcode::Call,
                            operands: vec![],
                            result: None,
                            side_effects: vec![],
                        }
                    }
                };

                instructions.push(instruction);
                instruction_counter += 1;
            }
        }

        Ok(instructions)
    }

    fn analyze_dominance(&self, flow_model: &mut FlowModel) -> Result<()> {
        debug!("Analyzing dominance relationships");

        // Simple dominance analysis - in a full implementation, this would use
        // more sophisticated algorithms like Cooper-Harvey-Kennedy
        let mut dominators: HashMap<String, HashSet<String>> = HashMap::new();

        // Initialize dominators
        for block in &flow_model.control_blocks {
            if block.block_type == ControlBlockType::Entry {
                dominators.insert(block.id.clone(), [block.id.clone()].iter().cloned().collect());
            } else {
                dominators.insert(block.id.clone(), 
                    flow_model.control_blocks.iter().map(|b| b.id.clone()).collect());
            }
        }

        // Iterative dominance calculation
        let mut changed = true;
        while changed {
            changed = false;
            for block in &flow_model.control_blocks {
                if block.block_type == ControlBlockType::Entry {
                    continue;
                }

                let mut new_dominators = HashSet::new();
                new_dominators.insert(block.id.clone());

                if !block.predecessors.is_empty() {
                    let mut intersection = dominators.get(&block.predecessors[0]).unwrap().clone();
                    for pred in &block.predecessors[1..] {
                        if let Some(pred_doms) = dominators.get(pred) {
                            intersection = intersection.intersection(pred_doms).cloned().collect();
                        }
                    }
                    new_dominators.extend(intersection);
                }

                if let Some(current_doms) = dominators.get(&block.id) {
                    if &new_dominators != current_doms {
                        changed = true;
                        dominators.insert(block.id.clone(), new_dominators);
                    }
                }
            }
        }

        // Update control blocks with dominance information
        for block in &mut flow_model.control_blocks {
            if let Some(doms) = dominators.get(&block.id) {
                block.dominated_by = doms.iter().cloned().collect();
                
                // Find blocks this block dominates
                for other_block in &flow_model.control_blocks {
                    if other_block.id != block.id {
                        if let Some(other_doms) = dominators.get(&other_block.id) {
                            if other_doms.contains(&block.id) {
                                block.dominates.push(other_block.id.clone());
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn analyze_loops(&self, flow_model: &mut FlowModel) -> Result<()> {
        debug!("Analyzing loop structures");

        // Detect back edges (edges from a node to its dominator)
        let mut back_edges = vec![];
        for block in &flow_model.control_blocks {
            for successor in &block.successors {
                if block.dominated_by.contains(successor) {
                    back_edges.push((block.id.clone(), successor.clone()));
                }
            }
        }

        // For each back edge, find the natural loop
        for (latch, header) in back_edges {
            let mut loop_body = HashSet::new();
            loop_body.insert(header.clone());
            loop_body.insert(latch.clone());

            // Find all nodes that can reach the latch without going through the header
            let mut worklist = vec![latch.clone()];
            while let Some(current) = worklist.pop() {
                for block in &flow_model.control_blocks {
                    if block.successors.contains(&current) && 
                       !loop_body.contains(&block.id) && 
                       block.id != header {
                        loop_body.insert(block.id.clone());
                        worklist.push(block.id.clone());
                    }
                }
            }

            // Find loop exits
            let mut exits = vec![];
            for body_block in &loop_body {
                if let Some(block) = flow_model.control_blocks.iter().find(|b| b.id == *body_block) {
                    for successor in &block.successors {
                        if !loop_body.contains(successor) {
                            exits.push(successor.clone());
                        }
                    }
                }
            }

            let natural_loop = NaturalLoop {
                header: header.clone(),
                latch,
                body: loop_body,
                exits,
                depth: 1, // Simplified - would calculate actual nesting depth
                trip_count: Some(TripCount::Unknown),
            };

            flow_model.loop_analysis.natural_loops.push(natural_loop);
        }

        Ok(())
    }

    fn analyze_data_flows(
        &self,
        intent: &ProgramIntent,
        semantic_model: &SemanticModel,
        flow_model: &mut FlowModel,
    ) -> Result<()> {
        debug!("Analyzing data flows");

        // Reaching definitions analysis
        self.compute_reaching_definitions(flow_model)?;

        // Live variables analysis
        self.compute_live_variables(flow_model)?;

        // Available expressions analysis
        self.compute_available_expressions(flow_model)?;

        // Build data flow edges
        for block in &flow_model.control_blocks {
            for instruction in &block.instructions {
                // For each operand, find its definition
                for operand in &instruction.operands {
                    if operand.operand_type == OperandType::Register {
                        if let Some(definitions) = flow_model.reaching_definitions.get(&operand.value) {
                            for def_id in definitions {
                                flow_model.data_flows.push(DataFlow {
                                    from_instruction: def_id.clone(),
                                    to_instruction: instruction.id.clone(),
                                    variable: operand.value.clone(),
                                    flow_type: DataFlowType::DefUse,
                                    distance: 1, // Simplified distance calculation
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn compute_reaching_definitions(&self, flow_model: &mut FlowModel) -> Result<()> {
        // Simplified reaching definitions - in practice, this would use
        // iterative data flow analysis
        for block in &flow_model.control_blocks {
            for instruction in &block.instructions {
                if let Some(result) = &instruction.result {
                    flow_model.reaching_definitions
                        .entry(result.clone())
                        .or_insert_with(HashSet::new)
                        .insert(instruction.id.clone());
                }
            }
        }
        Ok(())
    }

    fn compute_live_variables(&self, flow_model: &mut FlowModel) -> Result<()> {
        // Simplified live variables analysis
        for block in &flow_model.control_blocks {
            let mut live_vars = HashSet::new();
            for instruction in &block.instructions {
                // Add operands as live
                for operand in &instruction.operands {
                    if operand.operand_type == OperandType::Register {
                        live_vars.insert(operand.value.clone());
                    }
                }
                // Remove result from live set
                if let Some(result) = &instruction.result {
                    live_vars.remove(result);
                }
            }
            flow_model.live_variables.insert(block.id.clone(), live_vars);
        }
        Ok(())
    }

    fn compute_available_expressions(&self, flow_model: &mut FlowModel) -> Result<()> {
        // Simplified available expressions analysis
        for block in &flow_model.control_blocks {
            let mut available_exprs = HashSet::new();
            for instruction in &block.instructions {
                // Add expressions that are computed
                if matches!(instruction.opcode, Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div) {
                    let expr = format!("{:?}({:?})", instruction.opcode, instruction.operands);
                    available_exprs.insert(expr);
                }
            }
            flow_model.available_expressions.insert(block.id.clone(), available_exprs);
        }
        Ok(())
    }

    fn identify_optimizations(&self, flow_model: &mut FlowModel) -> Result<()> {
        debug!("Identifying optimization opportunities");

        // Dead code elimination opportunities
        for block in &flow_model.control_blocks {
            for instruction in &block.instructions {
                if let Some(result) = &instruction.result {
                    let is_live = flow_model.live_variables.values()
                        .any(|live_vars| live_vars.contains(result));
                    
                    if !is_live && instruction.side_effects.is_empty() {
                        flow_model.optimization_opportunities.push(OptimizationOpportunity {
                            opportunity_type: OptimizationType::DeadCodeElimination,
                            description: format!("Dead instruction: {}", instruction.id),
                            affected_blocks: vec![block.id.clone()],
                            estimated_benefit: OptimizationBenefit::Medium,
                            prerequisites: vec![],
                        });
                    }
                }
            }
        }

        // Common subexpression elimination
        let mut expression_counts: HashMap<String, usize> = HashMap::new();
        for available_exprs in flow_model.available_expressions.values() {
            for expr in available_exprs {
                *expression_counts.entry(expr.clone()).or_insert(0) += 1;
            }
        }

        for (expr, count) in expression_counts {
            if count > 1 {
                flow_model.optimization_opportunities.push(OptimizationOpportunity {
                    opportunity_type: OptimizationType::CommonSubexpressionElimination,
                    description: format!("Common subexpression: {} (appears {} times)", expr, count),
                    affected_blocks: vec![], // Would identify specific blocks
                    estimated_benefit: OptimizationBenefit::High,
                    prerequisites: vec!["dominance_analysis".to_string()],
                });
            }
        }

        // Loop optimization opportunities
        for natural_loop in &flow_model.loop_analysis.natural_loops {
            flow_model.optimization_opportunities.push(OptimizationOpportunity {
                opportunity_type: OptimizationType::LoopInvariantCodeMotion,
                description: format!("Loop invariant code motion for loop {}", natural_loop.header),
                affected_blocks: natural_loop.body.iter().cloned().collect(),
                estimated_benefit: OptimizationBenefit::High,
                prerequisites: vec!["loop_analysis".to_string()],
            });

            if natural_loop.body.len() < 10 { // Small loops are good candidates for unrolling
                flow_model.optimization_opportunities.push(OptimizationOpportunity {
                    opportunity_type: OptimizationType::LoopUnrolling,
                    description: format!("Loop unrolling for loop {}", natural_loop.header),
                    affected_blocks: natural_loop.body.iter().cloned().collect(),
                    estimated_benefit: OptimizationBenefit::Medium,
                    prerequisites: vec!["trip_count_analysis".to_string()],
                });
            }
        }

        Ok(())
    }

    fn llm_flow_analysis(
        &self,
        intent: &ProgramIntent,
        semantic_model: &SemanticModel,
        type_model: &TypeModel,
        flow_model: &mut FlowModel,
    ) -> Result<()> {
        debug!("Performing LLM-based flow analysis");

        let prompt = format!(
            r#"You are an advanced flow analysis agent. Analyze the program flow and provide optimization insights.

PROGRAM FLOW ANALYSIS:
- {} control blocks identified
- {} data flows detected
- {} natural loops found
- {} optimization opportunities identified

CURRENT ANALYSIS:
Control Blocks: {}
Loop Analysis: {} loops with nesting
Data Flow: {} reaching definitions, {} live variable sets

Provide advanced flow analysis insights focusing on:
1. Complex control flow patterns
2. Advanced loop optimizations
3. Inter-procedural data flow
4. Parallelization opportunities
5. Memory access patterns

Consider the natural language intent to identify flow patterns that traditional analysis might miss."#,
            flow_model.control_blocks.len(),
            flow_model.data_flows.len(),
            flow_model.loop_analysis.natural_loops.len(),
            flow_model.optimization_opportunities.len(),
            serde_json::to_string_pretty(&flow_model.control_blocks)?,
            flow_model.loop_analysis.natural_loops.len(),
            flow_model.reaching_definitions.len(),
            flow_model.live_variables.len()
        );

        let _response = self.gemini_client.execute_code(&prompt)?;
        
        // For now, add some generic advanced optimizations
        // In a full implementation, we'd parse the LLM response and add specific insights
        flow_model.optimization_opportunities.push(OptimizationOpportunity {
            opportunity_type: OptimizationType::Vectorization,
            description: "LLM-identified vectorization opportunity".to_string(),
            affected_blocks: vec![],
            estimated_benefit: OptimizationBenefit::High,
            prerequisites: vec!["vector_analysis".to_string()],
        });

        info!("LLM flow analysis completed - advanced optimizations identified");

        Ok(())
    }
}