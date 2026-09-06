pub struct Simulation {
    nx: usize,
    ny: usize,
    delay: f32,
    time: f32,
    last_switch_time: f32,
    inverted: bool,
}

impl Simulation {
    pub fn new(nx: usize, ny: usize) -> Self {
        assert!(nx > 0, "nx must be greater than zero");
        assert!(ny > 0, "ny must be greater than zero");

        let delay: f32 = 0.1;
        let time: f32 = 0.0;
        let last_switch_time: f32 = time;
        let inverted: bool = true;
        Self {
            nx,
            ny,
            delay,
            time,
            last_switch_time,
            inverted,
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
        // let is_even_cell: bool = (i + j).is_multiple_of(2);
        // let is_light: bool = is_even_cell ^ self.inverted;

        // if is_light { 1.0 } else { 0.0 }
        1.0
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;

        // let should_switch_color: bool = (self.time - self.last_switch_time) > self.delay;

        // if should_switch_color {
        //     self.last_switch_time = self.time;
        //     self.inverted = !self.inverted;
        // }
    }

    pub fn add_density(&self, x_pos: f32, y_pos: f32) {}
}
