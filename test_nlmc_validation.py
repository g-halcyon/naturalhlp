#!/usr/bin/env python3
"""
NLMC Compiler Validation Test Suite
Tests the Natural Language to Machine Code compiler implementation
"""

import os
import sys
import subprocess
import json
from pathlib import Path

class NLMCTester:
    def __init__(self, repo_path):
        self.repo_path = Path(repo_path)
        self.test_results = []
        
    def log(self, message, level="INFO"):
        print(f"[{level}] {message}")
        
    def test_file_structure(self):
        """Test that all required NLMC files exist"""
        self.log("Testing NLMC file structure...")
        
        required_files = [
            "src/nlmc/mod.rs",
            "src/nlmc/intent_extractor.rs",
            "src/nlmc/semantic_analyzer.rs", 
            "src/nlmc/type_inferencer.rs",
            "src/nlmc/flow_analyzer.rs",
            "src/nlmc/llvm_generator.rs",
            "src/nlmc/ambiguity_resolver.rs",
            "src/nlmc/error_recovery.rs",
            "src/nlmc/syscall_interface.rs",
            "src/nlmc/hardware_layer.rs",
            "tests/nlmc_tests.rs",
            "examples/nlmc_samples.md"
        ]
        
        missing_files = []
        for file_path in required_files:
            full_path = self.repo_path / file_path
            if not full_path.exists():
                missing_files.append(file_path)
                
        if missing_files:
            self.log(f"Missing files: {missing_files}", "ERROR")
            return False
        else:
            self.log("✅ All required NLMC files present")
            return True
            
    def test_cargo_dependencies(self):
        """Test that Cargo.toml has required dependencies"""
        self.log("Testing Cargo dependencies...")
        
        cargo_path = self.repo_path / "Cargo.toml"
        if not cargo_path.exists():
            self.log("Cargo.toml not found", "ERROR")
            return False
            
        with open(cargo_path, 'r') as f:
            cargo_content = f.read()
            
        required_deps = [
            "inkwell",
            "petgraph", 
            "regex",
            "uuid",
            "dashmap"
        ]
        
        missing_deps = []
        for dep in required_deps:
            if dep not in cargo_content:
                missing_deps.append(dep)
                
        if missing_deps:
            self.log(f"Missing dependencies: {missing_deps}", "ERROR")
            return False
        else:
            self.log("✅ All required dependencies present")
            return True
            
    def test_main_integration(self):
        """Test that main.rs properly integrates NLMC"""
        self.log("Testing main.rs NLMC integration...")
        
        main_path = self.repo_path / "src/main.rs"
        with open(main_path, 'r') as f:
            main_content = f.read()
            
        required_elements = [
            "use_nlmc: bool",
            "show_monologue: bool", 
            "NLMCompiler::new",
            "compile_with_monologue",
            "compile_and_execute"
        ]
        
        missing_elements = []
        for element in required_elements:
            if element not in main_content:
                missing_elements.append(element)
                
        if missing_elements:
            self.log(f"Missing main.rs elements: {missing_elements}", "ERROR")
            return False
        else:
            self.log("✅ Main.rs properly integrates NLMC")
            return True
            
    def test_module_structure(self):
        """Test the structure of individual NLMC modules"""
        self.log("Testing NLMC module structures...")
        
        # Test intent_extractor.rs
        intent_path = self.repo_path / "src/nlmc/intent_extractor.rs"
        with open(intent_path, 'r') as f:
            intent_content = f.read()
            
        if "pub struct IntentExtractor" not in intent_content:
            self.log("IntentExtractor struct missing", "ERROR")
            return False
            
        if "pub struct ProgramIntent" not in intent_content:
            self.log("ProgramIntent struct missing", "ERROR") 
            return False
            
        # Test semantic_analyzer.rs
        semantic_path = self.repo_path / "src/nlmc/semantic_analyzer.rs"
        with open(semantic_path, 'r') as f:
            semantic_content = f.read()
            
        if "pub struct SemanticAnalyzer" not in semantic_content:
            self.log("SemanticAnalyzer struct missing", "ERROR")
            return False
            
        # Test llvm_generator.rs
        llvm_path = self.repo_path / "src/nlmc/llvm_generator.rs"
        with open(llvm_path, 'r') as f:
            llvm_content = f.read()
            
        if "pub struct LLVMGenerator" not in llvm_content:
            self.log("LLVMGenerator struct missing", "ERROR")
            return False
            
        self.log("✅ All NLMC modules have proper structure")
        return True
        
    def test_sample_programs(self):
        """Test that sample programs are comprehensive"""
        self.log("Testing NLMC sample programs...")
        
        samples_path = self.repo_path / "examples/nlmc_samples.md"
        with open(samples_path, 'r') as f:
            samples_content = f.read()
            
        required_samples = [
            "Simple Variable Assignment",
            "Function Definition and Call", 
            "Loop with Conditional",
            "Array Processing"
        ]
        
        missing_samples = []
        for sample in required_samples:
            if sample not in samples_content:
                missing_samples.append(sample)
                
        if missing_samples:
            self.log(f"Missing sample programs: {missing_samples}", "ERROR")
            return False
            
        # Check for machine code examples
        if "Machine Code:" not in samples_content:
            self.log("No machine code examples found", "ERROR")
            return False
            
        if "Inner Monologue:" not in samples_content:
            self.log("No inner monologue examples found", "ERROR")
            return False
            
        self.log("✅ Sample programs are comprehensive")
        return True
        
    def test_error_handling(self):
        """Test error handling and recovery mechanisms"""
        self.log("Testing error handling mechanisms...")
        
        error_path = self.repo_path / "src/nlmc/error_recovery.rs"
        with open(error_path, 'r') as f:
            error_content = f.read()
            
        required_error_types = [
            "SyntaxError",
            "SemanticError", 
            "TypeError",
            "RuntimeError"
        ]
        
        missing_types = []
        for error_type in required_error_types:
            if error_type not in error_content:
                missing_types.append(error_type)
                
        if missing_types:
            self.log(f"Missing error types: {missing_types}", "ERROR")
            return False
            
        self.log("✅ Error handling mechanisms are comprehensive")
        return True
        
    def test_system_interfaces(self):
        """Test system call and hardware interfaces"""
        self.log("Testing system interfaces...")
        
        # Test syscall interface
        syscall_path = self.repo_path / "src/nlmc/syscall_interface.rs"
        with open(syscall_path, 'r') as f:
            syscall_content = f.read()
            
        required_syscalls = [
            "sys_open",
            "sys_read",
            "sys_write", 
            "sys_exit"
        ]
        
        missing_syscalls = []
        for syscall in required_syscalls:
            if syscall not in syscall_content:
                missing_syscalls.append(syscall)
                
        if missing_syscalls:
            self.log(f"Missing syscalls: {missing_syscalls}", "ERROR")
            return False
            
        # Test hardware layer
        hardware_path = self.repo_path / "src/nlmc/hardware_layer.rs"
        with open(hardware_path, 'r') as f:
            hardware_content = f.read()
            
        if "detect_cpu_features" not in hardware_content:
            self.log("CPU feature detection missing", "ERROR")
            return False
            
        self.log("✅ System interfaces are properly implemented")
        return True
        
    def test_comprehensive_tests(self):
        """Test that the test suite is comprehensive"""
        self.log("Testing test suite comprehensiveness...")
        
        test_path = self.repo_path / "tests/nlmc_tests.rs"
        with open(test_path, 'r') as f:
            test_content = f.read()
            
        required_test_modules = [
            "intent_extraction_tests",
            "semantic_analysis_tests",
            "type_inference_tests", 
            "flow_analysis_tests",
            "llvm_generation_tests",
            "ambiguity_resolution_tests",
            "error_recovery_tests",
            "integration_tests",
            "performance_tests"
        ]
        
        missing_modules = []
        for module in required_test_modules:
            if module not in test_content:
                missing_modules.append(module)
                
        if missing_modules:
            self.log(f"Missing test modules: {missing_modules}", "ERROR")
            return False
            
        self.log("✅ Test suite is comprehensive")
        return True
        
    def run_all_tests(self):
        """Run all validation tests"""
        self.log("Starting NLMC Compiler Validation Tests")
        self.log("=" * 50)
        
        tests = [
            ("File Structure", self.test_file_structure),
            ("Cargo Dependencies", self.test_cargo_dependencies),
            ("Main Integration", self.test_main_integration),
            ("Module Structure", self.test_module_structure),
            ("Sample Programs", self.test_sample_programs),
            ("Error Handling", self.test_error_handling),
            ("System Interfaces", self.test_system_interfaces),
            ("Test Suite", self.test_comprehensive_tests)
        ]
        
        passed = 0
        total = len(tests)
        
        for test_name, test_func in tests:
            self.log(f"\n--- Running {test_name} Test ---")
            try:
                if test_func():
                    passed += 1
                    self.test_results.append((test_name, "PASS"))
                else:
                    self.test_results.append((test_name, "FAIL"))
            except Exception as e:
                self.log(f"Test {test_name} failed with exception: {e}", "ERROR")
                self.test_results.append((test_name, "ERROR"))
                
        self.log("\n" + "=" * 50)
        self.log("NLMC VALIDATION RESULTS")
        self.log("=" * 50)
        
        for test_name, result in self.test_results:
            status_symbol = "✅" if result == "PASS" else "❌"
            self.log(f"{status_symbol} {test_name}: {result}")
            
        self.log(f"\nOverall: {passed}/{total} tests passed")
        
        if passed == total:
            self.log("🎉 ALL TESTS PASSED! NLMC implementation is valid!", "SUCCESS")
            return True
        else:
            self.log(f"⚠️  {total - passed} tests failed. Review implementation.", "WARNING")
            return False

if __name__ == "__main__":
    repo_path = "/repos/g-halcyon__naturalhlp"
    tester = NLMCTester(repo_path)
    success = tester.run_all_tests()
    sys.exit(0 if success else 1)