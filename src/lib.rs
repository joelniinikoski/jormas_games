pub use macroquad::prelude::*;

pub fn draw_text_centered(text: &str, x: f32, y:f32, font: Option<Font>, font_size: f32, color: Color) {
    let dimensions = measure_text(text, font, font_size as u16, 1.0);

    draw_text(text, x-dimensions.width/2., y-dimensions.height/2., font_size, color);
}

pub mod pong_mod;

pub mod game_of_life_mod;