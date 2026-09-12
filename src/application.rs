use crate::renderer::Renderer;
use crate::simulation::Simulation;

use macroquad::prelude::*;

pub struct Application {
    simulation: Simulation,
    renderer: Renderer,
    paused: bool,
}

impl Application {
    pub fn new() -> Self {
        let simulation: Simulation = Simulation::new(3, 3);
        let renderer: Renderer = Renderer::new(simulation.nx(), simulation.ny());

        Self {
            simulation,
            renderer,
            paused: false,
        }
    }

    pub fn update(&mut self) {
        if !self.paused {
            let dt: f32 = get_frame_time();

            self.simulation.update(dt);
        }

        // Met à jour le rendu même lorsque la simulation est en pause.
        self.renderer.update_from(&self.simulation);
    }

    pub fn draw(&mut self) {
        self.renderer.draw();
    }

    pub fn handle_inputs(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.paused = !self.paused;
        }

        if is_mouse_button_down(MouseButton::Left) {
            let click_world_coordinates: Vec2 = self.renderer.screen_to_world(mouse_position());

            self.simulation.add_density(
                1.0,
                click_world_coordinates.x / 800.0,
                click_world_coordinates.y / 800.0,
            );
        }
    }
}
//
