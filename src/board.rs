use bevy::{
    prelude::*
};

use crate::{
    config::DefendersConfig,
    defender::Defender,
};

#[derive(Component)]
pub struct Board {
    board: Vec<Vec<char>>
}

pub fn setup(mut commands: Commands, config: Res<DefendersConfig>) {
    let x: &DefendersConfig = config.as_ref();
    commands
        .spawn()
        .insert(new_board(x.skyline.clone()));
}

pub fn new_board(skyline: Vec<usize>) -> Board {
    const WIDTH: usize = 110;
    const HEIGHT: usize = 30;

    let mut board = Vec::new();
    for i in 0..HEIGHT {
        let mut row = Vec::new();
        for j in 0..WIDTH {
            if i == 1 {
                row.push('-');
            } else if i == HEIGHT - 1 {
                row.push('_');
            } else if j == 0 {
                row.push('|');
            } else if j == WIDTH - 1 {
                row.push('|');
            } else {
                row.push(' ');
            }
        }
        board.push(row);
    }

    let l = skyline.len();
    let mut last_y = skyline[0] + 1;
    let mut champ_y = 0;

    for i in 2..l {
        let y = skyline[i] + 1;
        let x = i - 1;

        board[y][x] = '_';

        if y > last_y {
            for y_i in 2..y {
                board[y_i][x-1] = '|';
            }
        } else if y < last_y {
            for y_i in 2..last_y {
                board[y_i][x] = '|';
            }
        }

        if y > champ_y {
            champ_y = y;
        }

        last_y = y;
    }

    return Board{
        board: board
    }
}

pub fn board_system(query: Query<&Board>, defender_query: Query<&Defender>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let board = query.single();
    let defender = defender_query.single();

    for row in (1..board.board.len()).rev() {
        if row == 15 {
            for col in 0..board.board[row].len() {
                if col >= defender.x && col <= defender.x + 5 {
                    print!("#");
                } else {
                    print!("{}", board.board[row][col]);
                }
            }
        } else if row == 1 {
            print!("{}", defender.x);
        } else {
            for c in board.board[row].iter() {
                print!("{}", c);
            }
        }
        println!();
    }
}
