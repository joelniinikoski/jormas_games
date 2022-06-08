use macroquad_jorma::*;
use game_of_life_mod::*;
use std::io::stdin;
use ::rand::Rng;

const WAIT_BETWEEN_FRAMES: f32 = 0.05;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Conway's Game Of Life"),
        window_width: 1260,
        window_height: 768,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {

    //create random pattern of blocks of size x * x and with probability of a block spawning y:
    let mut area = String::new();
    let mut prob = String::new();

    println!("Input integer value between 1 and 19 for block size:");
    stdin().read_line(&mut area).unwrap();
    let x = area.trim().parse::<usize>().expect("Please input a valid integer value");

    println!("Input positive floating value for probability of squares:");
    stdin().read_line(&mut prob).unwrap();
    let y = prob.trim().parse::<f32>().expect("Please input a valid floating point probability");

    let mut vec = vec![];
    if x > 0 && x < 20 && y > 0. && y <= 1. {
        for i in 0..x {
            for j in 0..x {
                let mut rng = ::rand::thread_rng();
                if rng.gen_range(0_f32..1_f32) < y {
                    vec.push(vec2(screen_width()/2.-(x as f32)*SQUARE_SIZE/2. + (i as f32) * SQUARE_SIZE,screen_height()/2.-(y as f32)*SQUARE_SIZE/2. + (j as f32) * SQUARE_SIZE));
                }
            }
        }
    } else {panic!()}


    //instantiate state with user parameters
    let mut state = State { alive: vec };
    let mut timer = 0.0;

    loop {
        let delta = get_frame_time();
        if timer > 0.0 {
            timer -= delta;
        }

        state.draw();

        if timer <= 0.0 {
            state.propagate();
            timer = WAIT_BETWEEN_FRAMES;
        }

        next_frame().await;
    }
}