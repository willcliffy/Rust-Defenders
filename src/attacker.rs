use rand::prelude::random;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{
    config::*,
};

#[derive(Component)]
pub struct Missile {
    speed: f32
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

    if !attacker.infinite && (attacker.missiles_left == 0) {
        return;
    }

    attacker.ticks_since_last_missile += 1;

    if attacker.ticks_since_last_missile > 3 {
        attacker.ticks_since_last_missile = 0;

        if attacker.missiles_left > 0 {
            attacker.missiles_left -= 1;

            let x_neg = if random::<i32>() % 2 == 0 {1.0} else {-1.0};
            let x = x_neg * 50.0 * random::<f32>();
            let spd = 4.0 + 3.0 * random::<f32>();

            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(10.0 * x, 300.0, 0.0),
                        scale: Vec3::new(10.0, 20.0, 0.0),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(1.0, 0.25, 0.25),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Missile { speed: spd });
        }
    }
}

pub fn missile_movement_system(mut query: Query<(&Missile, &mut Transform)>) {
    for (missile, mut transform) in &mut query.iter_mut() {
        let translation = &mut transform.translation;
        translation.y -= missile.speed;
    }
}

pub fn missile_collision_system(
    mut commands: Commands,
    query: Query<(Entity, &Missile, &Transform)>,
    collider_query: Query<(Entity, &Collider, &Transform)>
) {
    // TODO - I left off here, working on collision logic
    for (missile_entity, _missile, missile_transform) in query.iter() {
        for (_collider_entity, _collider, collider_transform) in collider_query.iter() {
            let missile_size = missile_transform.scale.truncate();
            let collider_size = collider_transform.scale.truncate();

            let collision = collide(
                missile_transform.translation,
                missile_size,
                collider_transform.translation,
                collider_size
            );

            if let Some(_collision) = collision {
                commands.entity(missile_entity).despawn();
            }
        }
    }
}
