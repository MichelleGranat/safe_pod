// Define modules
mod zeroable;
mod pod;

// Re-exports
pub use zeroable::Zeroable;
pub use pod::{Pod, PodError};
pub use safe_pod_derive::{
    Zeroable,
    Pod
};
