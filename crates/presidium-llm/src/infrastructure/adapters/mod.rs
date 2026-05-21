//! # LLM Adapters
//!
//! Stub implementations of LLM ports for development.

use crate::application::ports::{InferencePort, ModelManagerPort};
use crate::domain::inference::{InferenceResult, ModerationAnalysis};
use crate::domain::model::{ModelCapability, ModelInfo, Quantization};
use async_trait::async_trait;
use presidium_core::domain::errors::DomainError;
use tokio::sync::RwLock;

/// Stub LLM adapter that simulates inference without a real model.
pub struct StubLlmAdapter {
    /// Simulated loaded model state.
    loaded: RwLock<bool>,
}

impl StubLlmAdapter {
    /// Creates a new stub LLM adapter.
    #[must_use]
    pub fn new() -> Self {
        Self {
            loaded: RwLock::new(false),
        }
    }
}

impl Default for StubLlmAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ModelManagerPort for StubLlmAdapter {
    async fn load_model(&self, path: &str) -> Result<ModelInfo, DomainError> {
        let mut loaded = self.loaded.write().await;
        *loaded = true;
        tracing::info!(target: "presidium::llm", path = path, "stub: model loaded");
        Ok(ModelInfo {
            name: "stub-model-2b".into(),
            version: "0.1.0".into(),
            context_size: 2048,
            quantization: Quantization::Q4,
            file_size_bytes: 1_200_000_000,
            capabilities: vec![ModelCapability::TextGeneration, ModelCapability::Moderation],
        })
    }

    async fn current_model(&self) -> Result<Option<ModelInfo>, DomainError> {
        let loaded = self.loaded.read().await;
        if *loaded {
            Ok(Some(ModelInfo {
                name: "stub-model-2b".into(),
                version: "0.1.0".into(),
                context_size: 2048,
                quantization: Quantization::Q4,
                file_size_bytes: 1_200_000_000,
                capabilities: vec![ModelCapability::TextGeneration, ModelCapability::Moderation],
            }))
        } else {
            Ok(None)
        }
    }

    async fn unload_model(&self) -> Result<(), DomainError> {
        let mut loaded = self.loaded.write().await;
        *loaded = false;
        Ok(())
    }
}

#[async_trait]
impl InferencePort for StubLlmAdapter {
    async fn generate(&self, prompt: &str) -> Result<InferenceResult, DomainError> {
        Ok(InferenceResult {
            text: format!("[stub response to: {prompt}]"),
            prompt_tokens: prompt.split_whitespace().count() as u32,
            completion_tokens: 10,
            inference_time_ms: 5,
        })
    }

    async fn moderate(&self, _content: &str) -> Result<ModerationAnalysis, DomainError> {
        Ok(ModerationAnalysis::safe())
    }

    async fn cancel(&self) -> Result<(), DomainError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use presidium_core::domain::value_objects::ModerationResult;

    #[tokio::test]
    async fn test_stub_llm_lifecycle() {
        let adapter = StubLlmAdapter::new();

        // No model loaded initially
        let current = adapter.current_model().await.expect("current");
        assert!(current.is_none());

        // Load a model
        let info = adapter.load_model("/models/stub.gguf").await.expect("load");
        assert_eq!(info.name, "stub-model-2b");

        // Model is now loaded
        let current = adapter.current_model().await.expect("current");
        assert!(current.is_some());

        // Generate text
        let result = adapter.generate("Hello").await.expect("generate");
        assert!(!result.text.is_empty());

        // Moderate content
        let analysis = adapter
            .moderate("innocent message")
            .await
            .expect("moderate");
        assert_eq!(analysis.result, ModerationResult::Safe);

        // Unload
        adapter.unload_model().await.expect("unload");
        let current = adapter.current_model().await.expect("current");
        assert!(current.is_none());
    }
}
