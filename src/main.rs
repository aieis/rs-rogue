use std::env;

mod data_structures;
mod utils;
mod term;
mod world;
mod draw;
mod game;
mod demo;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        game::game();
        return;
    }

    if args[1] == "0" {
        demo::triangle();
    }

    if args[1] == "1" {
        demo::static_rectangle();
    }

    if args[1] == "2" {
        demo::rotating_rectangle();
    }

    if args[1] == "3" {
        demo::draw_geometry();
    }
    
}
