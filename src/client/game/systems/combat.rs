// Combat utility functions

pub fn calculate_damage(attacker_attack: i32, defender_defense: i32) -> i32 {
    let base_damage = attacker_attack - defender_defense / 2;
    base_damage.max(1)
}

pub fn is_critical_hit(critical_rate: i32) -> bool {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(0..100) < critical_rate
}

pub fn calculate_drop_gold(min: i32, max: i32) -> i32 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

