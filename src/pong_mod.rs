use crate::*;
use ::rand::prelude::SliceRandom;

const PADDLE_SIZE: (f32, f32) = (20.0,120.0);
const PADDLE_SPEED: f32 = 10.;

const BALL_SIZE: f32 = 10.0;
const BALL_SPEED: f32 = 7.;

#[derive(Clone)]
pub struct State {
    paddles: (Paddle, Paddle),
    ball: Ball,
}
#[derive(Clone)]
struct Paddle {
    pos: Vec2,
    side: u8,
    size: Vec2,
}

impl Paddle {
    fn new_paddles() -> (Paddle, Paddle) {
        (
            Paddle {
                pos: Vec2::from((20.0, screen_height()/2.)),
                side: 1,
                size: Vec2::from(PADDLE_SIZE),

            },
            Paddle {
                pos: Vec2::from((screen_width()-20.0, screen_height()/2.)),
                side: 2,
                size: Vec2::from(PADDLE_SIZE),
            }
        )
    }
    fn move_y(&mut self) {
        match self.side {
            1 => {
                if is_key_down(KeyCode::W) && self.pos.y > self.size.y/2. {
                    self.pos.y -= PADDLE_SPEED;
                } 
                if is_key_down(KeyCode::S) && self.pos.y < screen_height()-self.size.y/2.{
                    self.pos.y += PADDLE_SPEED;
                }
            },
            2 => {
                if is_key_down(KeyCode::Up) && self.pos.y > self.size.y/2.{
                    self.pos.y -= PADDLE_SPEED;
                }
                if is_key_down(KeyCode::Down) && self.pos.y < screen_height()-self.size.y/2.{
                    self.pos.y += PADDLE_SPEED;
                }
            },
            _ => (),
        }
    }
    fn move_x(&mut self) {
        match self.side {
            1 => {
                if is_key_down(KeyCode::A) && self.pos.x > self.size.x/2. {
                    self.pos.x -= PADDLE_SPEED;
                } 
                if is_key_down(KeyCode::D) && self.pos.x < screen_width()-self.size.x/2.{
                    self.pos.x += PADDLE_SPEED;
                }
            },
            2 => {
                if is_key_down(KeyCode::Left) && self.pos.x > self.size.x/2.{
                    self.pos.x -= PADDLE_SPEED;
                }
                if is_key_down(KeyCode::Right) && self.pos.x < screen_width()-self.size.x/2.{
                    self.pos.x += PADDLE_SPEED;
                }
            },
            _ => (),
        }
    }
}
#[derive(Clone)]
struct Ball {
    pos: Vec2,
    size: f32,
    ismoving: bool,
    direction: Vec2,
    score: (usize,usize),
}

impl Ball {
    fn play(&mut self) {
        match self.ismoving {
            true => {
                if self.pos.y > screen_height() || self.pos.y < 0. {
                    self.direction.y = -self.direction.y;
                }
                if self.pos.x > screen_width()  {
                    self.ismoving=false;
                    self.score.0 += 1;
                } else if self.pos.x < 0.{
                    self.ismoving=false;
                    self.score.1 +=1;
                }
                self.pos += self.direction * BALL_SPEED;
            },
            false => {
                let mut rng = ::rand::thread_rng();
                self.pos = (screen_width()/2., screen_height()/2.).into();
                self.ismoving = true; 
                self.direction = *[vec2(1.0,1.0), vec2(-1.0,1.0), vec2(1.0,-1.0),  vec2(-1.0,-1.0)].choose(&mut rng).unwrap();
            }
        }
    }
}

fn paddle_interaction_with_ball(state: &mut State) {
    let ball = &mut state.ball;
    let p1pos: &Vec2 = &(state.paddles.0.pos.x+BALL_SIZE-PADDLE_SIZE.0/2.,state.paddles.0.pos.y-PADDLE_SIZE.0/2.).into();
    let p2pos: &Vec2 = &(state.paddles.1.pos.x+BALL_SIZE-PADDLE_SIZE.0/2.,state.paddles.1.pos.y-PADDLE_SIZE.0/2.).into();

    if ball.direction.x == -1.0 && (ball.pos.x-p1pos.x).abs() < PADDLE_SIZE.0/2. && (ball.pos.y-p1pos.y).abs() < PADDLE_SIZE.1/2. {
        ball.direction.x = -ball.direction.x;

        ball.direction.y = match ball.pos.y-p1pos.y > 0.0 {
            true => 1.0,
            false => -1.0,
        };
    }
    else if ball.direction.x == 1.0 && (ball.pos.x-p2pos.x).abs() < PADDLE_SIZE.0/2. && (ball.pos.y-p2pos.y).abs() < PADDLE_SIZE.1/2. {
        
        ball.direction.x = -ball.direction.x;

        match ball.pos.y-p2pos.y > 0.0 {
            true => 1.0,
            false => -1.0,
        };
    }
}

pub fn setup() -> State {
    let (player1, player2) = Paddle::new_paddles();
    let ball = Ball {pos: (screen_width()/2.,screen_height()/2.).into(), size: BALL_SIZE, ismoving: false, direction: (0.,0.).into(), score: (0,0)};
    State {paddles: (player1, player2), ball: ball}
}

pub fn run(state: &mut State) {
    let _delta = get_frame_time();
    {
        if is_key_pressed(KeyCode::R) {
            state.ball.score = (0,0);
        }
    }
    {
        let (player1, player2) = &mut state.paddles;

        player1.move_y();
        player2.move_y();
        player1.move_x();
        player2.move_x();

        draw_rectangle(player1.pos.x-player1.size.x/2., player1.pos.y-player1.size.y/2., player1.size.x, player1.size.y, GREEN);
        draw_rectangle(player2.pos.x-player2.size.x/2., player2.pos.y-player2.size.y/2., player2.size.x, player2.size.y, RED);
    }

    {
        let text = format!("{}       {}", state.ball.score.0, state.ball.score.1);
        draw_text_centered(&text, screen_width()/2., 50., Option::None, 40., WHITE);
    }
    
    {
        let ball = &mut state.ball;
        ball.play();
        draw_circle(ball.pos.x, ball.pos.y, ball.size, WHITE);
    }

    paddle_interaction_with_ball(state);
    
}