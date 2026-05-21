//! # LLM Port Definitions
//!
//! Port traits for the on-device LLM subsystem.

use crate::domain::inference::{InferenceResult, ModerationAnalysis};
use crate::domain::model::ModelInfo;
use async_trait::async_trait;
use presidium_core::domain::errors::DomainError;

/// Port for LLM model loading and management.
#[async_trait]
pub trait ModelManagerPort: Send + Sync {
    /// Loads a GGUF model from the given file path.
    async fn load_model(&self, path: &str) -> Result<ModelInfo, DomainError>;

    /// Returns information about the currently loaded model.
    async fn current_model(&self) -> Result<Option<ModelInfo>, DomainError>;

    /// Unloads the current model, freeing GPU/NPU memory.
    async fn unload_model(&self) -> Result<(), DomainError>;
}

/// Port for performing LLM inference.
#[async_trait]
pub trait InferencePort: Send + Sync {
    /// Generates text given a prompt.
    async fn generate(&self, prompt: &str) -> Result<InferenceResult, DomainError>;

    /// Performs content moderation analysis.
    async fn moderate(&self, content: &str) -> Result<ModerationAnalysis, DomainError>;

    /// Cancels any in-progress inference.
    async fn cancel(&self) -> Result<(), DomainError>;
}

#[cfg(test)]
mod tests {
    fn _assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn test_port_traits_are_object_safe() {
        fn _use(_: &dyn crate::application::ports::ModelManagerPort) {}
        fn _use2(_: &dyn crate::application::ports::InferencePort) {}
    }
}
