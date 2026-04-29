use macroquad::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec2, D::Error>
where
    D: Deserializer<'de>,
{
    let [x, y] = <[f32; 2]>::deserialize(deserializer)?;
    Ok(Vec2::new(x, y))
}

pub fn serialize<S>(vec: &Vec2, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    [vec.x, vec.y].serialize(serializer)
}
