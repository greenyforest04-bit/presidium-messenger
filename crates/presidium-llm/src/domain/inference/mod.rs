//! # Inference Domain Types
//!
//! Types related to LLM inference operations.

use presidium_core::domain::value_objects::ModerationResult;
use serde::{Deserialize, Serialize};

/// The result of an LLM inference call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResult {
    /// The generated text output.
    pub text: String,
    /// Number of prompt tokens consumed.
    pub prompt_tokens: u32,
    /// Number of completion tokens generated.
    pub completion_tokens: u32,
    /// Wall-clock inference time in milliseconds.
    pub inference_time_ms: u64,
}

/// A structured moderation analysis result from the LLM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationAnalysis {
    /// The overall moderation decision.
    pub result: ModerationResult,
    /// Confidence score (0.0 to 1.0).
    pub confidence: f32,
    /// Detailed explanation (for debugging, not shown to users).
    pub explanation: String,
    /// Categories that were checked.
    pub categories_checked: Vec<String>,
}

impl ModerationAnalysis {
    /// Creates a "safe" analysis result.
    #[must_use]
    pub fn safe() -> Self {
        Self {
            result: ModerationResult::Safe,
            confidence: 1.0,
            explanation: "Content passed all moderation checks.".into(),
            categories_checked: vec![
                "extremism".into(),
                "terrorism".into(),
                "csam".into(),
                "fraud".into(),
                "drugs".into(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inference_result() {
        let result = InferenceResult {
            text: "Hello!".into(),
            prompt_tokens: 5,
            completion_tokens: 2,
            inference_time_ms: 150,
        };
        assert_eq!(result.prompt_tokens, 5);
    }

    #[test]
    fn test_moderation_analysis_safe() {
        let analysis = ModerationAnalysis::safe();
        assert_eq!(analysis.result, ModerationResult::Safe);
        assert!((analysis.confidence - 1.0).abs() < f32::EPSILON);
        assert!(!analysis.categories_checked.is_empty());
    }
}
