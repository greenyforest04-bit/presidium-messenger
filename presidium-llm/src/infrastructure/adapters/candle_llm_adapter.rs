use crate::application::ports::{LLMError, LLMPort};
use async_trait::async_trait;

/// Stub adapter — will be replaced by candle.rs / llama-cpp-rs integration
/// TODO(Day 8+): Implement real on-device LLM inference with GGUF model loading
pub struct CandleLlmAdapter;

impl Default for CandleLlmAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl CandleLlmAdapter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl LLMPort for CandleLlmAdapter {
    async fn load_model(&self, _model_path: &str, _quant: u32) -> Result<(), LLMError> {
        unimplemented!("TODO: Implement GGUF model loading with candle.rs")
    }

    async fn infer(&self, _prompt: &str, _max_tokens: usize) -> Result<String, LLMError> {
        unimplemented!("TODO: Implement LLM inference with candle.rs")
    }

    fn is_loaded(&self) -> bool {
        false
    }
}
