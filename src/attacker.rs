use bevy::{
    prelude::*
};

use crate::{
    config::*,
};

#[derive(Component)]
pub struct Missile {
    x: i32,
    y: i32,
}

#[derive(Component)]
pub struct Attacker {
    ticks_since_last_missile: i32,
    missiles_left: i32,
    infinite: bool,
}

pub fn setup(mut commands: Commands, config: Res<DefendersConfig>) {
    commands
        .spawn()
        .insert(new_attacker(config.attacker_num_missiles));
}

fn new_attacker(missiles: i32) -> Attacker {
    return Attacker {
        ticks_since_last_missile: 0,
        missiles_left: missiles,
        infinite: missiles == 0,
    }
}

pub fn attacker_system(mut commands: Commands, mut query: Query<&mut Attacker>) {
    let mut attacker = query.single_mut();

    if !attacker.infinite & (attacker.missiles_left == 0) {
        return;
    }

    attacker.ticks_since_last_missile += 1;

    if attacker.ticks_since_last_missile > 5 {
        attacker.ticks_since_last_missile = 0;

        if attacker.missiles_left > 0 {
            attacker.missiles_left -= 1;
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(0.0, 250.0, 0.0),
                        scale: Vec3::new(10.0, 20.0, 0.0),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(1.0, 0.25, 0.25),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Missile { x: 10, y: 0 })
                .insert(Collider::Paddle);
        }
    }
}

pub fn missile_movement_system(mut query: Query<(&Missile, &mut Transform)>) {
    for (_, mut transform) in &mut query.iter_mut() {
        let translation = &mut transform.translation;
        translation.y -= 10.0;
    }
}

pub fn missile_collision_system() {

}
