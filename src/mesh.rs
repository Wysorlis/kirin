pub struct SimulationMesh {
    dx: f32,
    dy: f32,
    nx: usize,
    ny: usize,
}

impl SimulationMesh {
    pub fn new(lx: f32, ly: f32, nx: usize, ny: usize) -> Self {
        assert!(nx > 0, "nx must be greater than zero");
        assert!(ny > 0, "ny must be greater than zero");

        let dx: f32 = lx / nx as f32;
        let dy: f32 = ly / ny as f32;

        Self { nx, ny, dx, dy }
    }

    pub fn nx(&self) -> usize {
        self.nx
    }

    pub fn ny(&self) -> usize {
        self.ny
    }

    pub fn dx(&self) -> f32 {
        self.dx
    }

    pub fn dy(&self) -> f32 {
        self.dy
    }

    pub fn at(&self, i: usize, j: usize) -> usize {
        i + j * self.nx
    }

    pub fn is_inside_bounds(&self, i: usize, j: usize) -> bool {
        i > 0 && j > 0 && i < self.ny && j < self.nx
    }
}
