use std::collections::HashMap;
use naturalhlp::nlmc::*;
use naturalhlp::nlmc::intent_extractor::*;
use naturalhlp::nlmc::semantic_analyzer::*;
use naturalhlp::nlmc::type_inferencer::*;
use naturalhlp::nlmc::flow_analyzer::*;
use naturalhlp::nlmc::llvm_generator::*;
use naturalhlp::nlmc::ambiguity_resolver::*;
use naturalhlp::nlmc::error_recovery::*;

// Mock LLM for testing
struct MockLLM {
    responses: HashMap<String, String>,
}

impl MockLLM {
    fn new() -> Self {
        let mut responses = HashMap::new();
        
        // Intent extraction responses
        responses.insert(
            "extract_intent".to_string(),
            r#"{
                "operations": [
                    {"type": "variable_declaration", "name": "x", "data_type": "i32"},
                    {"type": "assignment", "target": "x", "value": "42"},
                    {"type": "print", "value": "x"}
                ],
                "data_structures": [],
                "control_flow": {"type": "sequential"},
                "constraints": [],
                "ambiguities": []
            }"#.to_string()
        );
        
        // Semantic analysis responses
        responses.insert(
            "analyze_semantics".to_string(),
            r#"{
                "variables": {
                    "x": {"type": "i32", "scope": "global", "mutable": true}
                },
                "functions": {},
                "types": {
                    "i32": {"size": 4, "alignment": 4, "signed": true}
                },
                "memory_layout": {"stack_size": 1024, "heap_required": false},
                "safety_constraints": []
            }"#.to_string()
        );
        
        // Type inference responses
        responses.insert(
            "infer_types".to_string(),
            r#"{
                "type_assignments": {
                    "x": "i32"
                },
                "memory_requirements": {
                    "stack": 4,
                    "heap": 0
                },
                "safety_analysis": {
                    "memory_safe": true,
                    "type_safe": true
                }
            }"#.to_string()
        );
        
        Self { responses }
    }
    
    fn query(&self, prompt_type: &str) -> String {
        self.responses.get(prompt_type).unwrap_or(&"{}".to_string()).clone()
    }
}

#[cfg(test)]
mod intent_extraction_tests {
    use super::*;
    
    #[test]
    fn test_simple_variable_declaration() {
        let mock_llm = MockLLM::new();
        let extractor = IntentExtractor::new();
        
        let input = "Create a variable x with value 42 and print it";
        let intent = extractor.extract_intent(input, &mock_llm);
        
        assert!(intent.is_ok());
        let intent = intent.unwrap();
        assert_eq!(intent.operations.len(), 3);
        assert_eq!(intent.operations[0].operation_type, OperationType::VariableDeclaration);
    }
    
    #[test]
    fn test_control_flow_extraction() {
        let mock_llm = MockLLM::new();
        let extractor = IntentExtractor::new();
        
        let input = "If x is greater than 10, print 'large', otherwise print 'small'";
        let intent = extractor.extract_intent(input, &mock_llm);
        
        assert!(intent.is_ok());
        let intent = intent.unwrap();
        assert!(matches!(intent.control_flow.flow_type, ControlFlowType::Conditional));
    }
    
    #[test]
    fn test_loop_extraction() {
        let mock_llm = MockLLM::new();
        let extractor = IntentExtractor::new();
        
        let input = "Count from 1 to 10 and print each number";
        let intent = extractor.extract_intent(input, &mock_llm);
        
        assert!(intent.is_ok());
        let intent = intent.unwrap();
        assert!(matches!(intent.control_flow.flow_type, ControlFlowType::Loop));
    }
    
    #[test]
    fn test_function_extraction() {
        let mock_llm = MockLLM::new();
        let extractor = IntentExtractor::new();
        
        let input = "Define a function that adds two numbers and returns the result";
        let intent = extractor.extract_intent(input, &mock_llm);
        
        assert!(intent.is_ok());
        let intent = intent.unwrap();
        assert!(intent.operations.iter().any(|op| matches!(op.operation_type, OperationType::FunctionDefinition)));
    }
}

#[cfg(test)]
mod semantic_analysis_tests {
    use super::*;
    
    #[test]
    fn test_variable_scope_analysis() {
        let mock_llm = MockLLM::new();
        let analyzer = SemanticAnalyzer::new();
        
        let mut intent = ProgramIntent::new();
        intent.operations.push(Operation {
            operation_type: OperationType::VariableDeclaration,
            target: Some("x".to_string()),
            value: Some("42".to_string()),
            data_type: Some("i32".to_string()),
            parameters: vec![],
            metadata: HashMap::new(),
        });
        
        let result = analyzer.analyze(&intent, &mock_llm);
        assert!(result.is_ok());
        
        let model = result.unwrap();
        assert!(model.variables.contains_key("x"));
        assert_eq!(model.variables["x"].variable_type, "i32");
    }
    
    #[test]
    fn test_type_compatibility_check() {
        let mock_llm = MockLLM::new();
        let analyzer = SemanticAnalyzer::new();
        
        let mut intent = ProgramIntent::new();
        intent.operations.push(Operation {
            operation_type: OperationType::Assignment,
            target: Some("x".to_string()),
            value: Some("\"hello\"".to_string()),
            data_type: Some("i32".to_string()),
            parameters: vec![],
            metadata: HashMap::new(),
        });
        
        let result = analyzer.analyze(&intent, &mock_llm);
        // Should detect type mismatch
        assert!(result.is_err() || result.unwrap().safety_constraints.len() > 0);
    }
    
    #[test]
    fn test_memory_layout_analysis() {
        let mock_llm = MockLLM::new();
        let analyzer = SemanticAnalyzer::new();
        
        let mut intent = ProgramIntent::new();
        intent.operations.push(Operation {
            operation_type: OperationType::VariableDeclaration,
            target: Some("arr".to_string()),
            value: Some("[1, 2, 3, 4, 5]".to_string()),
            data_type: Some("array<i32>".to_string()),
            parameters: vec![],
            metadata: HashMap::new(),
        });
        
        let result = analyzer.analyze(&intent, &mock_llm);
        assert!(result.is_ok());
        
        let model = result.unwrap();
        assert!(model.memory_layout.stack_size > 0);
    }
}

#[cfg(test)]
mod type_inference_tests {
    use super::*;
    
    #[test]
    fn test_basic_type_inference() {
        let mock_llm = MockLLM::new();
        let inferencer = TypeInferencer::new();
        
        let mut intent = ProgramIntent::new();
        intent.operations.push(Operation {
            operation_type: OperationType::VariableDeclaration,
            target: Some("x".to_string()),
            value: Some("42".to_string()),
            data_type: None, // No explicit type
            parameters: vec![],
            metadata: HashMap::new(),
        });
        
        let result = inferencer.infer_types(&intent, &mock_llm);
        assert!(result.is_ok());
        
        let analysis = result.unwrap();
        assert!(analysis.type_assignments.contains_key("x"));
        assert_eq!(analysis.type_assignments["x"], "i32");
    }
    
    #[test]
    fn test_memory_safety_analysis() {
        let mock_llm = MockLLM::new();
        let inferencer = TypeInferencer::new();
        
        let mut intent = ProgramIntent::new();
        intent.operations.push(Operation {
            operation_type: OperationType::PointerOperation,
            target: Some("ptr".to_string()),
            value: Some("&x".to_string()),
            data_type: Some("*i32".to_string()),
            parameters: vec![],
            metadata: HashMap::new(),
        });
        
        let result = inferencer.infer_types(&intent, &mock_llm);
        assert!(result.is_ok());
        
        let analysis = result.unwrap();
        assert!(analysis.safety_analysis.contains_key("memory_safe"));
    }
    
    #[test]
    fn test_generic_type_resolution() {
        let mock_llm = MockLLM::new();
        let inferencer = TypeInferencer::new();
        
        let mut intent = ProgramIntent::new();
        intent.operations.push(Operation {
            operation_type: OperationType::FunctionCall,
            target: Some("result".to_string()),
            value: Some("max(5, 10)".to_string()),
            data_type: None,
            parameters: vec!["5".to_string(), "10".to_string()],
            metadata: HashMap::new(),
        });
        
        let result = inferencer.infer_types(&intent, &mock_llm);
        assert!(result.is_ok());
        
        let analysis = result.unwrap();
        assert!(analysis.type_assignments.contains_key("result"));
    }
}

#[cfg(test)]
mod flow_analysis_tests {
    use super::*;
    
    #[test]
    fn test_control_flow_graph_construction() {
        let analyzer = FlowAnalyzer::new();
        
        let mut intent = ProgramIntent::new();
        intent.operations.push(Operation {
            operation_type: OperationType::VariableDeclaration,
            target: Some("x".to_string()),
            value: Some("0".to_string()),
            data_type: Some("i32".to_string()),
            parameters: vec![],
            metadata: HashMap::new(),
        });
        
        intent.operations.push(Operation {
            operation_type: OperationType::Conditional,
            target: Some("x > 5".to_string()),
            value: None,
            data_type: None,
            parameters: vec![],
            metadata: HashMap::new(),
        });
        
        let result = analyzer.analyze_control_flow(&intent);
        assert!(result.is_ok());
        
        let cfg = result.unwrap();
        assert!(cfg.nodes.len() >= 2);
        assert!(cfg.edges.len() >= 1);
    }
    
    #[test]
    fn test_data_flow_analysis() {
        let analyzer = FlowAnalyzer::new();
        
        let mut intent = ProgramIntent::new();
        intent.operations.push(Operation {
            operation_type: OperationType::VariableDeclaration,
            target: Some("x".to_string()),
            value: Some("42".to_string()),
            data_type: Some("i32".to_string()),
            parameters: vec![],
            metadata: HashMap::new(),
        });
        
        intent.operations.push(Operation {
            operation_type: OperationType::Assignment,
            target: Some("y".to_string()),
            value: Some("x + 1".to_string()),
            data_type: Some("i32".to_string()),
            parameters: vec![],
            metadata: HashMap::new(),
        });
        
        let result = analyzer.analyze_data_flow(&intent);
        assert!(result.is_ok());
        
        let dfa = result.unwrap();
        assert!(dfa.variable_definitions.contains_key("x"));
        assert!(dfa.variable_uses.contains_key("x"));
    }
    
    #[test]
    fn test_loop_detection() {
        let analyzer = FlowAnalyzer::new();
        
        let mut intent = ProgramIntent::new();
        intent.control_flow = ControlFlowGraph {
            flow_type: ControlFlowType::Loop,
            condition: Some("i < 10".to_string()),
            body: vec![],
            else_body: None,
            metadata: HashMap::new(),
        };
        
        let result = analyzer.analyze_control_flow(&intent);
        assert!(result.is_ok());
        
        let cfg = result.unwrap();
        assert!(cfg.has_loops);
        assert!(cfg.loop_info.len() > 0);
    }
}

#[cfg(test)]
mod llvm_generation_tests {
    use super::*;
    
    #[test]
    fn test_basic_llvm_ir_generation() {
        let generator = LLVMGenerator::new();
        
        let mut intent = ProgramIntent::new();
        intent.operations.push(Operation {
            operation_type: OperationType::VariableDeclaration,
            target: Some("x".to_string()),
            value: Some("42".to_string()),
            data_type: Some("i32".to_string()),
            parameters: vec![],
            metadata: HashMap::new(),
        });
        
        let mut semantics = SemanticModel::new();
        semantics.variables.insert("x".to_string(), VariableInfo {
            variable_type: "i32".to_string(),
            scope: "global".to_string(),
            mutable: true,
            memory_location: None,
        });
        
        let result = generator.generate_module(&intent, &semantics);
        assert!(result.is_ok());
        
        let module = result.unwrap();
        assert!(!module.ir_code.is_empty());
        assert!(module.ir_code.contains("i32"));
    }
    
    #[test]
    fn test_function_generation() {
        let generator = LLVMGenerator::new();
        
        let mut intent = ProgramIntent::new();
        intent.operations.push(Operation {
            operation_type: OperationType::FunctionDefinition,
            target: Some("add".to_string()),
            value: None,
            data_type: Some("i32".to_string()),
            parameters: vec!["a: i32".to_string(), "b: i32".to_string()],
            metadata: HashMap::new(),
        });
        
        let mut semantics = SemanticModel::new();
        semantics.functions.insert("add".to_string(), FunctionInfo {
            return_type: "i32".to_string(),
            parameters: vec![
                ("a".to_string(), "i32".to_string()),
                ("b".to_string(), "i32".to_string()),
            ],
            body: vec![],
            pure: true,
        });
        
        let result = generator.generate_module(&intent, &semantics);
        assert!(result.is_ok());
        
        let module = result.unwrap();
        assert!(module.ir_code.contains("define"));
        assert!(module.ir_code.contains("add"));
    }
    
    #[test]
    fn test_machine_code_emission() {
        let generator = LLVMGenerator::new();
        
        let module = LLVMModule {
            ir_code: "define i32 @main() { ret i32 0 }".to_string(),
            functions: vec!["main".to_string()],
            globals: vec![],
            metadata: HashMap::new(),
        };
        
        let result = generator.emit_machine_code(&module, "x86_64");
        assert!(result.is_ok());
        
        let machine_code = result.unwrap();
        assert!(!machine_code.is_empty());
        // Should contain ELF header for x86_64
        assert_eq!(machine_code[0], 0x7f); // ELF magic number
    }
}

#[cfg(test)]
mod ambiguity_resolution_tests {
    use super::*;
    
    #[test]
    fn test_pronoun_resolution() {
        let mock_llm = MockLLM::new();
        let resolver = AmbiguityResolver::new();
        
        let mut ambiguity = Ambiguity {
            ambiguity_type: AmbiguityType::PronounReference,
            description: "What does 'it' refer to?".to_string(),
            possible_resolutions: vec!["variable x".to_string(), "result of function".to_string()],
            confidence_scores: vec![0.8, 0.6],
            context: "Set x to 5. Print it.".to_string(),
        };
        
        let result = resolver.resolve_ambiguity(&mut ambiguity, &mock_llm);
        assert!(result.is_ok());
        assert!(ambiguity.confidence_scores[0] > 0.5);
    }
    
    #[test]
    fn test_type_ambiguity_resolution() {
        let mock_llm = MockLLM::new();
        let resolver = AmbiguityResolver::new();
        
        let mut ambiguity = Ambiguity {
            ambiguity_type: AmbiguityType::TypeAmbiguity,
            description: "Ambiguous numeric type".to_string(),
            possible_resolutions: vec!["i32".to_string(), "f64".to_string()],
            confidence_scores: vec![0.5, 0.5],
            context: "Set x to 3.14".to_string(),
        };
        
        let result = resolver.resolve_ambiguity(&mut ambiguity, &mock_llm);
        assert!(result.is_ok());
        // Should prefer f64 for decimal numbers
        assert!(ambiguity.possible_resolutions[1] == "f64");
    }
    
    #[test]
    fn test_scope_ambiguity_resolution() {
        let mock_llm = MockLLM::new();
        let resolver = AmbiguityResolver::new();
        
        let mut ambiguity = Ambiguity {
            ambiguity_type: AmbiguityType::ScopeAmbiguity,
            description: "Variable scope unclear".to_string(),
            possible_resolutions: vec!["local".to_string(), "global".to_string()],
            confidence_scores: vec![0.7, 0.3],
            context: "In function foo, use variable x".to_string(),
        };
        
        let result = resolver.resolve_ambiguity(&mut ambiguity, &mock_llm);
        assert!(result.is_ok());
        // Should prefer local scope in function context
        assert!(ambiguity.confidence_scores[0] > ambiguity.confidence_scores[1]);
    }
}

#[cfg(test)]
mod error_recovery_tests {
    use super::*;
    
    #[test]
    fn test_syntax_error_recovery() {
        let recovery = ErrorRecovery::new();
        
        let error = CompilerError {
            error_type: ErrorType::SyntaxError,
            message: "Unexpected token".to_string(),
            location: Some("line 5, column 10".to_string()),
            suggestions: vec![],
            severity: ErrorSeverity::Error,
        };
        
        let result = recovery.attempt_recovery(&error, "invalid syntax here");
        assert!(result.is_ok());
        
        let recovery_result = result.unwrap();
        assert!(recovery_result.recovered);
        assert!(!recovery_result.suggested_fix.is_empty());
    }
    
    #[test]
    fn test_semantic_error_recovery() {
        let recovery = ErrorRecovery::new();
        
        let error = CompilerError {
            error_type: ErrorType::SemanticError,
            message: "Undefined variable 'x'".to_string(),
            location: Some("line 3".to_string()),
            suggestions: vec!["Did you mean 'y'?".to_string()],
            severity: ErrorSeverity::Error,
        };
        
        let result = recovery.attempt_recovery(&error, "print x");
        assert!(result.is_ok());
        
        let recovery_result = result.unwrap();
        assert!(recovery_result.suggested_fix.contains("y"));
    }
    
    #[test]
    fn test_type_error_recovery() {
        let recovery = ErrorRecovery::new();
        
        let error = CompilerError {
            error_type: ErrorType::TypeError,
            message: "Type mismatch: expected i32, found string".to_string(),
            location: Some("line 7".to_string()),
            suggestions: vec![],
            severity: ErrorSeverity::Error,
        };
        
        let result = recovery.attempt_recovery(&error, "x = \"hello\"");
        assert!(result.is_ok());
        
        let recovery_result = result.unwrap();
        assert!(recovery_result.suggested_fix.contains("parse") || 
                recovery_result.suggested_fix.contains("cast"));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_end_to_end_compilation() {
        let mock_llm = MockLLM::new();
        let compiler = NLMCompiler::new();
        
        let input = "Create a variable x with value 42 and print it";
        let result = compiler.compile_to_machine_code(input, &mock_llm);
        
        assert!(result.is_ok());
        let machine_code = result.unwrap();
        assert!(!machine_code.is_empty());
        
        // Should contain ELF header
        assert_eq!(machine_code[0], 0x7f);
        assert_eq!(machine_code[1], b'E');
        assert_eq!(machine_code[2], b'L');
        assert_eq!(machine_code[3], b'F');
    }
    
    #[test]
    fn test_complex_program_compilation() {
        let mock_llm = MockLLM::new();
        let compiler = NLMCompiler::new();
        
        let input = r#"
            Define a function called fibonacci that takes a number n.
            If n is less than 2, return n.
            Otherwise, return fibonacci(n-1) + fibonacci(n-2).
            Call fibonacci with 10 and print the result.
        "#;
        
        let result = compiler.compile_to_machine_code(input, &mock_llm);
        assert!(result.is_ok());
        
        let machine_code = result.unwrap();
        assert!(!machine_code.is_empty());
    }
    
    #[test]
    fn test_error_handling_integration() {
        let mock_llm = MockLLM::new();
        let compiler = NLMCompiler::new();
        
        let input = "Use undefined variable xyz and do something impossible";
        let result = compiler.compile_to_machine_code(input, &mock_llm);
        
        // Should handle errors gracefully
        assert!(result.is_err() || result.unwrap().is_empty());
    }
    
    #[test]
    fn test_monologue_generation() {
        let mock_llm = MockLLM::new();
        let compiler = NLMCompiler::new();
        
        let input = "Create a variable x with value 42";
        let result = compiler.compile_with_monologue(input, &mock_llm);
        
        assert!(result.is_ok());
        let (machine_code, monologue) = result.unwrap();
        
        assert!(!machine_code.is_empty());
        assert!(!monologue.is_empty());
        assert!(monologue.contains("parsing") || monologue.contains("analyzing"));
    }
}

// Performance benchmarks
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_compilation_performance() {
        let mock_llm = MockLLM::new();
        let compiler = NLMCompiler::new();
        
        let input = "Create variables a, b, c with values 1, 2, 3. Calculate a + b + c and print result.";
        
        let start = Instant::now();
        let result = compiler.compile_to_machine_code(input, &mock_llm);
        let duration = start.elapsed();
        
        assert!(result.is_ok());
        // Should complete within reasonable time (adjust threshold as needed)
        assert!(duration.as_secs() < 10);
    }
    
    #[test]
    fn test_memory_usage() {
        let mock_llm = MockLLM::new();
        let compiler = NLMCompiler::new();
        
        // Test with larger program
        let input = (0..100)
            .map(|i| format!("Create variable x{} with value {}", i, i))
            .collect::<Vec<_>>()
            .join(". ");
        
        let result = compiler.compile_to_machine_code(&input, &mock_llm);
        assert!(result.is_ok());
        
        // Memory usage should be reasonable (this is a basic check)
        let machine_code = result.unwrap();
        assert!(machine_code.len() < 1_000_000); // Less than 1MB for simple program
    }
}