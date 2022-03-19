use bevy::{
    prelude::*
};

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Component)]
pub struct DefendersConfig {
    pub skyline: Vec<usize>,

    pub attacker_num_missiles: i32,

    // attacker_name: String,
    // defender_name: String,
}

fn default_config() -> DefendersConfig {
    return DefendersConfig{
        skyline: vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 6, 6, 7, 7, 7, 5, 5, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 8, 8, 8, 6, 6, 6, 9, 9, 9, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 10, 10, 10, 4, 4, 4, 9, 9, 9, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 6, 6, 6, 9, 9, 9, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 6, 6, 6, 9, 9, 9, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        attacker_num_missiles: 50,
    }
}

impl DefendersConfig {
    pub fn new(filename: String) -> DefendersConfig {
        let config_file = File::open(filename);
        if config_file.is_err() {
            return default_config();
        }

        let config_reader = BufReader::new(config_file.unwrap());

        let mut skyline = Vec::new();
        let mut attacker_name = String::new();
        let mut attacker_num_missiles = -1;
        let mut defender_name = String::new();
    
        for line in config_reader.lines() {
            let line = line.unwrap();

            if line.len() == 0 || line.starts_with("#") {
                continue;
            }

            if defender_name == "" {
                defender_name = line;
            } else if attacker_name == "" {
                attacker_name = line;
            } else if attacker_num_missiles == -1 {
                attacker_num_missiles = line.parse::<i32>().unwrap();
            } else {
                line.split(' ').for_each(|x| skyline.push(x.parse::<usize>().unwrap()));
            }
        }

        return DefendersConfig {
            skyline: skyline,

            attacker_num_missiles: attacker_num_missiles,

            // attacker_name: attacker_name,
            // defender_name: defender_name,
        }
    }
}
