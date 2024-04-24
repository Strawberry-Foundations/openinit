use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct OpenService {
    pub service: Service,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Service {
    pub name: String,
    pub description: String,
    pub level: String,
    pub command: String,
}