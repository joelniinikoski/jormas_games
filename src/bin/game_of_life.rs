use macroquad_jorma::*;
use game_of_life_mod::*;
use std::io::stdin;
use ::rand::Rng;

const WAIT_BETWEEN_FRAMES: f32 = 0.08;
const CAMERA_SPEED: f32 = 0.07;
const ZOOM: f32 = 0.05;

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

    println!("Input integer value between 1 and 99 for block size:");
    stdin().read_line(&mut area).unwrap();
    let x = area.trim().parse::<usize>().expect("Please input a valid integer value");

    println!("Input positive floating value for probability of squares:");
    stdin().read_line(&mut prob).unwrap();
    let y = prob.trim().parse::<f32>().expect("Please input a valid floating point probability");

    let mut vec = vec![];
    if x > 0 && x < 100 && y > 0. && y <= 1. {
        for i in 0..x {
            for j in 0..x {
                let mut rng = ::rand::thread_rng();
                if rng.gen_range(0_f32..1_f32) <= y {
                    vec.push(vec2(screen_width()/2. + (i as f32) * SQUARE_SIZE,screen_height()/2. + (j as f32) * SQUARE_SIZE));
                }
            }
        }
    } else {panic!()}

    //instantiate state with user parameters
    let mut state = State { alive: vec};
    let mut timer = 0.0;

    let mut cam = Camera2D::from_display_rect(Rect::new(0.0,0.0,screen_width(),screen_height()));
    let mut offset = cam.offset;

    loop {
        //Camera movement and zoom
        if macroquad::input::is_key_down(KeyCode::Up) {
            offset += CAMERA_SPEED * -Vec2::Y;  
        }
        if macroquad::input::is_key_down(KeyCode::Down) {
            offset += CAMERA_SPEED * Vec2::Y;  
        }
        if macroquad::input::is_key_down(KeyCode::Right) {
            offset += CAMERA_SPEED * -Vec2::X;  
        }
        if macroquad::input::is_key_down(KeyCode::Left) {
            offset += CAMERA_SPEED * Vec2::X;  
        }
        if macroquad::input::is_key_down(KeyCode::R) {
            cam.zoom += cam.zoom * ZOOM;
        } else if macroquad::input::is_key_down(KeyCode::F) {
            cam.zoom += cam.zoom * -ZOOM;
        }
        cam.target = vec2(screen_width()/2. - offset.x * screen_width(), screen_height()/2. + offset.y * screen_height());

        set_camera(&cam);

        let delta = get_frame_time();
        if timer > 0.0 {
            timer -= delta;
        }

        state.draw();

        if timer <= 0.0 {
            state.propagate();
            timer = WAIT_BETWEEN_FRAMES;
        }
        
        //Draw coordinate text
        set_default_camera();
        draw_text(&format!("x:{:.2}, y:{:.2}",&offset.x * -1., &offset.y * -1.), 20., 20., 20., GREEN);

        next_frame().await;
    }
}