use bevy::{
    prelude::*
};

#[derive(Component)]
pub enum Collider {
    Solid,
    Scorable,
    Paddle,
}

#[derive(Component)]
pub struct DefendersConfig {
    pub skyline: Vec<f32>,

    pub attacker_num_missiles: i32,

    // attacker_name: String,
    // defender_name: String,
}

pub fn default_config() -> DefendersConfig {
    return DefendersConfig{
        skyline: vec![2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 6.0, 6.0, 7.0, 7.0, 7.0, 5.0, 5.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 8.0, 8.0, 8.0, 6.0, 6.0, 6.0, 9.0, 9.0, 9.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 10.0, 10.0, 10.0, 4.0, 4.0, 4.0, 9.0, 9.0, 9.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0, 6.0, 6.0, 6.0, 9.0, 9.0, 9.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0, 6.0, 6.0, 6.0, 9.0, 9.0, 9.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0],
        attacker_num_missiles: 50,
    }
}
