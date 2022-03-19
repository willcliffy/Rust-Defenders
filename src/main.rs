use bevy::{
    input::InputPlugin,
    core::FixedTimestep,
    prelude::*,
};

mod attacker;
mod defender;
mod board;
mod config;

fn main() {
    const TIME_STEP: f32 = 1.0 / 60.0;

    let config = config::DefendersConfig::new("./config/config-example.txt".to_string());

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(InputPlugin)
        .insert_resource(config)
        .add_startup_system(board::setup)
        .add_startup_system(attacker::setup)
        .add_startup_system(defender::setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(board::board_system)
                .with_system(attacker::attacker_system)
                .with_system(attacker::missile_movement_system)
                .with_system(attacker::missile_collision_system)
                .with_system(defender::defender_movement_system))
        .run();
}
