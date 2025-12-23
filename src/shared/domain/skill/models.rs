use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub icon_path: Option<String>,
    pub class_req: Option<String>,
    pub level_req: i32,
    pub mp_cost: i32,
    pub cooldown: i32,
    pub damage_min: i32,
    pub damage_max: i32,
}
