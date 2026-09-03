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
        let s: f32 = (t / animation_duration).clamp(0.0, 1.0);
        
        let curr_pos = initial_pos.lerp(final_pos, s);

        draw_circle(curr_pos.x, curr_pos.y, 50.0, color);
    }
    
    pub fn draw2(&self) {
        let square_size: f32 = 50.0;

        
        let square_number_x: i32 = 8;
        let square_number_y: i32 = 8;
        let total_square_number: i32 = square_number_x * square_number_y;

        let width = screen_width();
        let height = screen_height();

        let width_per_square: f32 = width / square_number_x as f32;
        let height_per_square: f32 = height / square_number_y as f32;



        // draw_rectangle(position.x, position.y, square_size, square_size, BLACK);
        let light_square: Color = Color::from_hex(0xEEEED2);
        let dark_square: Color = Color::from_hex(0x769656);

        for j in 0..square_number_y {
            for i in 0..square_number_x {
                let position: Vec2 = vec2(width_per_square * i as f32 , height_per_square * j as f32);
                let color: Color  = if (i + j) % 2 == 0 {light_square} else {dark_square};

                draw_rectangle(position.x, position.y, width_per_square, height_per_square, color);
            }
        }

    }

    pub fn show(&self) {
        println!("{}", self.size_x);
        println!("{}", self.size_y);
    }
} 