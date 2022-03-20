use rand::prelude::random;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide},
};

use crate::{
    config::*,
    scoreboard::*,
    menu::*,
};

#[derive(Component)]
pub struct Missile {
    speed: f32
}

#[derive(Component)]
pub struct Attacker {
    missiles_left: i32,
    infinite: bool,
}

pub fn setup(mut commands: Commands, defenders_config: Res<DefendersConfig>) {
    commands
        .spawn()
        .insert(new_attacker(defenders_config.attacker_ammo));
}

fn new_attacker(missiles: i32) -> Attacker {
    return Attacker {
        missiles_left: missiles,
        infinite: missiles == 0,
    }
}

pub fn attacker_system(menu: Res<Menu>, mut commands: Commands, mut query: Query<&mut Attacker>) {
    if menu.showing {
        return;
    }
    
    let mut attacker = query.single_mut();

    if !attacker.infinite && (attacker.missiles_left == 0) {
        return;
    }

    if attacker.missiles_left > 0 {
        attacker.missiles_left -= 1;

        let x_neg = if random::<i32>() % 2 == 0 {1.0} else {-1.0};
        let x = x_neg * 50.0 * random::<f32>();
        let spd = 4.0 + 3.0 * random::<f32>();

        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(10.0 * x, 300.0, 0.0),
                    scale: Vec3::new(5.0, 30.0, 0.0),
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

pub fn missile_movement_system(mut query: Query<(&Missile, &mut Transform)>) {
    for (missile, mut transform) in &mut query.iter_mut() {
        let translation = &mut transform.translation;
        translation.y -= missile.speed;
    }
}

pub fn missile_collision_system(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    query: Query<(Entity, &Missile, &Transform)>,
    collider_query: Query<(Entity, &Collider, &Transform)>
) {
    for (missile_entity, _missile, missile_transform) in query.iter() {
        for (collider_entity, collider, collider_transform) in collider_query.iter() {
            let missile_size = missile_transform.scale.truncate();
            let collider_size = collider_transform.scale.truncate();

            let collision = collide(
                missile_transform.translation,
                missile_size,
                collider_transform.translation,
                collider_size
            );

            if let Some(_collision) = collision {
                if let Collider::Scorable = *collider {
                    scoreboard.attacker_score += 1;
                    scoreboard.defender_health -= 1;

                    commands.entity(collider_entity).despawn();
                    commands
                        .spawn_bundle(SpriteBundle {
                            transform: Transform {
                                translation: collider_transform.translation,
                                scale: collider_transform.scale,
                                ..Default::default()
                            },
                            sprite: Sprite {
                                color: Color::rgb(1.0, 0.0, 0.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Collider::Solid);
                } else if let Collider::Paddle = *collider {
                    scoreboard.defender_score += 1;
                }

                scoreboard.attacker_ammo -= 1;
                commands.entity(missile_entity).despawn();
                return
            }
        }
    }
}
