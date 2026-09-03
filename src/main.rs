mod utils;
mod simulation;
use macroquad::prelude::*;

#[macroquad::main("KIRIN")]
async fn main() {

    let sim = simulation::Simulation::new(5, 5);
    sim.show();

    loop {
        clear_background(BLACK);

        sim.draw();
        
        next_frame().await;
    }

}
