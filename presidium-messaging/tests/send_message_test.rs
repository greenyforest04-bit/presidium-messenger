use mockall::mock;
use mockall::predicate::*;
use presidium_core::application::ports::{ContentVerdict, ModerationError, ModerationPort};
use presidium_core::domain::{DeviceId, MessageId, PreKeyBundle, SessionId, UserId};
use presidium_crypto::application::ports::{CryptoError, E2EECryptoPort};
use presidium_messaging::application::use_cases::{
    SendMessageInput, SendMessageInteractor, SendMessageUseCase,
};
use presidium_p2p::application::ports::{NetworkError, P2PPort};
use presidium_storage::application::ports::{MessageStoragePort, StorageError, StoredMessage};

mock! {
    pub Crypto {}

    #[async_trait::async_trait]
    impl E2EECryptoPort for Crypto {
        async fn generate_pre_key_bundle(
            &self, device_id: &DeviceId,
        ) -> Result<PreKeyBundle, CryptoError>;
        async fn establish_session(
            &self, remote_user: &UserId, remote_device: &DeviceId, bundle: &PreKeyBundle,
        ) -> Result<SessionId, CryptoError>;
        async fn encrypt_message(
            &self, session_id: &SessionId, plaintext: &[u8],
        ) -> Result<Vec<u8>, CryptoError>;
        async fn decrypt_message(
            &self, session_id: &SessionId, ciphertext: &[u8],
        ) -> Result<Vec<u8>, CryptoError>;
        async fn rotate_ratchet(&self, session_id: &SessionId) -> Result<(), CryptoError>;
    }
}

mock! {
    pub P2P {}

    #[async_trait::async_trait]
    impl P2PPort for P2P {
        async fn send_p2p(
            &self, target_device: &DeviceId, data: Vec<u8>,
        ) -> Result<(), NetworkError>;
        async fn receive_p2p(&self) -> Result<Option<Vec<u8>>, NetworkError>;
        async fn publish_pre_keys(
            &self, user_id: &UserId, device_id: &DeviceId, bundle: &[u8],
        ) -> Result<(), NetworkError>;
        async fn fetch_pre_keys(
            &self, user_id: &UserId, device_id: &DeviceId,
        ) -> Result<Vec<u8>, NetworkError>;
        async fn subscribe_topic(&self, topic: &str) -> Result<(), NetworkError>;
    }
}

mock! {
    pub Storage {}

    #[async_trait::async_trait]
    impl MessageStoragePort for Storage {
        async fn save_outgoing_message(&self, msg: StoredMessage) -> Result<(), StorageError>;
        async fn save_incoming_message(&self, msg: StoredMessage) -> Result<(), StorageError>;
        async fn mark_delivered(&self, msg_id: &MessageId) -> Result<(), StorageError>;
        async fn mark_read(&self, msg_id: &MessageId) -> Result<(), StorageError>;
        async fn get_messages_for_user(
            &self, user_id: &UserId, limit: u32,
        ) -> Result<Vec<StoredMessage>, StorageError>;
    }
}

mock! {
    pub Moderation {}

    #[async_trait::async_trait]
    impl ModerationPort for Moderation {
        async fn check_message(
            &self, sender: &UserId, plaintext: &str,
        ) -> Result<ContentVerdict, ModerationError>;
        async fn create_sarcophagus(
            &self, offender: &UserId, evidence: &str, reason: &str,
        ) -> Result<Vec<u8>, ModerationError>;
    }
}

fn make_input() -> SendMessageInput {
    SendMessageInput {
        sender: UserId::new("alice"),
        sender_device: DeviceId::new("alice_phone"),
        recipient: UserId::new("bob"),
        recipient_device: DeviceId::new("bob_laptop"),
        plaintext: "Hello, Bob!".to_string(),
    }
}

#[tokio::test]
async fn test_send_message_success() {
    let mut mock_crypto = MockCrypto::new();
    mock_crypto
        .expect_establish_session()
        .returning(|_, _, _| Ok(SessionId::default()));
    mock_crypto
        .expect_encrypt_message()
        .returning(|_, _| Ok(b"ciphertext".to_vec()));

    let mut mock_p2p = MockP2P::new();
    mock_p2p.expect_send_p2p().returning(|_, _| Ok(()));

    let mut mock_storage = MockStorage::new();
    mock_storage.expect_save_outgoing_message().returning(|_| Ok(()));

    let mut mock_moderation = MockModeration::new();
    mock_moderation
        .expect_check_message()
        .returning(|_, _| Ok(ContentVerdict::Safe));

    let interactor =
        SendMessageInteractor::new(mock_crypto, mock_p2p, mock_storage, mock_moderation);
    let result = interactor.execute(make_input()).await;
    assert!(result.is_ok(), "Expected success, got: {:?}", result.err());
}

#[tokio::test]
async fn test_send_message_blocked_by_moderation() {
    let mock_crypto = MockCrypto::new();
    let mock_p2p = MockP2P::new();
    let mock_storage = MockStorage::new();

    let mut mock_moderation = MockModeration::new();
    mock_moderation
        .expect_check_message()
        .returning(|_, _| Ok(ContentVerdict::Unsafe("violates policy".to_string())));
    mock_moderation
        .expect_create_sarcophagus()
        .returning(|_, _, _| Ok(vec![1, 2, 3]));

    let interactor =
        SendMessageInteractor::new(mock_crypto, mock_p2p, mock_storage, mock_moderation);
    let result = interactor.execute(make_input()).await;
    assert!(result.is_err());
    let err_msg = format!("{}", result.unwrap_err());
    assert!(err_msg.contains("Content blocked"), "Expected rejection, got: {}", err_msg);
}

#[tokio::test]
async fn test_send_message_crypto_failure() {
    let mut mock_crypto = MockCrypto::new();
    mock_crypto
        .expect_establish_session()
        .returning(|_, _, _| Err(CryptoError::SessionEstablishment("key error".to_string())));

    let mock_p2p = MockP2P::new();
    let mock_storage = MockStorage::new();

    let mut mock_moderation = MockModeration::new();
    mock_moderation
        .expect_check_message()
        .returning(|_, _| Ok(ContentVerdict::Safe));

    let interactor =
        SendMessageInteractor::new(mock_crypto, mock_p2p, mock_storage, mock_moderation);
    let result = interactor.execute(make_input()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_send_message_network_failure() {
    let mut mock_crypto = MockCrypto::new();
    mock_crypto
        .expect_establish_session()
        .returning(|_, _, _| Ok(SessionId::default()));
    mock_crypto
        .expect_encrypt_message()
        .returning(|_, _| Ok(b"ciphertext".to_vec()));

    let mut mock_p2p = MockP2P::new();
    mock_p2p
        .expect_send_p2p()
        .returning(|_, _| Err(NetworkError::SendFailed("timeout".to_string())));

    let mut mock_storage = MockStorage::new();
    mock_storage.expect_save_outgoing_message().returning(|_| Ok(()));

    let mut mock_moderation = MockModeration::new();
    mock_moderation
        .expect_check_message()
        .returning(|_, _| Ok(ContentVerdict::Safe));

    let interactor =
        SendMessageInteractor::new(mock_crypto, mock_p2p, mock_storage, mock_moderation);
    let result = interactor.execute(make_input()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_send_message_storage_failure() {
    let mut mock_crypto = MockCrypto::new();
    mock_crypto
        .expect_establish_session()
        .returning(|_, _, _| Ok(SessionId::default()));
    mock_crypto
        .expect_encrypt_message()
        .returning(|_, _| Ok(b"ciphertext".to_vec()));

    let mock_p2p = MockP2P::new();

    let mut mock_storage = MockStorage::new();
    mock_storage
        .expect_save_outgoing_message()
        .returning(|_| Err(StorageError::SaveFailed("disk full".to_string())));

    let mut mock_moderation = MockModeration::new();
    mock_moderation
        .expect_check_message()
        .returning(|_, _| Ok(ContentVerdict::Safe));

    let interactor =
        SendMessageInteractor::new(mock_crypto, mock_p2p, mock_storage, mock_moderation);
    let result = interactor.execute(make_input()).await;
    assert!(result.is_err());
}
