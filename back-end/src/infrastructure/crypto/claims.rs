use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}