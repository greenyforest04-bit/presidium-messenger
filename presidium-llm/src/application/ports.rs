use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum LLMError {
    #[error("Model loading failed: {0}")]
    ModelLoading(String),
    #[error("Inference failed: {0}")]
    Inference(String),
    #[error("Model not loaded")]
    NotLoaded,
}

#[async_trait]
pub trait LLMPort: Send + Sync {
    async fn load_model(&self, model_path: &str, quant: u32) -> Result<(), LLMError>;
    async fn infer(&self, prompt: &str, max_tokens: usize) -> Result<String, LLMError>;
    fn is_loaded(&self) -> bool;
}
