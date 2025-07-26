//! System Call Interface
//! 
//! Provides abstraction layer for system calls and OS interactions.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::{debug, info};

/// System call interface for OS interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCallInterface {
    pub supported_calls: Vec<SystemCall>,
    pub platform_mappings: HashMap<String, PlatformMapping>,
    pub security_policies: Vec<SecurityPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCall {
    pub name: String,
    pub call_type: SystemCallType,
    pub parameters: Vec<SystemCallParameter>,
    pub return_type: String,
    pub description: String,
    pub platform_specific: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemCallType {
    FileSystem,
    Memory,
    Process,
    Network,
    Time,
    Input,
    Output,
    Threading,
    Signal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCallParameter {
    pub name: String,
    pub param_type: String,
    pub is_optional: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformMapping {
    pub platform: String,
    pub syscall_number: Option<u32>,
    pub calling_convention: CallingConvention,
    pub abi_details: ABIDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CallingConvention {
    SystemV,
    MicrosoftX64,
    ARM64,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABIDetails {
    pub register_usage: Vec<String>,
    pub stack_alignment: u32,
    pub return_convention: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub policy_type: SecurityPolicyType,
    pub allowed_calls: Vec<String>,
    pub restrictions: Vec<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityPolicyType {
    Sandbox,
    Restricted,
    Standard,
    Privileged,
}

impl SystemCallInterface {
    pub fn new() -> Self {
        let supported_calls = Self::initialize_system_calls();
        let platform_mappings = Self::initialize_platform_mappings();
        let security_policies = Self::initialize_security_policies();

        Self {
            supported_calls,
            platform_mappings,
            security_policies,
        }
    }

    fn initialize_system_calls() -> Vec<SystemCall> {
        vec![
            // I/O System Calls
            SystemCall {
                name: "read".to_string(),
                call_type: SystemCallType::Input,
                parameters: vec![
                    SystemCallParameter {
                        name: "fd".to_string(),
                        param_type: "i32".to_string(),
                        is_optional: false,
                        description: "File descriptor".to_string(),
                    },
                    SystemCallParameter {
                        name: "buf".to_string(),
                        param_type: "*mut u8".to_string(),
                        is_optional: false,
                        description: "Buffer to read into".to_string(),
                    },
                    SystemCallParameter {
                        name: "count".to_string(),
                        param_type: "usize".to_string(),
                        is_optional: false,
                        description: "Number of bytes to read".to_string(),
                    },
                ],
                return_type: "isize".to_string(),
                description: "Read from file descriptor".to_string(),
                platform_specific: false,
            },
            SystemCall {
                name: "write".to_string(),
                call_type: SystemCallType::Output,
                parameters: vec![
                    SystemCallParameter {
                        name: "fd".to_string(),
                        param_type: "i32".to_string(),
                        is_optional: false,
                        description: "File descriptor".to_string(),
                    },
                    SystemCallParameter {
                        name: "buf".to_string(),
                        param_type: "*const u8".to_string(),
                        is_optional: false,
                        description: "Buffer to write from".to_string(),
                    },
                    SystemCallParameter {
                        name: "count".to_string(),
                        param_type: "usize".to_string(),
                        is_optional: false,
                        description: "Number of bytes to write".to_string(),
                    },
                ],
                return_type: "isize".to_string(),
                description: "Write to file descriptor".to_string(),
                platform_specific: false,
            },
            // Memory Management
            SystemCall {
                name: "mmap".to_string(),
                call_type: SystemCallType::Memory,
                parameters: vec![
                    SystemCallParameter {
                        name: "addr".to_string(),
                        param_type: "*mut u8".to_string(),
                        is_optional: true,
                        description: "Preferred address".to_string(),
                    },
                    SystemCallParameter {
                        name: "length".to_string(),
                        param_type: "usize".to_string(),
                        is_optional: false,
                        description: "Size of mapping".to_string(),
                    },
                    SystemCallParameter {
                        name: "prot".to_string(),
                        param_type: "i32".to_string(),
                        is_optional: false,
                        description: "Protection flags".to_string(),
                    },
                    SystemCallParameter {
                        name: "flags".to_string(),
                        param_type: "i32".to_string(),
                        is_optional: false,
                        description: "Mapping flags".to_string(),
                    },
                ],
                return_type: "*mut u8".to_string(),
                description: "Map memory".to_string(),
                platform_specific: true,
            },
            // Process Control
            SystemCall {
                name: "exit".to_string(),
                call_type: SystemCallType::Process,
                parameters: vec![
                    SystemCallParameter {
                        name: "status".to_string(),
                        param_type: "i32".to_string(),
                        is_optional: false,
                        description: "Exit status".to_string(),
                    },
                ],
                return_type: "!".to_string(),
                description: "Exit process".to_string(),
                platform_specific: false,
            },
        ]
    }

    fn initialize_platform_mappings() -> HashMap<String, PlatformMapping> {
        let mut mappings = HashMap::new();

        // Linux x86_64
        mappings.insert("linux-x86_64".to_string(), PlatformMapping {
            platform: "linux-x86_64".to_string(),
            syscall_number: Some(0), // read syscall number
            calling_convention: CallingConvention::SystemV,
            abi_details: ABIDetails {
                register_usage: vec![
                    "rax".to_string(), // syscall number
                    "rdi".to_string(), // arg1
                    "rsi".to_string(), // arg2
                    "rdx".to_string(), // arg3
                    "r10".to_string(), // arg4
                    "r8".to_string(),  // arg5
                    "r9".to_string(),  // arg6
                ],
                stack_alignment: 16,
                return_convention: "rax".to_string(),
            },
        });

        // Windows x86_64
        mappings.insert("windows-x86_64".to_string(), PlatformMapping {
            platform: "windows-x86_64".to_string(),
            syscall_number: None, // Windows uses different mechanism
            calling_convention: CallingConvention::MicrosoftX64,
            abi_details: ABIDetails {
                register_usage: vec![
                    "rcx".to_string(), // arg1
                    "rdx".to_string(), // arg2
                    "r8".to_string(),  // arg3
                    "r9".to_string(),  // arg4
                ],
                stack_alignment: 16,
                return_convention: "rax".to_string(),
            },
        });

        mappings
    }

    fn initialize_security_policies() -> Vec<SecurityPolicy> {
        vec![
            SecurityPolicy {
                policy_type: SecurityPolicyType::Sandbox,
                allowed_calls: vec![
                    "read".to_string(),
                    "write".to_string(),
                    "exit".to_string(),
                ],
                restrictions: vec![
                    "No network access".to_string(),
                    "No file system write outside sandbox".to_string(),
                    "No process creation".to_string(),
                ],
                description: "Sandboxed execution environment".to_string(),
            },
            SecurityPolicy {
                policy_type: SecurityPolicyType::Standard,
                allowed_calls: vec![
                    "read".to_string(),
                    "write".to_string(),
                    "open".to_string(),
                    "close".to_string(),
                    "mmap".to_string(),
                    "munmap".to_string(),
                    "exit".to_string(),
                ],
                restrictions: vec![
                    "No privileged operations".to_string(),
                ],
                description: "Standard user-level permissions".to_string(),
            },
        ]
    }

    /// Generate system call wrapper code
    pub fn generate_syscall_wrapper(&self, syscall_name: &str, target_platform: &str) -> Result<String> {
        debug!("Generating syscall wrapper for {} on {}", syscall_name, target_platform);

        let syscall = self.supported_calls.iter()
            .find(|sc| sc.name == syscall_name)
            .ok_or_else(|| anyhow::anyhow!("Unsupported system call: {}", syscall_name))?;

        let platform_mapping = self.platform_mappings.get(target_platform)
            .ok_or_else(|| anyhow::anyhow!("Unsupported platform: {}", target_platform))?;

        match target_platform {
            "linux-x86_64" => self.generate_linux_x86_64_wrapper(syscall, platform_mapping),
            "windows-x86_64" => self.generate_windows_x86_64_wrapper(syscall, platform_mapping),
            _ => Err(anyhow::anyhow!("Platform wrapper generation not implemented: {}", target_platform)),
        }
    }

    fn generate_linux_x86_64_wrapper(&self, syscall: &SystemCall, _mapping: &PlatformMapping) -> Result<String> {
        let mut wrapper = String::new();

        // Generate inline assembly for Linux x86_64 syscall
        wrapper.push_str(&format!("// System call wrapper for {}\n", syscall.name));
        wrapper.push_str("unsafe {\n");
        wrapper.push_str("    let result: isize;\n");
        wrapper.push_str("    asm!(\n");
        wrapper.push_str("        \"syscall\",\n");

        // Map syscall number
        let syscall_number = match syscall.name.as_str() {
            "read" => 0,
            "write" => 1,
            "open" => 2,
            "close" => 3,
            "mmap" => 9,
            "munmap" => 11,
            "exit" => 60,
            _ => 0,
        };

        wrapper.push_str(&format!("        in(\"rax\") {},\n", syscall_number));

        // Map parameters to registers
        let registers = ["rdi", "rsi", "rdx", "r10", "r8", "r9"];
        for (i, param) in syscall.parameters.iter().enumerate() {
            if i < registers.len() {
                wrapper.push_str(&format!("        in(\"{}\") {},\n", registers[i], param.name));
            }
        }

        wrapper.push_str("        out(\"rax\") result,\n");
        wrapper.push_str("        clobber_abi(\"system\")\n");
        wrapper.push_str("    );\n");
        wrapper.push_str("    result\n");
        wrapper.push_str("}\n");

        Ok(wrapper)
    }

    fn generate_windows_x86_64_wrapper(&self, syscall: &SystemCall, _mapping: &PlatformMapping) -> Result<String> {
        let mut wrapper = String::new();

        // Generate Windows API call wrapper
        wrapper.push_str(&format!("// Windows API wrapper for {}\n", syscall.name));
        wrapper.push_str("unsafe {\n");

        match syscall.name.as_str() {
            "read" => {
                wrapper.push_str("    let mut bytes_read: u32 = 0;\n");
                wrapper.push_str("    let result = ReadFile(\n");
                wrapper.push_str("        fd as HANDLE,\n");
                wrapper.push_str("        buf,\n");
                wrapper.push_str("        count as u32,\n");
                wrapper.push_str("        &mut bytes_read,\n");
                wrapper.push_str("        std::ptr::null_mut()\n");
                wrapper.push_str("    );\n");
                wrapper.push_str("    if result != 0 { bytes_read as isize } else { -1 }\n");
            }
            "write" => {
                wrapper.push_str("    let mut bytes_written: u32 = 0;\n");
                wrapper.push_str("    let result = WriteFile(\n");
                wrapper.push_str("        fd as HANDLE,\n");
                wrapper.push_str("        buf,\n");
                wrapper.push_str("        count as u32,\n");
                wrapper.push_str("        &mut bytes_written,\n");
                wrapper.push_str("        std::ptr::null_mut()\n");
                wrapper.push_str("    );\n");
                wrapper.push_str("    if result != 0 { bytes_written as isize } else { -1 }\n");
            }
            "exit" => {
                wrapper.push_str("    ExitProcess(status as u32);\n");
            }
            _ => {
                wrapper.push_str("    // Unsupported syscall on Windows\n");
                wrapper.push_str("    -1\n");
            }
        }

        wrapper.push_str("}\n");

        Ok(wrapper)
    }

    /// Check if system call is allowed under security policy
    pub fn is_syscall_allowed(&self, syscall_name: &str, policy_type: &SecurityPolicyType) -> bool {
        for policy in &self.security_policies {
            if policy.policy_type == *policy_type {
                return policy.allowed_calls.contains(&syscall_name.to_string());
            }
        }
        false
    }

    /// Get system call by name
    pub fn get_syscall(&self, name: &str) -> Option<&SystemCall> {
        self.supported_calls.iter().find(|sc| sc.name == name)
    }

    /// List available system calls for platform
    pub fn list_available_syscalls(&self, platform: &str) -> Vec<&SystemCall> {
        if self.platform_mappings.contains_key(platform) {
            self.supported_calls.iter().collect()
        } else {
            // Return only platform-independent syscalls
            self.supported_calls.iter()
                .filter(|sc| !sc.platform_specific)
                .collect()
        }
    }
}

/// System call builder for natural language integration
pub struct SystemCallBuilder {
    interface: SystemCallInterface,
}

impl SystemCallBuilder {
    pub fn new() -> Self {
        Self {
            interface: SystemCallInterface::new(),
        }
    }

    /// Infer system call from natural language description
    pub fn infer_syscall_from_description(&self, description: &str) -> Result<Option<String>> {
        debug!("Inferring system call from: {}", description);

        let description_lower = description.to_lowercase();

        // Simple pattern matching for common operations
        if description_lower.contains("read") || description_lower.contains("input") {
            return Ok(Some("read".to_string()));
        }
        
        if description_lower.contains("write") || description_lower.contains("print") || 
           description_lower.contains("output") || description_lower.contains("display") {
            return Ok(Some("write".to_string()));
        }
        
        if description_lower.contains("exit") || description_lower.contains("quit") || 
           description_lower.contains("terminate") {
            return Ok(Some("exit".to_string()));
        }
        
        if description_lower.contains("allocate") || description_lower.contains("memory") {
            return Ok(Some("mmap".to_string()));
        }

        Ok(None)
    }

    /// Generate system call integration code
    pub fn generate_integration_code(&self, syscall_name: &str, target_platform: &str) -> Result<String> {
        info!("Generating system call integration for {} on {}", syscall_name, target_platform);

        let wrapper = self.interface.generate_syscall_wrapper(syscall_name, target_platform)?;
        
        let mut integration_code = String::new();
        integration_code.push_str("// Generated system call integration\n");
        integration_code.push_str("#[inline(never)]\n");
        integration_code.push_str(&format!("fn syscall_{}(", syscall_name));

        if let Some(syscall) = self.interface.get_syscall(syscall_name) {
            let params: Vec<String> = syscall.parameters.iter()
                .map(|p| format!("{}: {}", p.name, p.param_type))
                .collect();
            integration_code.push_str(&params.join(", "));
        }

        integration_code.push_str(&format!(") -> {} {{\n", 
            self.interface.get_syscall(syscall_name)
                .map(|sc| sc.return_type.as_str())
                .unwrap_or("i32")));
        
        integration_code.push_str(&wrapper);
        integration_code.push_str("}\n");

        Ok(integration_code)
    }
}