use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Owner(pub Uuid);