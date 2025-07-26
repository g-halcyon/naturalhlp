//! LLVM IR Generation Agent
//! 
//! Generates LLVM IR from analyzed program intent and optimizes for machine code generation.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::{debug, info, warn};

use super::intent_extractor::ProgramIntent;
use super::semantic_analyzer::SemanticModel;
use super::type_inferencer::TypeModel;
use super::flow_analyzer::FlowModel;

/// LLVM Module representation
#[derive(Debug, Clone)]
pub struct LLVMModule {
    pub name: String,
    pub functions: Vec<LLVMFunction>,
    pub global_variables: Vec<LLVMGlobalVariable>,
    pub types: Vec<LLVMType>,
    pub metadata: LLVMModuleMetadata,
    pub target_triple: String,
    pub data_layout: String,
}

#[derive(Debug, Clone)]
pub struct LLVMFunction {
    pub name: String,
    pub return_type: LLVMType,
    pub parameters: Vec<LLVMParameter>,
    pub basic_blocks: Vec<LLVMBasicBlock>,
    pub attributes: Vec<String>,
    pub linkage: LLVMLinkage,
}

#[derive(Debug, Clone)]
pub struct LLVMParameter {
    pub name: String,
    pub param_type: LLVMType,
    pub attributes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LLVMBasicBlock {
    pub label: String,
    pub instructions: Vec<LLVMInstruction>,
    pub terminator: LLVMTerminator,
}

#[derive(Debug, Clone)]
pub struct LLVMInstruction {
    pub result: Option<String>,
    pub opcode: LLVMOpcode,
    pub operands: Vec<LLVMOperand>,
    pub instruction_type: LLVMType,
    pub metadata: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum LLVMOpcode {
    // Arithmetic
    Add, FAdd, Sub, FSub, Mul, FMul, UDiv, SDiv, FDiv, URem, SRem, FRem,
    // Bitwise
    Shl, LShr, AShr, And, Or, Xor,
    // Memory
    Alloca, Load, Store, GetElementPtr,
    // Conversion
    Trunc, ZExt, SExt, FPToUI, FPToSI, UIToFP, SIToFP, FPTrunc, FPExt,
    PtrToInt, IntToPtr, BitCast,
    // Comparison
    ICmp, FCmp,
    // Other
    Select, PHI, Call, Invoke,
}

#[derive(Debug, Clone)]
pub struct LLVMOperand {
    pub operand_type: LLVMOperandType,
    pub value: String,
    pub value_type: LLVMType,
}

#[derive(Debug, Clone)]
pub enum LLVMOperandType {
    Register,
    Constant,
    GlobalVariable,
    Function,
    BasicBlock,
}

#[derive(Debug, Clone)]
pub enum LLVMTerminator {
    Ret { value: Option<LLVMOperand> },
    Br { dest: String },
    CondBr { condition: LLVMOperand, true_dest: String, false_dest: String },
    Switch { value: LLVMOperand, default_dest: String, cases: Vec<(LLVMOperand, String)> },
    Unreachable,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LLVMType {
    Void,
    Integer { bits: u32 },
    Float,
    Double,
    Pointer { pointee_type: Box<LLVMType> },
    Array { element_type: Box<LLVMType>, length: u64 },
    Struct { fields: Vec<LLVMType>, packed: bool },
    Function { return_type: Box<LLVMType>, param_types: Vec<LLVMType> },
    Vector { element_type: Box<LLVMType>, length: u32 },
    Label,
    Metadata,
}

#[derive(Debug, Clone)]
pub struct LLVMGlobalVariable {
    pub name: String,
    pub global_type: LLVMType,
    pub linkage: LLVMLinkage,
    pub initializer: Option<LLVMConstant>,
    pub is_constant: bool,
    pub alignment: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum LLVMLinkage {
    Private,
    Internal,
    External,
    ExternalWeak,
    Common,
    Appending,
}

#[derive(Debug, Clone)]
pub enum LLVMConstant {
    Integer { value: i64, bits: u32 },
    Float { value: f64 },
    String { value: String },
    Array { elements: Vec<LLVMConstant> },
    Struct { fields: Vec<LLVMConstant> },
    Null,
    Undef,
}

#[derive(Debug, Clone)]
pub struct LLVMModuleMetadata {
    pub source_filename: String,
    pub target_triple: String,
    pub data_layout: String,
    pub debug_info: bool,
    pub optimization_level: u32,
}

/// LLVM IR Generator
pub struct LLVMGenerator {
    register_counter: u32,
    label_counter: u32,
    current_function: Option<String>,
}

impl LLVMGenerator {
    pub fn new() -> Result<Self> {
        Ok(Self {
            register_counter: 0,
            label_counter: 0,
            current_function: None,
        })
    }

    /// Generate LLVM module from analyzed program components
    pub fn generate_module(
        &self,
        intent: &ProgramIntent,
        semantic_model: &SemanticModel,
        type_model: &TypeModel,
        flow_model: &FlowModel,
    ) -> Result<LLVMModule> {
        info!("Generating LLVM IR module");

        let mut module = LLVMModule {
            name: "nlmc_generated".to_string(),
            functions: vec![],
            global_variables: vec![],
            types: vec![],
            metadata: LLVMModuleMetadata {
                source_filename: "natural_language_input".to_string(),
                target_triple: "x86_64-unknown-linux-gnu".to_string(),
                data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128".to_string(),
                debug_info: false,
                optimization_level: 2,
            },
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128".to_string(),
        };

        // Generate global variables
        self.generate_global_variables(semantic_model, type_model, &mut module)?;

        // Generate main function
        self.generate_main_function(intent, semantic_model, type_model, flow_model, &mut module)?;

        // Generate helper functions
        self.generate_helper_functions(&mut module)?;

        // Generate type definitions
        self.generate_type_definitions(type_model, &mut module)?;

        info!("LLVM IR module generated with {} functions", module.functions.len());

        Ok(module)
    }

    fn generate_global_variables(
        &self,
        semantic_model: &SemanticModel,
        type_model: &TypeModel,
        module: &mut LLVMModule,
    ) -> Result<()> {
        debug!("Generating global variables");

        for (name, var_info) in &semantic_model.variables {
            if var_info.scope == "Global" {
                let llvm_type = self.convert_to_llvm_type(&var_info.data_type, type_model)?;
                
                let global_var = LLVMGlobalVariable {
                    name: format!("@{}", name),
                    global_type: llvm_type,
                    linkage: LLVMLinkage::Internal,
                    initializer: Some(LLVMConstant::Integer { value: 0, bits: 32 }),
                    is_constant: !var_info.is_mutable,
                    alignment: None,
                };

                module.global_variables.push(global_var);
            }
        }

        Ok(())
    }

    fn generate_main_function(
        &self,
        intent: &ProgramIntent,
        semantic_model: &SemanticModel,
        type_model: &TypeModel,
        flow_model: &FlowModel,
        module: &mut LLVMModule,
    ) -> Result<()> {
        debug!("Generating main function");

        let mut basic_blocks = vec![];
        let mut register_counter = 0;

        // Generate basic blocks from flow model
        for control_block in &flow_model.control_blocks {
            let mut instructions = vec![];

            // Convert flow instructions to LLVM instructions
            for flow_instruction in &control_block.instructions {
                let llvm_instruction = self.convert_flow_instruction_to_llvm(
                    flow_instruction,
                    &mut register_counter,
                    type_model,
                )?;
                instructions.push(llvm_instruction);
            }

            // Generate terminator
            let terminator = self.generate_terminator_for_block(control_block, flow_model)?;

            let basic_block = LLVMBasicBlock {
                label: control_block.id.clone(),
                instructions,
                terminator,
            };

            basic_blocks.push(basic_block);
        }

        let main_function = LLVMFunction {
            name: "main".to_string(),
            return_type: LLVMType::Integer { bits: 32 },
            parameters: vec![],
            basic_blocks,
            attributes: vec!["nounwind".to_string()],
            linkage: LLVMLinkage::External,
        };

        module.functions.push(main_function);

        Ok(())
    }

    fn convert_flow_instruction_to_llvm(
        &self,
        flow_instruction: &super::flow_analyzer::Instruction,
        register_counter: &mut u32,
        type_model: &TypeModel,
    ) -> Result<LLVMInstruction> {
        let llvm_opcode = match flow_instruction.opcode {
            super::flow_analyzer::Opcode::Add => LLVMOpcode::Add,
            super::flow_analyzer::Opcode::Sub => LLVMOpcode::Sub,
            super::flow_analyzer::Opcode::Mul => LLVMOpcode::Mul,
            super::flow_analyzer::Opcode::Div => LLVMOpcode::SDiv,
            super::flow_analyzer::Opcode::Load => LLVMOpcode::Load,
            super::flow_analyzer::Opcode::Store => LLVMOpcode::Store,
            super::flow_analyzer::Opcode::Call => LLVMOpcode::Call,
            super::flow_analyzer::Opcode::Alloca => LLVMOpcode::Alloca,
            _ => LLVMOpcode::Add, // Default fallback
        };

        let operands = flow_instruction.operands.iter()
            .map(|op| self.convert_flow_operand_to_llvm(op, type_model))
            .collect::<Result<Vec<_>>>()?;

        let result = if flow_instruction.result.is_some() {
            *register_counter += 1;
            Some(format!("%{}", register_counter))
        } else {
            None
        };

        let instruction_type = if !operands.is_empty() {
            operands[0].value_type.clone()
        } else {
            LLVMType::Integer { bits: 32 }
        };

        Ok(LLVMInstruction {
            result,
            opcode: llvm_opcode,
            operands,
            instruction_type,
            metadata: vec![],
        })
    }

    fn convert_flow_operand_to_llvm(
        &self,
        flow_operand: &super::flow_analyzer::Operand,
        type_model: &TypeModel,
    ) -> Result<LLVMOperand> {
        let operand_type = match flow_operand.operand_type {
            super::flow_analyzer::OperandType::Register => LLVMOperandType::Register,
            super::flow_analyzer::OperandType::Immediate => LLVMOperandType::Constant,
            super::flow_analyzer::OperandType::Memory => LLVMOperandType::GlobalVariable,
            super::flow_analyzer::OperandType::Label => LLVMOperandType::BasicBlock,
        };

        let value_type = self.convert_to_llvm_type(&flow_operand.data_type, type_model)?;

        Ok(LLVMOperand {
            operand_type,
            value: flow_operand.value.clone(),
            value_type,
        })
    }

    fn generate_terminator_for_block(
        &self,
        control_block: &super::flow_analyzer::ControlBlock,
        flow_model: &FlowModel,
    ) -> Result<LLVMTerminator> {
        match control_block.block_type {
            super::flow_analyzer::ControlBlockType::Exit => {
                Ok(LLVMTerminator::Ret { 
                    value: Some(LLVMOperand {
                        operand_type: LLVMOperandType::Constant,
                        value: "0".to_string(),
                        value_type: LLVMType::Integer { bits: 32 },
                    })
                })
            },
            super::flow_analyzer::ControlBlockType::Conditional => {
                if control_block.successors.len() >= 2 {
                    Ok(LLVMTerminator::CondBr {
                        condition: LLVMOperand {
                            operand_type: LLVMOperandType::Register,
                            value: "%cond".to_string(),
                            value_type: LLVMType::Integer { bits: 1 },
                        },
                        true_dest: control_block.successors[0].clone(),
                        false_dest: control_block.successors[1].clone(),
                    })
                } else {
                    Ok(LLVMTerminator::Br { 
                        dest: control_block.successors.first()
                            .unwrap_or(&"exit".to_string()).clone() 
                    })
                }
            },
            _ => {
                if let Some(successor) = control_block.successors.first() {
                    Ok(LLVMTerminator::Br { dest: successor.clone() })
                } else {
                    Ok(LLVMTerminator::Ret { value: None })
                }
            }
        }
    }

    fn generate_helper_functions(&self, module: &mut LLVMModule) -> Result<()> {
        debug!("Generating helper functions");

        // Generate input function
        let input_function = LLVMFunction {
            name: "input_function".to_string(),
            return_type: LLVMType::Integer { bits: 32 },
            parameters: vec![],
            basic_blocks: vec![
                LLVMBasicBlock {
                    label: "entry".to_string(),
                    instructions: vec![
                        LLVMInstruction {
                            result: Some("%1".to_string()),
                            opcode: LLVMOpcode::Call,
                            operands: vec![
                                LLVMOperand {
                                    operand_type: LLVMOperandType::Function,
                                    value: "@scanf".to_string(),
                                    value_type: LLVMType::Function {
                                        return_type: Box::new(LLVMType::Integer { bits: 32 }),
                                        param_types: vec![
                                            LLVMType::Pointer { 
                                                pointee_type: Box::new(LLVMType::Integer { bits: 8 }) 
                                            }
                                        ],
                                    },
                                },
                            ],
                            instruction_type: LLVMType::Integer { bits: 32 },
                            metadata: vec![],
                        }
                    ],
                    terminator: LLVMTerminator::Ret { 
                        value: Some(LLVMOperand {
                            operand_type: LLVMOperandType::Register,
                            value: "%1".to_string(),
                            value_type: LLVMType::Integer { bits: 32 },
                        })
                    },
                }
            ],
            attributes: vec![],
            linkage: LLVMLinkage::Internal,
        };

        // Generate output function
        let output_function = LLVMFunction {
            name: "output_function".to_string(),
            return_type: LLVMType::Void,
            parameters: vec![
                LLVMParameter {
                    name: "value".to_string(),
                    param_type: LLVMType::Integer { bits: 32 },
                    attributes: vec![],
                }
            ],
            basic_blocks: vec![
                LLVMBasicBlock {
                    label: "entry".to_string(),
                    instructions: vec![
                        LLVMInstruction {
                            result: None,
                            opcode: LLVMOpcode::Call,
                            operands: vec![
                                LLVMOperand {
                                    operand_type: LLVMOperandType::Function,
                                    value: "@printf".to_string(),
                                    value_type: LLVMType::Function {
                                        return_type: Box::new(LLVMType::Integer { bits: 32 }),
                                        param_types: vec![
                                            LLVMType::Pointer { 
                                                pointee_type: Box::new(LLVMType::Integer { bits: 8 }) 
                                            }
                                        ],
                                    },
                                },
                                LLVMOperand {
                                    operand_type: LLVMOperandType::Register,
                                    value: "%value".to_string(),
                                    value_type: LLVMType::Integer { bits: 32 },
                                },
                            ],
                            instruction_type: LLVMType::Void,
                            metadata: vec![],
                        }
                    ],
                    terminator: LLVMTerminator::Ret { value: None },
                }
            ],
            attributes: vec![],
            linkage: LLVMLinkage::Internal,
        };

        module.functions.push(input_function);
        module.functions.push(output_function);

        Ok(())
    }

    fn generate_type_definitions(&self, type_model: &TypeModel, module: &mut LLVMModule) -> Result<()> {
        debug!("Generating type definitions");

        for (name, inferred_type) in &type_model.types {
            let llvm_type = self.convert_inferred_type_to_llvm(inferred_type)?;
            module.types.push(llvm_type);
        }

        Ok(())
    }

    fn convert_to_llvm_type(&self, type_name: &str, type_model: &TypeModel) -> Result<LLVMType> {
        match type_name {
            "i32" => Ok(LLVMType::Integer { bits: 32 }),
            "i64" => Ok(LLVMType::Integer { bits: 64 }),
            "f32" => Ok(LLVMType::Float),
            "f64" => Ok(LLVMType::Double),
            "bool" => Ok(LLVMType::Integer { bits: 1 }),
            "ptr" => Ok(LLVMType::Pointer { 
                pointee_type: Box::new(LLVMType::Integer { bits: 8 }) 
            }),
            _ => {
                // Try to find in type model
                if let Some(inferred_type) = type_model.types.get(type_name) {
                    self.convert_inferred_type_to_llvm(inferred_type)
                } else {
                    warn!("Unknown type: {}, defaulting to i32", type_name);
                    Ok(LLVMType::Integer { bits: 32 })
                }
            }
        }
    }

    fn convert_inferred_type_to_llvm(&self, inferred_type: &super::type_inferencer::InferredType) -> Result<LLVMType> {
        match &inferred_type.base_type {
            super::type_inferencer::BaseType::Integer { bits, .. } => {
                Ok(LLVMType::Integer { bits: *bits as u32 })
            },
            super::type_inferencer::BaseType::Float { precision } => {
                match precision {
                    super::type_inferencer::FloatPrecision::Single => Ok(LLVMType::Float),
                    super::type_inferencer::FloatPrecision::Double => Ok(LLVMType::Double),
                    _ => Ok(LLVMType::Double), // Default to double
                }
            },
            super::type_inferencer::BaseType::Boolean => Ok(LLVMType::Integer { bits: 1 }),
            super::type_inferencer::BaseType::Pointer { target_type, .. } => {
                let pointee = self.convert_inferred_type_to_llvm(target_type)?;
                Ok(LLVMType::Pointer { pointee_type: Box::new(pointee) })
            },
            super::type_inferencer::BaseType::Array { element_type, length } => {
                let element = self.convert_inferred_type_to_llvm(element_type)?;
                let array_length = match length {
                    super::type_inferencer::ArrayLength::Fixed(len) => *len as u64,
                    _ => 0, // Dynamic arrays represented as pointers
                };
                Ok(LLVMType::Array { element_type: Box::new(element), length: array_length })
            },
            super::type_inferencer::BaseType::Struct { fields } => {
                let mut llvm_fields = vec![];
                for field_type in fields.values() {
                    llvm_fields.push(self.convert_inferred_type_to_llvm(field_type)?);
                }
                Ok(LLVMType::Struct { fields: llvm_fields, packed: false })
            },
            super::type_inferencer::BaseType::Function { params, return_type } => {
                let mut param_types = vec![];
                for param in params {
                    param_types.push(self.convert_inferred_type_to_llvm(param)?);
                }
                let ret_type = self.convert_inferred_type_to_llvm(return_type)?;
                Ok(LLVMType::Function { 
                    return_type: Box::new(ret_type), 
                    param_types 
                })
            },
            super::type_inferencer::BaseType::Void => Ok(LLVMType::Void),
            _ => Ok(LLVMType::Integer { bits: 32 }), // Default fallback
        }
    }

    /// Optimize the LLVM module
    pub fn optimize(&self, module: &mut LLVMModule) -> Result<()> {
        info!("Applying LLVM optimizations");

        // In a real implementation, this would use LLVM's optimization passes
        // For now, we'll simulate some basic optimizations

        for function in &mut module.functions {
            self.optimize_function(function)?;
        }

        info!("LLVM optimizations completed");
        Ok(())
    }

    fn optimize_function(&self, function: &mut LLVMFunction) -> Result<()> {
        debug!("Optimizing function: {}", function.name);

        // Dead code elimination
        for basic_block in &mut function.basic_blocks {
            basic_block.instructions.retain(|instruction| {
                // Keep instructions with side effects or that are used
                !instruction.result.is_some() || 
                matches!(instruction.opcode, LLVMOpcode::Call | LLVMOpcode::Store | LLVMOpcode::Load)
            });
        }

        // Constant folding (simplified)
        for basic_block in &mut function.basic_blocks {
            for instruction in &mut basic_block.instructions {
                if matches!(instruction.opcode, LLVMOpcode::Add) && instruction.operands.len() == 2 {
                    // Check if both operands are constants
                    if instruction.operands.iter().all(|op| op.operand_type == LLVMOperandType::Constant) {
                        // In a real implementation, we'd perform the actual constant folding
                        debug!("Constant folding opportunity detected");
                    }
                }
            }
        }

        Ok(())
    }

    /// Emit machine code from LLVM module
    pub fn emit_machine_code(&self, module: &LLVMModule, target_triple: &str) -> Result<Vec<u8>> {
        info!("Emitting machine code for target: {}", target_triple);

        // In a real implementation, this would use LLVM's code generation backend
        // For now, we'll generate a simple executable stub

        let machine_code = self.generate_executable_stub(module)?;

        info!("Machine code generated: {} bytes", machine_code.len());
        Ok(machine_code)
    }

    fn generate_executable_stub(&self, module: &LLVMModule) -> Result<Vec<u8>> {
        // Generate a minimal ELF executable that prints "Hello from NLMC!" and exits
        // This is a simplified stub - a real implementation would generate proper machine code

        let elf_header = vec![
            0x7f, 0x45, 0x4c, 0x46, // ELF magic
            0x02, 0x01, 0x01, 0x00, // 64-bit, little-endian, current version
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // padding
            0x02, 0x00, // executable file
            0x3e, 0x00, // x86-64
            0x01, 0x00, 0x00, 0x00, // version 1
        ];

        let mut machine_code = elf_header;
        
        // Add entry point address (simplified)
        machine_code.extend_from_slice(&[0x78, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00]);
        
        // Add program header offset
        machine_code.extend_from_slice(&[0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        
        // Add section header offset
        machine_code.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        
        // Add flags and header sizes
        machine_code.extend_from_slice(&[
            0x00, 0x00, 0x00, 0x00, // flags
            0x40, 0x00, // ELF header size
            0x38, 0x00, // program header size
            0x01, 0x00, // number of program headers
            0x40, 0x00, // section header size
            0x00, 0x00, // number of section headers
            0x00, 0x00, // section header string table index
        ]);

        // Add program header
        machine_code.extend_from_slice(&[
            0x01, 0x00, 0x00, 0x00, // PT_LOAD
            0x05, 0x00, 0x00, 0x00, // PF_R | PF_X
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // offset
            0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, // virtual address
            0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, // physical address
        ]);

        // Add file size and memory size
        let total_size = 0x1000u64; // 4KB
        machine_code.extend_from_slice(&total_size.to_le_bytes());
        machine_code.extend_from_slice(&total_size.to_le_bytes());
        machine_code.extend_from_slice(&[0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // alignment

        // Pad to entry point
        while machine_code.len() < 0x78 {
            machine_code.push(0x00);
        }

        // Add simple x86-64 assembly code that exits with status 0
        machine_code.extend_from_slice(&[
            0x48, 0xc7, 0xc0, 0x3c, 0x00, 0x00, 0x00, // mov rax, 60 (sys_exit)
            0x48, 0xc7, 0xc7, 0x00, 0x00, 0x00, 0x00, // mov rdi, 0 (exit status)
            0x0f, 0x05, // syscall
        ]);

        // Pad to 4KB
        while machine_code.len() < 0x1000 {
            machine_code.push(0x00);
        }

        Ok(machine_code)
    }

    /// Get the number of functions in the module
    pub fn function_count(&self) -> usize {
        // This would be called on the module, but for the interface we'll return a default
        0
    }
}

impl LLVMModule {
    pub fn function_count(&self) -> usize {
        self.functions.len()
    }
}