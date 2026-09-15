use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Matrix {
    vector: Vec<f32>,
    nx: usize,
    ny: usize,
}

///
/// avec nx = 5
///      ny = 4
///
///                       i → axe X
///
///         i=0      i=1      i=2      i=3      i=4
///      ┌────────┬────────┬────────┬────────┬────────┐
/// j=0  │ (0,0)  │ (1,0)  │ (2,0)  │ (3,0)  │ (4,0)  │
///      ├────────┼────────┼────────┼────────┼────────┤
/// j=1  │ (0,1)  │ (1,1)  │ (2,1)  │ (3,1)  │ (4,1)  │
///      ├────────┼────────┼────────┼────────┼────────┤
/// j=2  │ (0,2)  │ (1,2)  │ (2,2)  │ (3,2)  │ (4,2)  │
///      ├────────┼────────┼────────┼────────┼────────┤
/// j=3  │ (0,3)  │ (1,3)  │ (2,3)  │ (3,3)  │ (4,3)  │
///      └────────┴────────┴────────┴────────┴────────┘
/// ↓
/// j, axe Y
///
impl Matrix {
    pub fn new(initial_value: f32, nx: usize, ny: usize) -> Self {
        let vector = vec![initial_value; nx * ny];

        Self { nx, ny, vector }
    }

    fn flat_index(&self, i: usize, j: usize) -> usize {
        assert!(i < self.nx, "Indice i={i} hors de la matrice");
        assert!(j < self.ny, "Indice j={j} hors de la matrice");

        i + j * self.nx
    }

    pub fn vector(&self) -> Vec<f32> {
        self.vector.clone()
    }

    pub fn nx(&self) -> usize {
        self.nx
    }

    pub fn ny(&self) -> usize {
        self.ny
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f32;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        let index: usize = self.flat_index(i, j);
        &self.vector[index]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        let index: usize = self.flat_index(i, j);
        &mut self.vector[index]
    }
}
