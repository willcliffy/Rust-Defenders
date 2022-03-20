use bevy::{
    core::FixedTimestep,
    prelude::*,
};

mod menu;
mod attacker;
mod defender;
mod config;
mod scoreboard;

fn main() {
    const TIME_STEP: f32 = 1.0 / 60.0;

    let defenders_config = config::default_config();
    let scoreboard = scoreboard::new_scoreboard(&defenders_config);
    let menu = menu::new_menu();

    App::new()
        .insert_resource(defenders_config)
        .insert_resource(scoreboard)
        .insert_resource(menu)
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .add_startup_system(scoreboard::setup)
        .add_startup_system(attacker::setup)
        .add_startup_system(defender::setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(10.0 * TIME_STEP as f64))
                .with_system(menu::menu_system)
                .with_system(attacker::attacker_system)
                .with_system(scoreboard::scoreboard_system))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(attacker::missile_movement_system)
                .with_system(attacker::missile_collision_system)
                .with_system(defender::defender_movement_system))
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn startup(mut commands: Commands, config: Res<config::DefendersConfig>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let conf: &config::DefendersConfig = config.as_ref();

    let mut x = -1100.0 / 2.0;

    let mut last_y = 2;

    for y in &conf.skyline {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x as f32, *y as f32 * 10.0- 250.0, 0.0),
                    scale: Vec3::new(10.0, 10.0, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(config::Collider::Scorable);

        if *y < last_y as f32 {
            for i in 2..last_y {
                commands
                    .spawn_bundle(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(x-10.0, i as f32 * 10.0 - 250.0, 0.0),
                            scale: Vec3::new(10.0, 10.0, 0.0),
                            ..Default::default()
                        },
                        sprite: Sprite {
                            color: Color::rgb(0.0, 0.0, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
            }
        } else if *y > last_y as f32 {
            for i in 2..*y as usize {
                commands
                    .spawn_bundle(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(x, i as f32 * 10.0 - 250.0, 0.0),
                            scale: Vec3::new(10.0, 10.0, 0.0),
                            ..Default::default()
                        },
                        sprite: Sprite {
                            color: Color::rgb(0.0, 0.0, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
            }
        }

        last_y = *y as usize;
        x += 10.0;
    }
}