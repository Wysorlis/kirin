use crate::utils;
use macroquad::prelude::*;

pub struct Simulation {
    size_x: i32,
    size_y: i32,
}

impl Simulation {
    pub fn new(size_x: i32, size_y: i32) -> Self {
        Self {
            size_x,
            size_y,
        }
    }

    pub fn draw(&self) {
        let t: f32 = get_time() as f32;

        let r = (t.sin() + 1.0) / 2.0;
        let g = ((t + 2.0).sin() + 1.0) / 2.0;
        let b = ((t + 4.0).sin() + 1.0) / 2.0;

        let color = Color::new(r, g, b,  1.0);

        let initial_pos = vec2(200.0, 200.0);
        let final_pos = vec2(400.0, 400.0);

        let animation_duration: f32 = 2.0;
        let s: f32 = clamp(t / animation_duration, 0.0, 1.0);
        
        let curr_pos = initial_pos.lerp(final_pos, s);

        draw_circle(curr_pos.x, curr_pos.y, 50.0, color);
    }

    pub fn show(&self) {
        println!("{}", self.size_x);
        println!("{}", self.size_y);
    }
} 