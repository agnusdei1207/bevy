use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub class_id: Option<i32>,
    pub req_level: i32,
    pub mp_cost: i32,
    pub cooldown_ms: i32,
    pub description: Option<String>,
    pub effect_type: Option<String>,
    pub base_value: i32,
    pub icon_path: Option<String>,
}
