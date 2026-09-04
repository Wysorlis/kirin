mod application;
mod renderer;
mod simulation;

use application::Application;
use macroquad::prelude::next_frame;

#[macroquad::main("KIRIN")]
async fn main() {
    let mut app: Application = Application::new();

    loop {
        app.update();
        app.draw();

        next_frame().await;
    }
}
