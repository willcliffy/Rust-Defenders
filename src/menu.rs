use bevy::{
    prelude::*,
};

use crate::{
    scoreboard::*,
    config::*,
};

#[derive(Component)]
pub struct Menu {
    pub showing: bool,
}

pub fn new_menu() -> Menu {
    return Menu {
        showing: true,
    }
}

pub fn menu_system(
    commands: Commands,
    asset_server: Res<AssetServer>,
    scoreboard: Res<Scoreboard>,
    mut menu: ResMut<Menu>,
) {
    if menu.showing {
        return
    }
    
    if !menu.showing {
        if scoreboard.defender_health == 0 {
            menu.showing = true;
            let text = "Attacker won!\nPress space to restart.".to_string();
            let color = Color::rgb(1.0, 0.5, 0.5);
            show_endgame_screen_win(commands, menu, asset_server, text, color);
        } else if scoreboard.attacker_ammo == 0 {
            let text = "Defender won!\nPress space to restart.".to_string();
            let color = Color::rgb(0.5, 0.5, 1.0);
            show_endgame_screen_win(commands, menu, asset_server, text, color);
        }
    }
}

fn show_endgame_screen_win(
    mut commands: Commands,
    mut menu: ResMut<Menu>,
    asset_server: Res<AssetServer>,
    text: String,
    color: Color,
) {
    menu.showing = true;

    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: text,
                    style: TextStyle {
                        font: asset_server.load("fonts/lunchds.ttf"),
                        font_size: 40.0,
                        color: color,
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(600.0 / 2.0 - 100.0),
                left: Val::Px(900.0 / 2.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }).insert(TextType::Menu);
}
