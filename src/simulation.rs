use macroquad::models::Mesh;

use crate::matrix::Matrix;
use crate::mesh::SimulationMesh;
use std::env::JoinPathsError;
use std::process::ExitCode;

const PRESSURE_MAX_ITERATIONS: usize = 500;
const PRESSURE_TOLERANCE: f32 = 1.0e-5;

pub struct Simulation {
    nx: usize,
    ny: usize,
    mesh: SimulationMesh,

    time: f32,

    u: Matrix,
    v: Matrix,

    u_star: Matrix,
    v_star: Matrix,

    pressure: Matrix,

    dye: Matrix,

    density: f32,
}

impl Simulation {
    pub fn new() -> Self {
        let nx: usize = 10;
        let ny: usize = 10;
        let mesh: SimulationMesh = SimulationMesh::new(1.0, 1.0, nx, ny);

        let time: f32 = 0.0;

        let u: Matrix = Matrix::new(0.0, nx + 1, ny);
        let v: Matrix = Matrix::new(0.0, nx, ny + 1);
        let u_star: Matrix = Matrix::new(0.0, nx + 1, ny);
        let v_star: Matrix = Matrix::new(0.0, nx, ny + 1);
        let pressure: Matrix = Matrix::new(0.0, nx, ny);
        let dye: Matrix = Matrix::new(0.0, nx, ny);

        Self {
            nx,
            ny,
            mesh,
            time,
            u,
            v,
            u_star,
            v_star,
            pressure,
            dye,
            density: 100.0,
        }
    }

    pub fn nx(&self) -> usize {
        self.mesh.nx()
    }

    pub fn ny(&self) -> usize {
        self.mesh.ny()
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn value_at(&self, i: usize, j: usize) -> f32 {
        self.dye[(i, j)]
    }

    pub fn update(&mut self, dt: f32) {
        // 1. Imposer les conditions sur les vitesses actuelles.
        self.apply_velocity_boundary_conditions();

        // 2. Calculer les vitesses intermédiaires par convection.
        self.calculate_ustar(dt);
        self.calculate_vstar(dt);

        // 3. Trouver la pression qui permettra d'annuler la divergence.
        self.solve_pressure(dt);

        // // 4. Corriger u_star et v_star pour obtenir les nouveaux u et v.
        self.project_velocity(dt);

        // // 5. Garantir les conditions aux murs après la correction.
        self.apply_velocity_boundary_conditions();

        // // 6. Transporter le colorant avec les nouvelles vitesses.
        self.advect_dye(dt);

        self.time += dt;
    }

    fn calculate_ustar(&mut self, dt: f32) {
        // Toutes les faces u sauf celles sur les murs ouest et est
        for j in 0..self.ny() {
            for i in 1..self.nx() {
                // Convective term in u
                let u_e: f32 = (self.u[(i, j)] + self.u[(i + 1, j)]) / 2.0;
                let u_w: f32 = (self.u[(i, j)] + self.u[(i - 1, j)]) / 2.0;

                // Réflexion tangentielle aux murs nord et sud.
                let u_n: f32 = if j == self.ny - 1 {
                    self.u[(i, j)]
                } else {
                    (self.u[(i, j)] + self.u[(i, j + 1)]) / 2.0
                };

                let u_s: f32 = if j == 0 {
                    self.u[(i, j)]
                } else {
                    (self.u[(i, j)] + self.u[(i, j - 1)]) / 2.0
                };

                // Aucun flux normal à travers les murs nord et sud.
                let v_n: f32 = if j == self.ny - 1 {
                    0.0
                } else {
                    (self.v[(i, j + 1)] + self.v[(i - 1, j + 1)]) / 2.0
                };

                let v_s: f32 = if j == 0 {
                    0.0
                } else {
                    (self.v[(i, j)] + self.v[(i - 1, j)]) / 2.0
                };

                let convectif_term_u: f32 = (u_e * u_e - u_w * u_w) / self.mesh.dx()
                    + (u_n * v_n - u_s * v_s) / self.mesh.dy();

                self.u_star[(i, j)] = self.u[(i, j)] - dt * convectif_term_u;
            }
        }
    }

    fn calculate_vstar(&mut self, dt: f32) {
        // Toutes les faces v sauf celles sur les murs nord et sud.
        for j in 1..self.ny() {
            for i in 0..self.nx() {
                // Convective term in v
                let v_n: f32 = (self.v[(i, j)] + self.v[(i, j + 1)]) / 2.0;
                let v_s: f32 = (self.v[(i, j)] + self.v[(i, j - 1)]) / 2.0;

                // Réflexion tangentielle aux murs est et ouest.
                let v_e: f32 = if i == self.nx - 1 {
                    self.v[(i, j)]
                } else {
                    (self.v[(i, j)] + self.v[(i + 1, j)]) / 2.0
                };

                let v_w: f32 = if i == 0 {
                    self.v[(i, j)]
                } else {
                    (self.v[(i - 1, j)] + self.v[(i, j)]) / 2.0
                };

                // Aucun flux normal à travers les murs est et ouest.
                let u_e: f32 = if i == self.nx - 1 {
                    0.0
                } else {
                    (self.u[(i + 1, j - 1)] + self.u[(i + 1, j)]) / 2.0
                };

                let u_w: f32 = if i == 0 {
                    0.0
                } else {
                    (self.u[(i, j - 1)] + self.u[(i, j)]) / 2.0
                };

                let convectif_term_v: f32 = (u_e * v_e - u_w * v_w) / self.mesh.dx()
                    + (v_n * v_n - v_s * v_s) / self.mesh.dy();

                self.v_star[(i, j)] = self.v[(i, j)] - dt * convectif_term_v;
            }
        }
    }

    fn apply_velocity_boundary_conditions(&mut self) {
        // Vitesse normale nulle aux murs ouest et est.
        for j in 0..self.ny {
            self.u_star[(0, j)] = 0.0;
            self.u_star[(self.nx, j)] = 0.0;
        }

        // Vitesse normale nulle aux murs nord et sud.
        for i in 0..self.nx {
            self.v_star[(i, 0)] = 0.0;
            self.v_star[(i, self.ny)] = 0.0;
        }
    }

    fn solve_pressure(&mut self, dt: f32) {
        let b: Matrix = self.compute_pressure_rhs(dt);

        let mut p_old: Matrix = self.pressure.clone();
        let mut p_new: Matrix = self.pressure.clone();

        let inv_dx2: f32 = self.mesh.dx() * self.mesh.dx();
        let inv_dy2: f32 = self.mesh.dy() * self.mesh.dy();
        let inv2_dxdy: f32 = 2.0 / inv_dx2 + 2.0 / inv_dy2;

        for _ in 0..PRESSURE_MAX_ITERATIONS {
            for j in 0..self.ny() {
                for i in 0..self.nx() {
                    let p_c: f32 = p_old[(i, j)];

                    let p_e: f32 = if i + 1 < self.nx() {
                        p_old[(i + 1, j)]
                    } else {
                        p_c
                    };

                    let p_w: f32 = if i > 0 { p_old[(i - 1, j)] } else { p_c };

                    let p_n: f32 = if j + 1 < self.ny() {
                        p_old[(i, j + 1)]
                    } else {
                        p_c
                    };

                    let p_s: f32 = if j > 0 { p_old[(i, j - 1)] } else { p_c };

                    let x_pressure_terms: f32 = (p_e + p_w) / inv_dx2;
                    let y_pressure_terms: f32 = (p_n + p_s) / inv_dy2;

                    p_new[(i, j)] = (x_pressure_terms + y_pressure_terms - b[(i, j)]) / inv2_dxdy;
                }
            }

            // Le nouveau résultat devient l'ancien pour le prochain passage.
            std::mem::swap(&mut p_old, &mut p_new);

            // Vérifier si A * pressure_old est suffisamment proche de b.
            if self.residual_is_small(&p_old, &b, PRESSURE_TOLERANCE) {
                break;
            }
        }

        // The last pressure is kept
        self.pressure = p_old;
    }

    fn residual_is_small(&self, pressure: &Matrix, rhs: &Matrix, tolerance: f32) -> bool {
        let dx = self.mesh.dx();
        let dy = self.mesh.dy();

        let inv_dx2 = 1.0 / (dx * dx);
        let inv_dy2 = 1.0 / (dy * dy);

        let mut max_residual: f32 = 0.0;

        for j in 0..self.ny {
            for i in 0..self.nx {
                let p_c = pressure[(i, j)];

                let p_e = if i + 1 < self.nx {
                    pressure[(i + 1, j)]
                } else {
                    p_c
                };

                let p_w = if i > 0 { pressure[(i - 1, j)] } else { p_c };

                let p_n = if j + 1 < self.ny {
                    pressure[(i, j + 1)]
                } else {
                    p_c
                };

                let p_s = if j > 0 { pressure[(i, j - 1)] } else { p_c };

                let laplacian =
                    (p_e - 2.0 * p_c + p_w) * inv_dx2 + (p_n - 2.0 * p_c + p_s) * inv_dy2;

                let residual = (laplacian - rhs[(i, j)]).abs();

                if !residual.is_finite() {
                    return false;
                }

                max_residual = max_residual.max(residual);
            }
        }

        max_residual <= tolerance
    }

    fn compute_pressure_rhs(&mut self, dt: f32) -> Matrix {
        let mut b = Matrix::new(0.0, self.nx, self.ny);

        let dx: f32 = self.mesh.dx();
        let dy: f32 = self.mesh.dy();

        // We compute the RHS before solving for the pressure because
        // this term doesn't change throught the Jacobi proccess
        for j in 0..self.ny() {
            for i in 0..self.nx() {
                let div_u_star_term: f32 = (self.u_star[(i + 1, j)] - self.u_star[(i, j)]) / dx;

                let div_v_star_term: f32 = (self.v_star[(i, j + 1)] - self.v_star[(i, j)]) / dy;

                let divergence: f32 = div_u_star_term + div_v_star_term;
                b[(i, j)] = (self.density / dt) * divergence;
            }
        }

        b
    }

    fn project_velocity(&mut self, dt: f32) {
        let dx: f32 = self.mesh.dx();
        let dy: f32 = self.mesh.dy();

        for j in 0..self.ny() {
            for i in 0..self.nx() {
                if i > 0 {
                    let x_pressure_gradient: f32 =
                        (self.pressure[(i, j)] - self.pressure[(i - 1, j)]) / dx;
                    self.u[(i, j)] =
                        self.u_star[(i, j)] - (dt / self.density) * x_pressure_gradient;
                }

                if j > 0 {
                    let y_pressure_gradient: f32 =
                        (self.pressure[(i, j)] - self.pressure[(i, j - 1)]) / dy;
                    self.v[(i, j)] =
                        self.v_star[(i, j)] - (dt / self.density) * y_pressure_gradient;
                }
            }
        }
    }

    fn advect_dye(&mut self, dt: f32) {
        let dx: f32 = self.mesh.dx();
        let dy: f32 = self.mesh.dy();

        let mut dye_new = Matrix::new(0.0, self.nx, self.ny);

        for j in 0..self.ny() {
            for i in 0..self.nx() {
                let u_e: f32 = self.u[(i + 1, j)];
                let u_w: f32 = self.u[(i, j)];
                let v_n: f32 = self.v[(i, j + 1)];
                let v_s: f32 = self.v[(i, j)];

                // flux = u * c
                let flux_e: f32 = if i == self.nx() - 1 {
                    0.0
                } else if u_e >= 0.0 {
                    u_e * self.dye[(i, j)]
                } else {
                    u_e * self.dye[(i + 1, j)]
                };

                let flux_w: f32 = if i == 0 {
                    0.0
                } else if u_w >= 0.0 {
                    u_w * self.dye[(i - 1, j)]
                } else {
                    u_w * self.dye[(i, j)]
                };

                let flux_n: f32 = if j == self.ny() - 1 {
                    0.0
                } else if v_n >= 0.0 {
                    v_n * self.dye[(i, j)]
                } else {
                    v_n * self.dye[(i, j + 1)]
                };

                let flux_s: f32 = if j == 0 {
                    0.0
                } else if v_s >= 0.0 {
                    v_s * self.dye[(i, j - 1)]
                } else {
                    v_s * self.dye[(i, j)]
                };

                dye_new[(i, j)] =
                    self.dye[(i, j)] - dt * ((flux_e - flux_w) / dx + (flux_n - flux_s) / dy);
            }
        }

        self.dye = dye_new;
    }

    pub fn add_dye(&mut self, amount: f32, x_ratio: f32, y_ratio: f32) {
        // Ignorer les clics hors du domaine.
        if !(0.0..1.0).contains(&x_ratio) || !(0.0..1.0).contains(&y_ratio) {
            return;
        }

        let i: usize = (x_ratio * self.nx as f32) as usize;
        let j: usize = (y_ratio * self.ny as f32) as usize;

        self.dye[(i, j)] += amount;
    }

    pub fn get_field_minmax(&self) -> (f32, f32) {
        let min: f32 = self.dye.vector().iter().copied().reduce(f32::min).unwrap();
        let max: f32 = self.dye.vector().iter().copied().reduce(f32::max).unwrap();

        (min, max)
    }

    pub fn add_mouse_velocity(
        &mut self,
        x_ratio: f32,
        y_ratio: f32,
        mouse_vx: f32,
        mouse_vy: f32,
        dt: f32,
    ) {
        if !(0.0..1.0).contains(&x_ratio) || !(0.0..1.0).contains(&y_ratio) {
            return;
        }

        let i = (x_ratio * self.nx as f32) as usize;
        let j = (y_ratio * self.ny as f32) as usize;

        let lx = self.mesh.dx() * self.nx as f32;
        let ly = self.mesh.dy() * self.ny as f32;

        // Intensité de l'entraînement, à ajuster.
        let strength = 5.0;

        // Conversion en unités physiques, puis incrément de vitesse.
        let du = strength * mouse_vx * lx * dt;
        let dv = strength * mouse_vy * ly * dt;

        // Épargner les faces normales situées sur les murs.
        if i > 0 {
            self.u[(i, j)] += du;
        }
        if i + 1 < self.nx {
            self.u[(i + 1, j)] += du;
        }

        if j > 0 {
            self.v[(i, j)] += dv;
        }
        if j + 1 < self.ny {
            self.v[(i, j + 1)] += dv;
        }
    }
}
