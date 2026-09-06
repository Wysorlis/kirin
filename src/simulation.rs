use macroquad::prelude::Vec2;
pub struct Simulation {
    nx: usize,
    ny: usize,
    time: f32,
    density: Vec<f32>,
}

impl Simulation {
    pub fn new(nx: usize, ny: usize) -> Self {
        assert!(nx > 0, "nx must be greater than zero");
        assert!(ny > 0, "ny must be greater than zero");

        let time: f32 = 0.0;
        let density: Vec<f32> = vec![0.0; nx * ny];
        Self {
            nx,
            ny,
            time,
            density,
        }
    }

    pub fn nx(&self) -> usize {
        self.nx
    }

    pub fn ny(&self) -> usize {
        self.ny
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn value_at(&self, i: usize, j: usize) -> f32 {
        self.density[i + j * self.nx]
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;

        // let should_switch_color: bool = (self.time - self.last_switch_time) > self.delay;

        // if should_switch_color {
        //     self.last_switch_time = self.time;
        //     self.inverted = !self.inverted;
        // }
    }

    pub fn add_density(&mut self, x_ratio: f32, y_ratio: f32) {
        // Ignorer les clics hors du domaine.
        if !(0.0..1.0).contains(&x_ratio) || !(0.0..1.0).contains(&y_ratio) {
            return;
        }

        let i: usize = (x_ratio * self.nx as f32) as usize;
        let j: usize = (y_ratio * self.ny as f32) as usize;

        self.density[i + j * self.nx] = 1.0;
    }
}
