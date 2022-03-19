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
    const TIME_STEP: f32 = 30.0 / 60.0;

    let config = config::DefendersConfig::new("./config/config-example.txt".to_string());

    App::new()
        .insert_resource(config)
        .add_plugins(MinimalPlugins)
        .add_plugin(InputPlugin)
        .add_startup_system(board::setup)
        .add_startup_system(attacker::setup)
        .add_startup_system(defender::setup)
        .add_system(defender::defender_movement_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(board::board_system)
                .with_system(attacker::attacker_system)
                .with_system(attacker::missile_movement_system)
                .with_system(attacker::missile_collision_system))
        .run();
}
