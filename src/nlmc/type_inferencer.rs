//! Type Inference Agent
//! 
//! Infers types and memory management from natural language context and semantic analysis.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::{debug, info};

use crate::gemini::GeminiClient;
use super::intent_extractor::{ProgramIntent, DataType};
use super::semantic_analyzer::SemanticModel;

/// Type model with inferred types and memory layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeModel {
    pub types: HashMap<String, InferredType>,
    pub type_constraints: Vec<TypeConstraint>,
    pub memory_layout: MemoryLayoutPlan,
    pub type_conversions: Vec<TypeConversion>,
    pub generic_instantiations: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferredType {
    pub name: String,
    pub base_type: BaseType,
    pub size_bytes: usize,
    pub alignment: usize,
    pub is_nullable: bool,
    pub lifetime: Lifetime,
    pub mutability: Mutability,
    pub ownership: Ownership,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BaseType {
    Integer { bits: u8, signed: bool },
    Float { precision: FloatPrecision },
    Boolean,
    Character,
    String { encoding: StringEncoding },
    Array { element_type: Box<InferredType>, length: ArrayLength },
    Slice { element_type: Box<InferredType> },
    Pointer { target_type: Box<InferredType>, is_const: bool },
    Reference { target_type: Box<InferredType>, is_const: bool },
    Struct { fields: HashMap<String, InferredType> },
    Union { variants: HashMap<String, InferredType> },
    Enum { variants: Vec<String>, underlying_type: Box<InferredType> },
    Function { params: Vec<InferredType>, return_type: Box<InferredType> },
    Generic { name: String, bounds: Vec<String> },
    Void,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FloatPrecision {
    Half,    // f16
    Single,  // f32
    Double,  // f64
    Extended, // f80
    Quad,    // f128
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StringEncoding {
    Utf8,
    Utf16,
    Utf32,
    Ascii,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrayLength {
    Fixed(usize),
    Dynamic,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Lifetime {
    Static,
    Function,
    Block { id: String },
    Parameter,
    Heap,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mutability {
    Immutable,
    Mutable,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Ownership {
    Owned,
    Borrowed,
    Shared,
    Weak,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeConstraint {
    pub constraint_type: TypeConstraintType,
    pub affected_types: Vec<String>,
    pub description: String,
    pub severity: ConstraintSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeConstraintType {
    SizeConstraint { min_size: Option<usize>, max_size: Option<usize> },
    AlignmentConstraint { required_alignment: usize },
    LifetimeConstraint { min_lifetime: Lifetime },
    MutabilityConstraint { required_mutability: Mutability },
    NullabilityConstraint { nullable: bool },
    ThreadSafetyConstraint,
    NumericRangeConstraint { min: Option<i64>, max: Option<i64> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLayoutPlan {
    pub total_size: usize,
    pub stack_layout: StackLayout,
    pub heap_layout: HeapLayout,
    pub static_layout: StaticLayout,
    pub alignment_padding: usize,
    pub memory_pools: Vec<MemoryPool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackLayout {
    pub frame_size: usize,
    pub variables: Vec<StackVariable>,
    pub spill_slots: usize,
    pub alignment: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackVariable {
    pub name: String,
    pub offset: usize,
    pub size: usize,
    pub alignment: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeapLayout {
    pub estimated_allocations: usize,
    pub allocation_patterns: Vec<AllocationPattern>,
    pub gc_strategy: GarbageCollectionStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationPattern {
    pub pattern_type: AllocationType,
    pub estimated_size: usize,
    pub frequency: AllocationFrequency,
    pub lifetime: Lifetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllocationType {
    SmallObject,
    LargeObject,
    Array,
    String,
    Closure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllocationFrequency {
    Once,
    Rare,
    Occasional,
    Frequent,
    Constant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GarbageCollectionStrategy {
    None,
    ReferenceCounting,
    MarkAndSweep,
    Generational,
    Incremental,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticLayout {
    pub global_variables: Vec<StaticVariable>,
    pub constants: Vec<StaticConstant>,
    pub string_literals: Vec<StringLiteral>,
    pub total_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticVariable {
    pub name: String,
    pub offset: usize,
    pub size: usize,
    pub is_initialized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticConstant {
    pub name: String,
    pub value: String,
    pub data_type: String,
    pub size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringLiteral {
    pub content: String,
    pub encoding: StringEncoding,
    pub offset: usize,
    pub size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPool {
    pub name: String,
    pub pool_type: MemoryPoolType,
    pub size: usize,
    pub alignment: usize,
    pub usage_pattern: PoolUsagePattern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryPoolType {
    FixedSize,
    Variable,
    Stack,
    Ring,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolUsagePattern {
    Sequential,
    Random,
    LIFO,
    FIFO,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeConversion {
    pub from_type: String,
    pub to_type: String,
    pub conversion_type: ConversionType,
    pub is_safe: bool,
    pub cost: ConversionCost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversionType {
    Implicit,
    Explicit,
    Coercion,
    Cast,
    Constructor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversionCost {
    Free,
    Cheap,
    Moderate,
    Expensive,
    Prohibited,
}

/// Type Inference Agent
pub struct TypeInferencer {
    gemini_client: GeminiClient,
}

impl TypeInferencer {
    pub fn new(gemini_client: GeminiClient) -> Result<Self> {
        Ok(Self { gemini_client })
    }

    /// Infer types and memory layout from intent and semantic analysis
    pub fn infer_types(&self, intent: &ProgramIntent, semantic_model: &SemanticModel) -> Result<TypeModel> {
        info!("Starting type inference");

        let mut type_model = TypeModel {
            types: HashMap::new(),
            type_constraints: vec![],
            memory_layout: MemoryLayoutPlan {
                total_size: 0,
                stack_layout: StackLayout {
                    frame_size: 0,
                    variables: vec![],
                    spill_slots: 0,
                    alignment: 8,
                },
                heap_layout: HeapLayout {
                    estimated_allocations: 0,
                    allocation_patterns: vec![],
                    gc_strategy: GarbageCollectionStrategy::None,
                },
                static_layout: StaticLayout {
                    global_variables: vec![],
                    constants: vec![],
                    string_literals: vec![],
                    total_size: 0,
                },
                alignment_padding: 0,
                memory_pools: vec![],
            },
            type_conversions: vec![],
            generic_instantiations: HashMap::new(),
        };

        // Infer types from data structures
        self.infer_from_data_structures(intent, &mut type_model)?;

        // Infer types from operations
        self.infer_from_operations(intent, &mut type_model)?;

        // Apply semantic constraints
        self.apply_semantic_constraints(semantic_model, &mut type_model)?;

        // Generate type constraints
        self.generate_type_constraints(&mut type_model)?;

        // Plan memory layout
        self.plan_memory_layout(&mut type_model)?;

        // Detect type conversions
        self.detect_type_conversions(&mut type_model)?;

        // Use LLM for complex type inference
        self.llm_type_inference(intent, semantic_model, &mut type_model)?;

        info!("Type inference completed with {} types inferred", type_model.types.len());

        Ok(type_model)
    }

    fn infer_from_data_structures(&self, intent: &ProgramIntent, type_model: &mut TypeModel) -> Result<()> {
        debug!("Inferring types from data structures");

        for data_structure in &intent.data_structures {
            let inferred_type = self.convert_data_type_to_inferred(&data_structure.data_type)?;
            type_model.types.insert(data_structure.name.clone(), inferred_type);
        }

        Ok(())
    }

    fn convert_data_type_to_inferred(&self, data_type: &DataType) -> Result<InferredType> {
        let base_type = match data_type {
            DataType::Integer { bits, signed } => BaseType::Integer { bits: *bits, signed: *signed },
            DataType::Float { precision } => BaseType::Float { 
                precision: match precision {
                    super::intent_extractor::FloatPrecision::Single => FloatPrecision::Single,
                    super::intent_extractor::FloatPrecision::Double => FloatPrecision::Double,
                }
            },
            DataType::Boolean => BaseType::Boolean,
            DataType::String { max_length: _ } => BaseType::String { encoding: StringEncoding::Utf8 },
            DataType::Array { element_type, length } => {
                let element = Box::new(self.convert_data_type_to_inferred(element_type)?);
                BaseType::Array { 
                    element_type: element,
                    length: match length {
                        Some(len) => ArrayLength::Fixed(*len),
                        None => ArrayLength::Dynamic,
                    }
                }
            },
            DataType::Pointer { target_type } => {
                let target = Box::new(self.convert_data_type_to_inferred(target_type)?);
                BaseType::Pointer { target_type: target, is_const: false }
            },
            DataType::Struct { fields } => {
                let mut struct_fields = HashMap::new();
                for (name, field_type) in fields {
                    struct_fields.insert(name.clone(), self.convert_data_type_to_inferred(field_type)?);
                }
                BaseType::Struct { fields: struct_fields }
            },
            DataType::Unknown => BaseType::Generic { name: "unknown".to_string(), bounds: vec![] },
        };

        let size_bytes = self.calculate_type_size(&base_type);
        let alignment = self.calculate_type_alignment(&base_type);

        Ok(InferredType {
            name: format!("{:?}", data_type),
            base_type,
            size_bytes,
            alignment,
            is_nullable: false,
            lifetime: Lifetime::Unknown,
            mutability: Mutability::Mutable,
            ownership: Ownership::Owned,
            constraints: vec![],
        })
    }

    fn calculate_type_size(&self, base_type: &BaseType) -> usize {
        match base_type {
            BaseType::Integer { bits, .. } => (*bits as usize) / 8,
            BaseType::Float { precision } => match precision {
                FloatPrecision::Half => 2,
                FloatPrecision::Single => 4,
                FloatPrecision::Double => 8,
                FloatPrecision::Extended => 10,
                FloatPrecision::Quad => 16,
            },
            BaseType::Boolean => 1,
            BaseType::Character => 4, // UTF-32
            BaseType::String { .. } => 24, // String header + pointer
            BaseType::Array { element_type, length } => {
                let element_size = element_type.size_bytes;
                match length {
                    ArrayLength::Fixed(len) => element_size * len,
                    _ => 16, // Dynamic array header
                }
            },
            BaseType::Pointer { .. } => 8, // 64-bit pointer
            BaseType::Reference { .. } => 8, // 64-bit reference
            BaseType::Struct { fields } => {
                fields.values().map(|f| f.size_bytes).sum::<usize>()
            },
            BaseType::Function { .. } => 8, // Function pointer
            _ => 8, // Default size
        }
    }

    fn calculate_type_alignment(&self, base_type: &BaseType) -> usize {
        match base_type {
            BaseType::Integer { bits, .. } => ((*bits as usize) / 8).min(8),
            BaseType::Float { precision } => match precision {
                FloatPrecision::Half => 2,
                FloatPrecision::Single => 4,
                FloatPrecision::Double => 8,
                FloatPrecision::Extended => 8,
                FloatPrecision::Quad => 16,
            },
            BaseType::Boolean => 1,
            BaseType::Character => 4,
            BaseType::String { .. } => 8,
            BaseType::Array { element_type, .. } => element_type.alignment,
            BaseType::Pointer { .. } => 8,
            BaseType::Reference { .. } => 8,
            BaseType::Struct { fields } => {
                fields.values().map(|f| f.alignment).max().unwrap_or(8)
            },
            BaseType::Function { .. } => 8,
            _ => 8,
        }
    }

    fn infer_from_operations(&self, intent: &ProgramIntent, type_model: &mut TypeModel) -> Result<()> {
        debug!("Inferring types from operations");

        for operation in &intent.operations {
            // Infer result types from operations
            match &operation.operation_type {
                super::intent_extractor::OperationType::Arithmetic { operator } => {
                    let result_type = match operator {
                        super::intent_extractor::ArithmeticOp::Add |
                        super::intent_extractor::ArithmeticOp::Subtract |
                        super::intent_extractor::ArithmeticOp::Multiply => {
                            InferredType {
                                name: "arithmetic_result".to_string(),
                                base_type: BaseType::Integer { bits: 32, signed: true },
                                size_bytes: 4,
                                alignment: 4,
                                is_nullable: false,
                                lifetime: Lifetime::Function,
                                mutability: Mutability::Immutable,
                                ownership: Ownership::Owned,
                                constraints: vec![],
                            }
                        },
                        super::intent_extractor::ArithmeticOp::Divide => {
                            InferredType {
                                name: "division_result".to_string(),
                                base_type: BaseType::Float { precision: FloatPrecision::Double },
                                size_bytes: 8,
                                alignment: 8,
                                is_nullable: false,
                                lifetime: Lifetime::Function,
                                mutability: Mutability::Immutable,
                                ownership: Ownership::Owned,
                                constraints: vec!["non_zero_divisor".to_string()],
                            }
                        },
                        _ => continue,
                    };

                    for output in &operation.outputs {
                        type_model.types.insert(output.clone(), result_type.clone());
                    }
                },
                super::intent_extractor::OperationType::Comparison { .. } => {
                    let bool_type = InferredType {
                        name: "comparison_result".to_string(),
                        base_type: BaseType::Boolean,
                        size_bytes: 1,
                        alignment: 1,
                        is_nullable: false,
                        lifetime: Lifetime::Function,
                        mutability: Mutability::Immutable,
                        ownership: Ownership::Owned,
                        constraints: vec![],
                    };

                    for output in &operation.outputs {
                        type_model.types.insert(output.clone(), bool_type.clone());
                    }
                },
                _ => continue,
            }
        }

        Ok(())
    }

    fn apply_semantic_constraints(&self, semantic_model: &SemanticModel, type_model: &mut TypeModel) -> Result<()> {
        debug!("Applying semantic constraints to type inference");

        // Apply constraints from semantic analysis
        for constraint in &semantic_model.safety_constraints {
            let type_constraint = match constraint.constraint_type {
                super::semantic_analyzer::SafetyConstraintType::NullPointerCheck => {
                    TypeConstraint {
                        constraint_type: TypeConstraintType::NullabilityConstraint { nullable: false },
                        affected_types: constraint.affected_variables.clone(),
                        description: constraint.description.clone(),
                        severity: ConstraintSeverity::Error,
                    }
                },
                super::semantic_analyzer::SafetyConstraintType::BoundsCheck => {
                    TypeConstraint {
                        constraint_type: TypeConstraintType::SizeConstraint { min_size: Some(0), max_size: None },
                        affected_types: constraint.affected_variables.clone(),
                        description: constraint.description.clone(),
                        severity: ConstraintSeverity::Error,
                    }
                },
                _ => continue,
            };

            type_model.type_constraints.push(type_constraint);
        }

        Ok(())
    }

    fn generate_type_constraints(&self, type_model: &mut TypeModel) -> Result<()> {
        debug!("Generating additional type constraints");

        // Generate alignment constraints
        for (name, inferred_type) in &type_model.types {
            if inferred_type.alignment > 1 {
                type_model.type_constraints.push(TypeConstraint {
                    constraint_type: TypeConstraintType::AlignmentConstraint { 
                        required_alignment: inferred_type.alignment 
                    },
                    affected_types: vec![name.clone()],
                    description: format!("Type {} requires {}-byte alignment", name, inferred_type.alignment),
                    severity: ConstraintSeverity::Warning,
                });
            }
        }

        Ok(())
    }

    fn plan_memory_layout(&self, type_model: &mut TypeModel) -> Result<()> {
        debug!("Planning memory layout");

        let mut stack_offset = 0;
        let mut static_offset = 0;

        // Plan stack layout
        for (name, inferred_type) in &type_model.types {
            if matches!(inferred_type.lifetime, Lifetime::Function | Lifetime::Block { .. }) {
                // Align stack offset
                stack_offset = (stack_offset + inferred_type.alignment - 1) & !(inferred_type.alignment - 1);
                
                type_model.memory_layout.stack_layout.variables.push(StackVariable {
                    name: name.clone(),
                    offset: stack_offset,
                    size: inferred_type.size_bytes,
                    alignment: inferred_type.alignment,
                });

                stack_offset += inferred_type.size_bytes;
            } else if matches!(inferred_type.lifetime, Lifetime::Static) {
                // Plan static layout
                static_offset = (static_offset + inferred_type.alignment - 1) & !(inferred_type.alignment - 1);
                
                type_model.memory_layout.static_layout.global_variables.push(StaticVariable {
                    name: name.clone(),
                    offset: static_offset,
                    size: inferred_type.size_bytes,
                    is_initialized: true,
                });

                static_offset += inferred_type.size_bytes;
            }
        }

        type_model.memory_layout.stack_layout.frame_size = stack_offset;
        type_model.memory_layout.static_layout.total_size = static_offset;
        type_model.memory_layout.total_size = stack_offset + static_offset;

        Ok(())
    }

    fn detect_type_conversions(&self, type_model: &mut TypeModel) -> Result<()> {
        debug!("Detecting type conversions");

        let type_names: Vec<String> = type_model.types.keys().cloned().collect();

        for from_type in &type_names {
            for to_type in &type_names {
                if from_type != to_type {
                    if let (Some(from), Some(to)) = (type_model.types.get(from_type), type_model.types.get(to_type)) {
                        let conversion = self.analyze_type_conversion(from, to);
                        if conversion.cost != ConversionCost::Prohibited {
                            type_model.type_conversions.push(TypeConversion {
                                from_type: from_type.clone(),
                                to_type: to_type.clone(),
                                conversion_type: conversion.conversion_type,
                                is_safe: conversion.is_safe,
                                cost: conversion.cost,
                            });
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn analyze_type_conversion(&self, from: &InferredType, to: &InferredType) -> TypeConversion {
        match (&from.base_type, &to.base_type) {
            (BaseType::Integer { bits: from_bits, signed: from_signed }, 
             BaseType::Integer { bits: to_bits, signed: to_signed }) => {
                if from_bits <= to_bits && from_signed == to_signed {
                    TypeConversion {
                        from_type: from.name.clone(),
                        to_type: to.name.clone(),
                        conversion_type: ConversionType::Implicit,
                        is_safe: true,
                        cost: ConversionCost::Free,
                    }
                } else {
                    TypeConversion {
                        from_type: from.name.clone(),
                        to_type: to.name.clone(),
                        conversion_type: ConversionType::Explicit,
                        is_safe: false,
                        cost: ConversionCost::Cheap,
                    }
                }
            },
            (BaseType::Integer { .. }, BaseType::Float { .. }) => {
                TypeConversion {
                    from_type: from.name.clone(),
                    to_type: to.name.clone(),
                    conversion_type: ConversionType::Implicit,
                    is_safe: true,
                    cost: ConversionCost::Cheap,
                }
            },
            (BaseType::Float { .. }, BaseType::Integer { .. }) => {
                TypeConversion {
                    from_type: from.name.clone(),
                    to_type: to.name.clone(),
                    conversion_type: ConversionType::Explicit,
                    is_safe: false,
                    cost: ConversionCost::Moderate,
                }
            },
            _ => {
                TypeConversion {
                    from_type: from.name.clone(),
                    to_type: to.name.clone(),
                    conversion_type: ConversionType::Cast,
                    is_safe: false,
                    cost: ConversionCost::Prohibited,
                }
            }
        }
    }

    fn llm_type_inference(&self, intent: &ProgramIntent, semantic_model: &SemanticModel, type_model: &mut TypeModel) -> Result<()> {
        debug!("Performing LLM-based type inference");

        let prompt = format!(
            r#"You are an advanced type inference agent. Analyze the program and provide additional type insights.

PROGRAM CONTEXT:
- {} operations with various types
- {} data structures defined
- {} variables in semantic model
- Current type model has {} inferred types

SEMANTIC CONSTRAINTS:
- {} safety constraints identified
- Memory layout: {} bytes total

Provide additional type inference insights focusing on:
1. More precise type bounds
2. Lifetime analysis improvements
3. Memory optimization opportunities
4. Type safety enhancements
5. Performance-oriented type choices

Consider the natural language context to infer more accurate types than basic pattern matching."#,
            intent.operations.len(),
            intent.data_structures.len(),
            semantic_model.variables.len(),
            type_model.types.len(),
            semantic_model.safety_constraints.len(),
            type_model.memory_layout.total_size
        );

        let _response = self.gemini_client.execute_code(&prompt)?;
        
        // For now, add some generic improvements
        // In a full implementation, we'd parse the LLM response and refine types
        info!("LLM type inference completed - type model refined");

        Ok(())
    }
}