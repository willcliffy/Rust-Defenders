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

impl DefendersConfig {
    pub fn new(filename: String) -> DefendersConfig {
        let config_file = File::open(filename).unwrap();
        let config_rile_reader = BufReader::new(config_file);

        let mut skyline = Vec::new();
        let mut attacker_name = String::new();
        let mut attacker_num_missiles = -1;
        let mut defender_name = String::new();
    
        for line in config_rile_reader.lines() {
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
