use crate::shared::domain::*;

pub fn move_towards(from: &Position, to: &Position, speed: f64, delta_time: f64) -> Position {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let distance = (dx * dx + dy * dy).sqrt();
    
    if distance < speed * delta_time {
        *to
    } else {
        let move_distance = speed * delta_time;
        Position {
            x: from.x + (dx / distance) * move_distance,
            y: from.y + (dy / distance) * move_distance,
        }
    }
}

