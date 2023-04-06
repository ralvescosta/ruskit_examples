use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ToDoRequest {}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ToDoResponse {}
