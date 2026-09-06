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
        let simulation: Simulation = Simulation::new(8, 8);
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

    pub fn draw(&self) {
        self.renderer.draw();
    }

    pub fn handle_inputs(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.paused = !self.paused;
        }

        if is_mouse_button_down(MouseButton::Right) {
            let (x, y) = mouse_position();
            // self.renderer.camera
            self.simulation.add_density(x, y);
        }
    }
}
