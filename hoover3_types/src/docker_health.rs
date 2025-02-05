//! Types and structures related to Docker container health and status.

/// Represents the health and status information for a single Docker container
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, Hash,
)]
pub struct ContainerHealthUi {
    /// Unique identifier of the Docker container
    pub container_id: String,
    /// Name of the Docker container
    pub container_name: String,
    /// Current running status of the container
    pub container_running: String,
    /// Health status of the container
    pub container_health: String,
}
