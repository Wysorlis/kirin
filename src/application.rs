use crate::renderer::Renderer;
use crate::simulation::Simulation;

use macroquad::prelude::*;

pub struct Application {
    simulation: Simulation,
    renderer: Renderer,
    paused: bool,
    previous_drag_position: Option<Vec2>,
}

impl Application {
    pub fn new() -> Self {
        let simulation: Simulation = Simulation::new();
        let renderer: Renderer = Renderer::new(simulation.nx(), simulation.ny());

        Self {
            simulation,
            renderer,
            paused: false,
            previous_drag_position: None,
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
            let position = self.renderer.screen_to_simulation_ratio(mouse_position());

            self.simulation.add_dye(1.0, position.x, position.y);
        }

        self.handle_velocity_drag();
    }

    fn handle_velocity_drag(&mut self) {
        if !is_mouse_button_down(MouseButton::Right) {
            self.previous_drag_position = None;
            return;
        }

        let position = self.renderer.screen_to_simulation_ratio(mouse_position());

        if !(0.0..1.0).contains(&position.x) || !(0.0..1.0).contains(&position.y) {
            self.previous_drag_position = None;
            return;
        }

        let dt = get_frame_time();

        if let Some(previous) = self.previous_drag_position {
            if dt > 0.0 {
                let velocity = (position - previous) / dt;

                self.simulation
                    .add_mouse_velocity(position.x, position.y, velocity.x, velocity.y, dt);
            }
        }

        self.previous_drag_position = Some(position);
    }
}
//
