use crate::renderer::Renderer;
use crate::simulation::Simulation;

use macroquad::prelude::*;

pub struct Application {
    simulation: Simulation,
    renderer: Renderer,
}

impl Application {
    pub fn new() -> Self {
        let simulation: Simulation = Simulation::new(8, 8);
        let renderer: Renderer = Renderer::new(simulation.nx(), simulation.ny());

        Self {
            simulation,
            renderer,
        }
    }

    pub fn update(&mut self) {
        let dt: f32 = get_frame_time();

        self.simulation.update(dt);
        self.renderer.update_from(&self.simulation);
    }

    pub fn draw(&self) {
        self.renderer.draw();
    }
}
