use bevy::{
    prelude::*
};

#[derive(Component)]
pub enum Collider {
    Solid,
    Scorable,
    Paddle,
}

#[derive(Component, PartialEq)]
pub enum TextType {
    Scoreboard,
    Menu
}

#[derive(Component)]
pub struct DefendersConfig {
    pub skyline: Vec<f32>,

    pub attacker_name: String,
    pub attacker_ammo: i32,
    
    pub defender_name: String,
    pub defender_hp: i32,
}

pub fn default_config() -> DefendersConfig {
    return DefendersConfig{
        skyline: vec![2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 6.0, 6.0, 7.0, 7.0, 7.0, 5.0, 5.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 8.0, 8.0, 8.0, 6.0, 6.0, 6.0, 9.0, 9.0, 9.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 10.0, 10.0, 10.0, 4.0, 4.0, 4.0, 9.0, 9.0, 9.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0, 6.0, 6.0, 6.0, 9.0, 9.0, 9.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0, 6.0, 6.0, 6.0, 9.0, 9.0, 9.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0],

        attacker_name: "Attacker".to_string(),
        attacker_ammo: 100,

        defender_name: "Defender".to_string(),
        defender_hp: 50,
    }
}
