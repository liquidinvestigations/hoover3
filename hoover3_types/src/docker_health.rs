#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, Hash,
)]
pub struct ContainerHealthUi {
    pub container_id: String,
    pub container_name: String,
    pub container_running: String,
    pub container_health: String,
}
