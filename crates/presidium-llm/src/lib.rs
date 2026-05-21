//! # Presidium LLM
//!
//! On-device LLM inference subsystem for Presidium Messenger.
//!
//! This crate manages local GGUF model inference for:
//! - Content moderation (extremism, CSAM, fraud detection)
//! - AI assistant (RAG over chat history, tool use)
//! - Semantic embeddings for search
//!
//! ## Model Strategy
//!
//! - Default model: Gemma-2B or Phi-3 (4-bit quantized GGUF)
//! - Inference engine: candle.rs or llama-cpp-rs
//! - Hardware acceleration: NPU/GPU when available, CPU fallback
//!
//! ## Privacy
//!
//! All inference happens on-device. No data leaves the user's device.
//! Models must be open-source GGUF with no hidden layers.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod application;
pub mod domain;
pub mod infrastructure;
