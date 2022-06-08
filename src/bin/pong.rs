use macroquad_jorma::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Pong"),
        window_width: 1260,
        window_height: 768,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut state = pong_mod::setup();
    loop {
        pong_mod::run(&mut state);
        next_frame().await
    }
}