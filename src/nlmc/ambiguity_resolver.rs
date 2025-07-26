//! Ambiguity Resolution System
//! 
//! Resolves semantic ambiguities in natural language using LLM reasoning and context analysis.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::{debug, info, warn};

use crate::gemini::GeminiClient;
use super::intent_extractor::{ProgramIntent, Ambiguity};

/// Ambiguity resolution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionResult {
    pub resolved_ambiguities: Vec<ResolvedAmbiguity>,
    pub remaining_ambiguities: Vec<Ambiguity>,
    pub confidence_score: f32,
    pub resolution_strategy: ResolutionStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedAmbiguity {
    pub original_ambiguity: Ambiguity,
    pub chosen_interpretation: String,
    pub confidence: f32,
    pub reasoning: String,
    pub context_factors: Vec<ContextFactor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionStrategy {
    ContextualInference,
    TypeBasedDisambiguation,
    PrecedenceRules,
    UserInteraction,
    DefaultAssumption,
    LLMReasoning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextFactor {
    pub factor_type: ContextFactorType,
    pub description: String,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextFactorType {
    PreviousOperations,
    VariableTypes,
    DomainKnowledge,
    SyntacticPatterns,
    SemanticConstraints,
    UserIntent,
}

/// Ambiguity Resolution System
pub struct AmbiguityResolver {
    gemini_client: GeminiClient,
    resolution_rules: Vec<ResolutionRule>,
    context_history: Vec<ResolutionContext>,
}

#[derive(Debug, Clone)]
struct ResolutionRule {
    rule_type: RuleType,
    pattern: String,
    resolution: String,
    confidence: f32,
}

#[derive(Debug, Clone)]
enum RuleType {
    PronounReference,
    OperationDisambiguation,
    TypeInference,
    ScopeResolution,
    ControlFlowClarification,
}

#[derive(Debug, Clone)]
struct ResolutionContext {
    variables_in_scope: Vec<String>,
    recent_operations: Vec<String>,
    current_types: HashMap<String, String>,
    control_flow_state: String,
}

impl AmbiguityResolver {
    pub fn new(gemini_client: GeminiClient) -> Result<Self> {
        let resolution_rules = Self::initialize_resolution_rules()?;
        
        Ok(Self {
            gemini_client,
            resolution_rules,
            context_history: vec![],
        })
    }

    /// Resolve ambiguities in program intent
    pub fn resolve_ambiguities(&self, mut intent: ProgramIntent) -> Result<ProgramIntent> {
        info!("Resolving {} ambiguities", intent.ambiguities.len());

        if intent.ambiguities.is_empty() {
            return Ok(intent);
        }

        let mut resolved_ambiguities = vec![];
        let mut remaining_ambiguities = vec![];

        // Build current context
        let context = self.build_resolution_context(&intent)?;

        for ambiguity in intent.ambiguities {
            match self.resolve_single_ambiguity(&ambiguity, &context, &intent)? {
                Some(resolved) => {
                    resolved_ambiguities.push(resolved.clone());
                    // Apply resolution to intent
                    self.apply_resolution_to_intent(&resolved, &mut intent)?;
                }
                None => {
                    warn!("Could not resolve ambiguity: {}", ambiguity.description);
                    remaining_ambiguities.push(ambiguity);
                }
            }
        }

        intent.ambiguities = remaining_ambiguities;

        info!("Resolved {}/{} ambiguities", 
              resolved_ambiguities.len(), 
              resolved_ambiguities.len() + intent.ambiguities.len());

        Ok(intent)
    }

    fn initialize_resolution_rules() -> Result<Vec<ResolutionRule>> {
        let mut rules = vec![];

        // Pronoun reference rules
        rules.push(ResolutionRule {
            rule_type: RuleType::PronounReference,
            pattern: r"(?i)\bit\b".to_string(),
            resolution: "most_recent_variable".to_string(),
            confidence: 0.7,
        });

        rules.push(ResolutionRule {
            rule_type: RuleType::PronounReference,
            pattern: r"(?i)\bthis\b".to_string(),
            resolution: "current_context_variable".to_string(),
            confidence: 0.8,
        });

        // Operation disambiguation rules
        rules.push(ResolutionRule {
            rule_type: RuleType::OperationDisambiguation,
            pattern: r"(?i)calculate.*sum".to_string(),
            resolution: "addition_operation".to_string(),
            confidence: 0.9,
        });

        rules.push(ResolutionRule {
            rule_type: RuleType::OperationDisambiguation,
            pattern: r"(?i)calculate.*product".to_string(),
            resolution: "multiplication_operation".to_string(),
            confidence: 0.9,
        });

        // Type inference rules
        rules.push(ResolutionRule {
            rule_type: RuleType::TypeInference,
            pattern: r"(?i)number|integer|count".to_string(),
            resolution: "integer_type".to_string(),
            confidence: 0.8,
        });

        rules.push(ResolutionRule {
            rule_type: RuleType::TypeInference,
            pattern: r"(?i)decimal|float|real".to_string(),
            resolution: "float_type".to_string(),
            confidence: 0.8,
        });

        Ok(rules)
    }

    fn build_resolution_context(&self, intent: &ProgramIntent) -> Result<ResolutionContext> {
        let variables_in_scope = intent.data_structures.iter()
            .map(|ds| ds.name.clone())
            .collect();

        let recent_operations = intent.operations.iter()
            .take(5) // Last 5 operations for context
            .map(|op| op.description.clone())
            .collect();

        let current_types = intent.data_structures.iter()
            .map(|ds| (ds.name.clone(), format!("{:?}", ds.data_type)))
            .collect();

        Ok(ResolutionContext {
            variables_in_scope,
            recent_operations,
            current_types,
            control_flow_state: "sequential".to_string(), // Simplified
        })
    }

    fn resolve_single_ambiguity(
        &self,
        ambiguity: &Ambiguity,
        context: &ResolutionContext,
        intent: &ProgramIntent,
    ) -> Result<Option<ResolvedAmbiguity>> {
        debug!("Resolving ambiguity: {}", ambiguity.description);

        // Try rule-based resolution first
        if let Some(resolved) = self.try_rule_based_resolution(ambiguity, context)? {
            return Ok(Some(resolved));
        }

        // Try contextual inference
        if let Some(resolved) = self.try_contextual_resolution(ambiguity, context)? {
            return Ok(Some(resolved));
        }

        // Use LLM for complex reasoning
        if let Some(resolved) = self.try_llm_resolution(ambiguity, context, intent)? {
            return Ok(Some(resolved));
        }

        // Fall back to default assumption
        self.try_default_resolution(ambiguity)
    }

    fn try_rule_based_resolution(
        &self,
        ambiguity: &Ambiguity,
        context: &ResolutionContext,
    ) -> Result<Option<ResolvedAmbiguity>> {
        for rule in &self.resolution_rules {
            if self.rule_matches(rule, ambiguity, context)? {
                let resolved = ResolvedAmbiguity {
                    original_ambiguity: ambiguity.clone(),
                    chosen_interpretation: rule.resolution.clone(),
                    confidence: rule.confidence,
                    reasoning: format!("Applied rule: {:?}", rule.rule_type),
                    context_factors: vec![
                        ContextFactor {
                            factor_type: ContextFactorType::SyntacticPatterns,
                            description: format!("Matched pattern: {}", rule.pattern),
                            weight: rule.confidence,
                        }
                    ],
                };
                return Ok(Some(resolved));
            }
        }
        Ok(None)
    }

    fn rule_matches(
        &self,
        rule: &ResolutionRule,
        ambiguity: &Ambiguity,
        _context: &ResolutionContext,
    ) -> Result<bool> {
        use regex::Regex;
        let pattern = Regex::new(&rule.pattern)?;
        Ok(pattern.is_match(&ambiguity.context) || pattern.is_match(&ambiguity.description))
    }

    fn try_contextual_resolution(
        &self,
        ambiguity: &Ambiguity,
        context: &ResolutionContext,
    ) -> Result<Option<ResolvedAmbiguity>> {
        // Pronoun resolution using context
        if ambiguity.description.contains("pronoun") {
            if let Some(most_recent_var) = context.variables_in_scope.last() {
                let resolved = ResolvedAmbiguity {
                    original_ambiguity: ambiguity.clone(),
                    chosen_interpretation: most_recent_var.clone(),
                    confidence: 0.75,
                    reasoning: "Resolved pronoun to most recently mentioned variable".to_string(),
                    context_factors: vec![
                        ContextFactor {
                            factor_type: ContextFactorType::PreviousOperations,
                            description: format!("Most recent variable: {}", most_recent_var),
                            weight: 0.8,
                        }
                    ],
                };
                return Ok(Some(resolved));
            }
        }

        // Type-based disambiguation
        if ambiguity.description.contains("operation") {
            // Check if we have numeric types in context
            let has_integers = context.current_types.values()
                .any(|t| t.contains("Integer"));
            let has_floats = context.current_types.values()
                .any(|t| t.contains("Float"));

            if has_floats && !has_integers {
                let resolved = ResolvedAmbiguity {
                    original_ambiguity: ambiguity.clone(),
                    chosen_interpretation: "floating_point_operation".to_string(),
                    confidence: 0.8,
                    reasoning: "Context contains only floating-point types".to_string(),
                    context_factors: vec![
                        ContextFactor {
                            factor_type: ContextFactorType::VariableTypes,
                            description: "All variables are floating-point".to_string(),
                            weight: 0.9,
                        }
                    ],
                };
                return Ok(Some(resolved));
            }
        }

        Ok(None)
    }

    fn try_llm_resolution(
        &self,
        ambiguity: &Ambiguity,
        context: &ResolutionContext,
        intent: &ProgramIntent,
    ) -> Result<Option<ResolvedAmbiguity>> {
        let prompt = format!(
            r#"You are an advanced ambiguity resolution agent. Resolve the following ambiguity using context and reasoning.

AMBIGUITY TO RESOLVE:
Description: {}
Possible Interpretations: {:?}
Context: {}
Confidence Scores: {:?}

PROGRAM CONTEXT:
Variables in scope: {:?}
Recent operations: {:?}
Current types: {:?}
Control flow state: {}

PROGRAM INTENT:
Total operations: {}
Data structures: {}

Analyze the ambiguity and choose the most likely interpretation based on:
1. Contextual clues from surrounding code
2. Type information and constraints
3. Common programming patterns
4. Semantic consistency with the overall program

Respond with a JSON object:
{{
    "chosen_interpretation": "selected_interpretation",
    "confidence": 0.85,
    "reasoning": "detailed explanation of why this interpretation was chosen",
    "context_factors": [
        {{
            "factor_type": "VariableTypes",
            "description": "explanation of how this factor influenced the decision",
            "weight": 0.8
        }}
    ]
}}

Return ONLY the JSON object."#,
            ambiguity.description,
            ambiguity.possible_interpretations,
            ambiguity.context,
            ambiguity.confidence_scores,
            context.variables_in_scope,
            context.recent_operations,
            context.current_types,
            context.control_flow_state,
            intent.operations.len(),
            intent.data_structures.len()
        );

        let response = self.gemini_client.execute_code(&prompt)?;

        // Try to parse the JSON response
        match serde_json::from_str::<serde_json::Value>(&response) {
            Ok(json) => {
                if let (Some(interpretation), Some(confidence), Some(reasoning)) = (
                    json.get("chosen_interpretation").and_then(|v| v.as_str()),
                    json.get("confidence").and_then(|v| v.as_f64()),
                    json.get("reasoning").and_then(|v| v.as_str()),
                ) {
                    let mut context_factors = vec![];
                    
                    if let Some(factors) = json.get("context_factors").and_then(|v| v.as_array()) {
                        for factor in factors {
                            if let (Some(factor_type), Some(description), Some(weight)) = (
                                factor.get("factor_type").and_then(|v| v.as_str()),
                                factor.get("description").and_then(|v| v.as_str()),
                                factor.get("weight").and_then(|v| v.as_f64()),
                            ) {
                                let factor_type_enum = match factor_type {
                                    "VariableTypes" => ContextFactorType::VariableTypes,
                                    "PreviousOperations" => ContextFactorType::PreviousOperations,
                                    "DomainKnowledge" => ContextFactorType::DomainKnowledge,
                                    "SyntacticPatterns" => ContextFactorType::SyntacticPatterns,
                                    "SemanticConstraints" => ContextFactorType::SemanticConstraints,
                                    _ => ContextFactorType::UserIntent,
                                };

                                context_factors.push(ContextFactor {
                                    factor_type: factor_type_enum,
                                    description: description.to_string(),
                                    weight: weight as f32,
                                });
                            }
                        }
                    }

                    let resolved = ResolvedAmbiguity {
                        original_ambiguity: ambiguity.clone(),
                        chosen_interpretation: interpretation.to_string(),
                        confidence: confidence as f32,
                        reasoning: reasoning.to_string(),
                        context_factors,
                    };

                    return Ok(Some(resolved));
                }
            }
            Err(e) => {
                warn!("Failed to parse LLM resolution response: {}", e);
            }
        }

        Ok(None)
    }

    fn try_default_resolution(&self, ambiguity: &Ambiguity) -> Result<Option<ResolvedAmbiguity>> {
        // Use the first interpretation as default with low confidence
        if let Some(first_interpretation) = ambiguity.possible_interpretations.first() {
            let resolved = ResolvedAmbiguity {
                original_ambiguity: ambiguity.clone(),
                chosen_interpretation: first_interpretation.clone(),
                confidence: 0.3, // Low confidence for default resolution
                reasoning: "Default resolution - chose first available interpretation".to_string(),
                context_factors: vec![
                    ContextFactor {
                        factor_type: ContextFactorType::UserIntent,
                        description: "No strong contextual evidence, using default".to_string(),
                        weight: 0.3,
                    }
                ],
            };
            return Ok(Some(resolved));
        }

        Ok(None)
    }

    fn apply_resolution_to_intent(
        &self,
        resolved: &ResolvedAmbiguity,
        intent: &mut ProgramIntent,
    ) -> Result<()> {
        debug!("Applying resolution: {} -> {}", 
               resolved.original_ambiguity.description, 
               resolved.chosen_interpretation);

        // Apply the resolution based on the ambiguity type
        match resolved.original_ambiguity.id.as_str() {
            "pronoun_ambiguity" => {
                self.apply_pronoun_resolution(resolved, intent)?;
            }
            "operation_ambiguity" => {
                self.apply_operation_resolution(resolved, intent)?;
            }
            _ => {
                // Generic resolution application
                debug!("Applied generic resolution for ambiguity: {}", 
                       resolved.original_ambiguity.id);
            }
        }

        Ok(())
    }

    fn apply_pronoun_resolution(
        &self,
        resolved: &ResolvedAmbiguity,
        intent: &mut ProgramIntent,
    ) -> Result<()> {
        // Replace pronoun references in operations
        for operation in &mut intent.operations {
            for input in &mut operation.inputs {
                if input == "it" || input == "this" || input == "that" {
                    *input = resolved.chosen_interpretation.clone();
                }
            }
            
            // Update operation description
            operation.description = operation.description
                .replace("it", &resolved.chosen_interpretation)
                .replace("this", &resolved.chosen_interpretation)
                .replace("that", &resolved.chosen_interpretation);
        }

        Ok(())
    }

    fn apply_operation_resolution(
        &self,
        resolved: &ResolvedAmbiguity,
        intent: &mut ProgramIntent,
    ) -> Result<()> {
        // Update operation types based on resolution
        for operation in &mut intent.operations {
            if operation.description.contains("calculate") {
                match resolved.chosen_interpretation.as_str() {
                    "addition_operation" => {
                        operation.operation_type = super::intent_extractor::OperationType::Arithmetic {
                            operator: super::intent_extractor::ArithmeticOp::Add,
                        };
                    }
                    "multiplication_operation" => {
                        operation.operation_type = super::intent_extractor::OperationType::Arithmetic {
                            operator: super::intent_extractor::ArithmeticOp::Multiply,
                        };
                    }
                    "floating_point_operation" => {
                        // Keep existing operation but ensure floating-point handling
                        operation.description = format!("{} (floating-point)", operation.description);
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// Get resolution statistics
    pub fn get_resolution_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        stats.insert("total_rules".to_string(), self.resolution_rules.len());
        stats.insert("context_history_size".to_string(), self.context_history.len());
        stats
    }

    /// Add custom resolution rule
    pub fn add_resolution_rule(&mut self, rule: ResolutionRule) {
        self.resolution_rules.push(rule);
    }
}