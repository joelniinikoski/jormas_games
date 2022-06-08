use crate::*;

pub const SQUARE_SIZE: f32 = 20.0;

pub struct State {
    pub alive: Vec<Vec2>,
}

impl State {
    pub fn draw(&self) {
        for square in &self.alive {
            draw_rectangle(square.x, square.y, SQUARE_SIZE, SQUARE_SIZE, WHITE)
        }
    }
    pub fn propagate(&mut self) {
        let dirs: Vec<Vec2> = vec![
            vec2(SQUARE_SIZE, SQUARE_SIZE),
            vec2(SQUARE_SIZE, 0.0),
            vec2(SQUARE_SIZE,-SQUARE_SIZE),
            vec2(0.0,SQUARE_SIZE),
            vec2(0.0,-SQUARE_SIZE),
            vec2(-SQUARE_SIZE,SQUARE_SIZE),
            vec2(-SQUARE_SIZE,0.0),
            vec2(-SQUARE_SIZE,-SQUARE_SIZE)
        ];
        
        let mut next_alive = vec![];

        let mut squares_to_calculate = self.alive.clone();
        for square in &self.alive {
            for dir in &dirs {
                if !squares_to_calculate.contains(&(*square+*dir)){
                    squares_to_calculate.push(*square+*dir);
                }
            }
        }

        for square in squares_to_calculate {
            let mut neighbors = 0;
            for dir in &dirs {
                if self.alive.contains(&(square+*dir)) {
                    neighbors += 1;
                }
            }

            if neighbors == 3 || neighbors == 2 && self.alive.contains(&square) {
                next_alive.push(square);
            }
        }

        self.alive = next_alive;
    }
}