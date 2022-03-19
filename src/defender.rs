use bevy::{
    prelude::*
};

#[derive(Component)]
pub struct Defender {
    pub x: usize,
    speed : usize,
}

pub fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(new_defender(1));
}

fn new_defender(speed: usize) -> Defender {
    return Defender {
        x: 0,
        speed: speed,
    }
}

pub fn defender_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Defender>
) {
    let mut defender = query.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        defender.x -= defender.speed;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        print!("{}", defender.x);
        defender.x += defender.speed;
    }
}

