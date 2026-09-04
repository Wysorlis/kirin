use crate::simulation::Simulation;
use macroquad::prelude::*;

const WORLD_WIDTH: f32 = 800.0;
const WORLD_HEIGHT: f32 = 800.0;

const LIGHT_SQUARE_COLOR: Color = Color::from_hex(0xEEEED2);
const DARK_SQUARE_COLOR: Color = Color::from_hex(0x769656);

pub struct Renderer {
    texture: Texture2D,
    image: Image,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        let width = u16::try_from(width).expect("texture width must fit in a u16");
        let height = u16::try_from(height).expect("texture height must fit in a u16");
        let image = Image::gen_image_color(width, height, BLACK);

        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest);

        Self { texture, image }
    }

    pub fn update_from(&mut self, simulation: &Simulation) {
        for j in 0..simulation.ny() {
            for i in 0..simulation.nx() {
                let value: f32 = simulation.value_at(i, j);
                let color: Color = if value > 0.5 {
                    LIGHT_SQUARE_COLOR
                } else {
                    DARK_SQUARE_COLOR
                };
                self.image.set_pixel(i as u32, j as u32, color);
            }
        }

        self.texture.update(&self.image);
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        let camera: Camera2D = self.create_camera();

        set_camera(&camera);

        draw_texture_ex(
            &self.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(800.0, 800.0)),
                ..Default::default()
            },
        );
    }

    fn create_camera(&self) -> Camera2D {
        let screen_ratio: f32 = screen_width() / screen_height();
        let world_ratio: f32 = WORLD_WIDTH / WORLD_HEIGHT;

        let viewport: (i32, i32, i32, i32) = if screen_ratio > world_ratio {
            // fenêtre trop large
            let height: f32 = screen_height();
            let width: f32 = height * world_ratio;

            // (((screen_width() - width) / 2.0) as i32, 0, width as i32, height as i32)
            (0, 0, width as i32, height as i32)
        } else {
            // fenêtre trop haute
            let width: f32 = screen_width();
            let height: f32 = width / world_ratio;

            // (0, ((screen_height() - height) / 2.0) as i32, width as i32, height as i32)
            (0, 0, width as i32, height as i32)
        };

        Camera2D {
            target: vec2(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0),
            zoom: vec2(2.0 / WORLD_WIDTH, -2.0 / WORLD_HEIGHT),
            viewport: Some(viewport),
            ..Default::default()
        }
    }
}
