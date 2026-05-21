//! # Model Domain Types
//!
//! Represents GGUF models and their configuration.

use serde::{Deserialize, Serialize};

/// Supported quantization levels for GGUF models.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Quantization {
    /// 4-bit quantization (smallest, fastest, least accurate).
    Q4,
    /// 5-bit quantization (1.58-bit efficient variants).
    Q5,
    /// 8-bit quantization (good balance of size and quality).
    Q8,
    /// Full precision (16-bit, no quantization).
    Fp16,
}

impl Quantization {
    /// Returns the approximate size multiplier relative to FP16.
    #[must_use]
    pub const fn size_multiplier(&self) -> f32 {
        match self {
            Self::Q4 => 0.25,
            Self::Q5 => 0.35,
            Self::Q8 => 0.50,
            Self::Fp16 => 1.0,
        }
    }

    /// Returns a string representation for file naming.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Q4 => "Q4_K_M",
            Self::Q5 => "Q5_K_M",
            Self::Q8 => "Q8_0",
            Self::Fp16 => "F16",
        }
    }
}

impl std::fmt::Display for Quantization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Metadata describing a loaded GGUF model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Human-readable model name (e.g., "gemma-2b-it").
    pub name: String,
    /// Model version or revision.
    pub version: String,
    /// Context window size (number of tokens).
    pub context_size: u32,
    /// Quantization level.
    pub quantization: Quantization,
    /// Approximate model file size in bytes.
    pub file_size_bytes: u64,
    /// Supported capabilities of this model.
    pub capabilities: Vec<ModelCapability>,
}

/// Capabilities that a model may support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelCapability {
    /// Text generation (chat completion).
    TextGeneration,
    /// Content moderation and classification.
    Moderation,
    /// Embedding generation for semantic search.
    Embeddings,
    /// Tool/function calling.
    ToolUse,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantization_size_multipliers() {
        assert!((Quantization::Q4.size_multiplier() - 0.25).abs() < f32::EPSILON);
        assert!((Quantization::Q8.size_multiplier() - 0.50).abs() < f32::EPSILON);
        assert!((Quantization::Fp16.size_multiplier() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_quantization_display() {
        assert_eq!(Quantization::Q4.to_string(), "Q4_K_M");
        assert_eq!(Quantization::Fp16.to_string(), "F16");
    }

    #[test]
    fn test_model_info_creation() {
        let info = ModelInfo {
            name: "gemma-2b-it".into(),
            version: "1.0".into(),
            context_size: 4096,
            quantization: Quantization::Q4,
            file_size_bytes: 1_500_000_000,
            capabilities: vec![ModelCapability::TextGeneration, ModelCapability::Moderation],
        };
        assert_eq!(info.context_size, 4096);
        assert_eq!(info.capabilities.len(), 2);
    }
}
