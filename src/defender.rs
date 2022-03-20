use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

use crate::{
    config::*,
    menu::*,
};

#[derive(Component)]
pub struct Defender {
    pub x: usize,
    speed : f32,
}

pub fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -100.0, 0.0),
                scale: Vec3::new(120.0, 20.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(new_defender(15.0))
        .insert(Collider::Paddle);
}

fn new_defender(speed: f32) -> Defender {
    return Defender {
        x: 52,
        speed: speed,
    }
}

pub fn defender_movement_system(
    mut menu: ResMut<Menu>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Defender, &mut Transform)>,
) {
    let (defender, mut transform) = query.single_mut();
    let translation = &mut transform.translation;

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        translation.x -= defender.speed;
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        translation.x += defender.speed;
    }

    if menu.showing {
        if keyboard_input.pressed(KeyCode::Space) {
            menu.showing = false;
        }
    }
}

