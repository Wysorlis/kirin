use crate::simulation::Simulation;
use macroquad::prelude::*;

const WORLD_WIDTH: f32 = 800.0;
const WORLD_HEIGHT: f32 = 800.0;

pub struct Renderer {
    texture: Texture2D,
    image: Image,
    camera: Camera2D,
    render_time: f32,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        let width: u16 = width as u16;
        let height: u16 = height as u16;

        let image: Image = Image::gen_image_color(width, height, BLACK);
        let texture: Texture2D = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest);
        // texture.set_filter(FilterMode::Linear);
        let camera: Camera2D = Self::create_camera();

        let render_time: f32 = 0.0;

        Self {
            texture,
            image,
            camera,
            render_time,
        }
    }

    pub fn update_from(&mut self, simulation: &Simulation) {
        self.render_time = simulation.time();

        let (min_value, max_value) = simulation.get_field_minmax();

        for j in 0..simulation.ny() {
            for i in 0..simulation.nx() {
                let value: f32 = simulation.value_at(i, j);
                let color: Color = self.color_map(value, min_value, max_value);
                self.image.set_pixel(i as u32, j as u32, color);
            }
        }

        self.texture.update(&self.image);
    }

    pub fn draw(&mut self) {
        clear_background(BLACK);

        self.update_camera();

        set_camera(&self.camera);
        // camera.screen_to_world(point)

        draw_texture_ex(
            &self.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(WORLD_WIDTH, WORLD_HEIGHT)),
                ..Default::default()
            },
        );

        set_default_camera();

        draw_text(
            &format!("Time {:.2} s", &self.render_time),
            0.0,
            25.0,
            25.0,
            DARKGRAY,
        );
    }

    fn create_camera() -> Camera2D {
        Camera2D {
            target: vec2(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0),
            zoom: vec2(2.0 / WORLD_WIDTH, -2.0 / WORLD_HEIGHT),
            viewport: Some(Self::calculate_camera_viewport()),
            ..Default::default()
        }
    }

    fn update_camera(&mut self) {
        self.camera.viewport = Some(Self::calculate_camera_viewport());
    }

    fn calculate_camera_viewport() -> (i32, i32, i32, i32) {
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

        viewport
    }

    pub fn screen_to_world(&self, mouse_position: (f32, f32)) -> Vec2 {
        let (x_mouse, y_mouse) = mouse_position;

        let world_coordinates: Vec2 = self.camera.screen_to_world(vec2(x_mouse, y_mouse));

        world_coordinates
    }

    fn color_map(&self, value: f32, min_field_value: f32, max_field_value: f32) -> Color {
        let min_color: Color = BLUE;
        let max_color: Color = RED;

        if (max_field_value - min_field_value) > 0.0 {
            let ratio: f32 = value / (max_field_value - min_field_value);
            let clamped_ratio: f32 = ratio.clamp(0.0, 1.0);
    
            let interpolated_color: Color = self.lerp_between_colors(min_color, max_color, clamped_ratio);

            interpolated_color
        } else {
            MAGENTA
        }
    }

    fn lerp_between_colors(&self, min_color: Color, max_color: Color, t: f32) -> Color {
        let t: f32 = t.clamp(0.0, 1.0);
        let min_color_vector: glam::Vec4 = min_color.to_vec();
        let max_color_vector: glam::Vec4 = max_color.to_vec();

        let interpolated_color_vector: glam::Vec4 = glam::Vec4::lerp(min_color_vector, max_color_vector, t);

        Color::from_vec(interpolated_color_vector)
    }
}
