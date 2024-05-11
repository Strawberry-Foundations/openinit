use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct OpenService {
    pub service: Service,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Service {
    pub name: String,
    pub description: String,
    pub target: String,
    pub level: String,
    pub command: String,
}