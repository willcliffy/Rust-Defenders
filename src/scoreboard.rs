use bevy::{
    prelude::*,
};

use crate::{
    config::*,
};

#[derive(Component)]
pub struct Scoreboard {
    pub defender_name: String,
    pub defender_score: i32,
    pub defender_health: i32,
    
    pub attacker_name: String,
    pub attacker_score: i32,
    pub attacker_ammo: i32,

}

pub fn new_scoreboard(config: &DefendersConfig) -> Scoreboard {
    return Scoreboard {
        defender_name: config.defender_name.clone(),
        defender_score: 0,
        defender_health: config.defender_hp,
        attacker_name: config.attacker_name.clone(),
        attacker_score: 0,
        attacker_ammo: config.attacker_ammo,
    };
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, scoreboard: Res<Scoreboard>) {
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "DEFENDER: ".to_string() + &scoreboard.defender_name + "\n",
                    style: TextStyle {
                        font: asset_server.load("fonts/lunchds.ttf"),
                        font_size: 25.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
                TextSection {
                    value: "  Score:  ".to_string() + &scoreboard.defender_score.to_string() + "\n",
                    style: TextStyle {
                        font: asset_server.load("fonts/lunchds.ttf"),
                        font_size: 25.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
                TextSection {
                    value: "  Health: ".to_string() + &scoreboard.defender_health.to_string() + "\n",
                    style: TextStyle {
                        font: asset_server.load("fonts/lunchds.ttf"),
                        font_size: 25.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },

                TextSection {
                    value: "ATTACKER: ".to_string() + &scoreboard.attacker_name + "\n",
                    style: TextStyle {
                        font: asset_server.load("fonts/lunchds.ttf"),
                        font_size: 25.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                },
                TextSection {
                    value: "  Score:  ".to_string() + &scoreboard.attacker_score.to_string() + "\n",
                    style: TextStyle {
                        font: asset_server.load("fonts/lunchds.ttf"),
                        font_size: 25.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                },
                TextSection {
                    value: "  Ammo:   ".to_string() + &scoreboard.attacker_ammo.to_string() + "\n",
                    style: TextStyle {
                        font: asset_server.load("fonts/lunchds.ttf"),
                        font_size: 25.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }).insert(TextType::Scoreboard);
}

pub fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<(&mut Text, &TextType)>) {
    for (mut text, text_type) in query.iter_mut() {
        if *text_type == TextType::Scoreboard {
            text.sections[1].value = "  Score:  ".to_string() + &scoreboard.defender_score.to_string() + "\n";
            text.sections[2].value = "  Health: ".to_string() + &scoreboard.defender_health.to_string() + "\n";

            text.sections[4].value = "  Score:  ".to_string() + &scoreboard.attacker_score.to_string() + "\n";
            text.sections[5].value = "  Ammo:   ".to_string() + &scoreboard.attacker_ammo.to_string() + "\n";
        }
    }
}
