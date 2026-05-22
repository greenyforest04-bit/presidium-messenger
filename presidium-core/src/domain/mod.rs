pub mod aggregates;
pub mod entities;
pub mod errors;
pub mod events;
pub mod value_objects;

// No public re-export from aggregates yet (empty placeholder)
pub use errors::DomainError;
pub use events::DomainEvent;
pub use value_objects::{DeviceId, MessageId, PreKeyBundle, SessionId, Timestamp, UserId};
