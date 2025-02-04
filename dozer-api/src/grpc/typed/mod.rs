use prost_reflect::DynamicMessage;
mod codec;
mod service;

mod helper;
pub use service::TypedService;

#[cfg(test)]
pub mod tests;

#[derive(Debug, Clone)]
pub struct TypedRequest {
    pub message: DynamicMessage,
}
impl TypedRequest {
    pub fn new(message: DynamicMessage) -> Self {
        Self { message }
    }
}

#[derive(Debug, Clone)]
pub struct TypedResponse {
    pub message: DynamicMessage,
}

impl TypedResponse {
    pub fn new(message: DynamicMessage) -> Self {
        Self { message }
    }
}

pub const SERVICE_NAME: &str = "typed";
