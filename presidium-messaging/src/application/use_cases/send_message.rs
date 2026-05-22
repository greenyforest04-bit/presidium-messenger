use async_trait::async_trait;
use presidium_core::application::ports::{ContentVerdict, ModerationError, ModerationPort};
use presidium_core::domain::{DeviceId, MessageId, PreKeyBundle, Timestamp, UserId};
use presidium_crypto::application::ports::{CryptoError, E2EECryptoPort};
use presidium_p2p::application::ports::{NetworkError, P2PPort};
use presidium_storage::application::ports::{MessageStoragePort, StorageError, StoredMessage};

#[derive(Debug)]
pub struct SendMessageInput {
    pub sender: UserId,
    pub sender_device: DeviceId,
    pub recipient: UserId,
    pub recipient_device: DeviceId,
    pub plaintext: String,
}

#[derive(Debug)]
pub struct SendMessageOutput {
    pub message_id: MessageId,
    pub timestamp: Timestamp,
}

#[derive(Debug, thiserror::Error)]
pub enum SendMessageError {
    #[error("Moderation failed: {0}")]
    Moderation(#[from] ModerationError),
    #[error("Crypto error: {0}")]
    Crypto(#[from] CryptoError),
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("Message rejected: {0}")]
    Rejected(String),
}

#[async_trait]
pub trait SendMessageUseCase: Send + Sync {
    async fn execute(&self, input: SendMessageInput)
        -> Result<SendMessageOutput, SendMessageError>;
}

pub struct SendMessageInteractor<C, P, S, M>
where
    C: E2EECryptoPort,
    P: P2PPort,
    S: MessageStoragePort,
    M: ModerationPort,
{
    crypto: C,
    p2p: P,
    storage: S,
    moderation: M,
}

impl<C, P, S, M> SendMessageInteractor<C, P, S, M>
where
    C: E2EECryptoPort,
    P: P2PPort,
    S: MessageStoragePort,
    M: ModerationPort,
{
    pub fn new(crypto: C, p2p: P, storage: S, moderation: M) -> Self {
        Self { crypto, p2p, storage, moderation }
    }
}

#[async_trait]
impl<C, P, S, M> SendMessageUseCase for SendMessageInteractor<C, P, S, M>
where
    C: E2EECryptoPort + Send + Sync,
    P: P2PPort + Send + Sync,
    S: MessageStoragePort + Send + Sync,
    M: ModerationPort + Send + Sync,
{
    async fn execute(
        &self, input: SendMessageInput,
    ) -> Result<SendMessageOutput, SendMessageError> {
        // 1. Moderation check — local, on-device, does not leak plaintext
        let verdict = self.moderation.check_message(&input.sender, &input.plaintext).await?;
        if let ContentVerdict::Unsafe(reason) = verdict {
            // Create sarcophagus (encrypted evidence package for law enforcement)
            let _sarcophagus = self
                .moderation
                .create_sarcophagus(&input.sender, &input.plaintext, &reason)
                .await?;
            return Err(SendMessageError::Rejected(format!("Content blocked: {}", reason)));
        }

        // 2. Fetch remote pre-key bundle and establish E2EE session
        // TODO(Day 7+): Replace with real pre-key fetch via P2P DHT
        let bundle = dummy_bundle();
        let session_id = self
            .crypto
            .establish_session(&input.recipient, &input.recipient_device, &bundle)
            .await?;

        // 3. Encrypt the plaintext message using Double Ratchet
        let ciphertext =
            self.crypto.encrypt_message(&session_id, input.plaintext.as_bytes()).await?;

        // 4. Store outgoing message in local database
        let message_id = MessageId::new();
        let timestamp = Timestamp::now();
        let stored_msg = StoredMessage {
            id: message_id.clone(),
            sender: input.sender.clone(),
            recipient: input.recipient.clone(),
            ciphertext: ciphertext.clone(),
            timestamp: timestamp.clone(),
            delivered: false,
            read: false,
        };
        self.storage.save_outgoing_message(stored_msg).await?;

        // 5. Send ciphertext via P2P network
        self.p2p.send_p2p(&input.recipient_device, ciphertext).await?;

        Ok(SendMessageOutput { message_id, timestamp })
    }
}

/// Temporary stub for PreKeyBundle — will be replaced by real key fetch
/// TODO(Day 7+): Implement real pre-key bundle retrieval from P2P DHT
fn dummy_bundle() -> PreKeyBundle {
    PreKeyBundle {
        identity_key: vec![0u8; 32],
        signed_pre_key: vec![0u8; 32],
        pre_key_signature: vec![0u8; 64],
        one_time_pre_key: Some(vec![0u8; 32]),
        registration_id: 0,
    }
}
